use super::{
    MAX_PACKAGE_BYTES, UPGRADE_LOCK, download_package, install_staged, managed_core_path, package_url,
    read_core_version, resolve_latest_version, stage_core,
};
use crate::{
    config::{Config, IVerge},
    core::{SilentUpdater, handle::Handle, notify::NoticeStatus},
    utils::dirs,
};
use anyhow::{Context as _, Result, ensure};
use clash_verge_logging::{Type, logging};
use sha2::{Digest as _, Sha256};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

fn pending(core: &str) -> Result<PathBuf> {
    Ok(dirs::app_home_dir()?.join(format!("{core}.pending-package")))
}

fn verify(package: &[u8], release: &serde_json::Value, alpha: bool, version: &str) -> Result<()> {
    let url = package_url(alpha, version)?;
    let expected = release["assets"]
        .as_array()
        .context("missing release assets")?
        .iter()
        .find(|asset| {
            asset["browser_download_url"]
                .as_str()
                .is_some_and(|asset_url| asset_url.eq_ignore_ascii_case(&url))
        })
        .and_then(|asset| asset["digest"].as_str())
        .context("missing release asset digest")?;
    let hash = Sha256::digest(package);
    let digest: String = hash.iter().map(|byte| format!("{byte:02x}")).collect();
    ensure!(
        expected.strip_prefix("sha256:") == Some(digest.as_str()),
        "core package digest mismatch"
    );
    Ok(())
}

fn read_package(path: &Path) -> Result<(String, Vec<u8>)> {
    use std::io::Read as _;
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(MAX_PACKAGE_BYTES as u64 + 66)
        .read_to_end(&mut bytes)?;
    let end = bytes
        .iter()
        .take(65)
        .position(|&b| b == b'\n')
        .context("missing cached core version")?;
    let package = bytes.split_off(end + 1);
    ensure!(package.len() <= MAX_PACKAGE_BYTES, "core package too large");
    let version = std::str::from_utf8(&bytes[..end])?;
    release_number(version)?;
    Ok((version.to_owned(), package))
}

fn remove(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        result => result.with_context(|| format!("removing {}", path.display())),
    }
}

pub(crate) fn discard_pending_logged() {
    for core in IVerge::VALID_CLASH_CORES {
        if let Err(e) = pending(core).and_then(|path| remove(&path)) {
            logging!(warn, Type::Core, "Discarding pending core: {e:#}");
        }
    }
}

// The fork publishes vMAJOR.MINOR.PATCH.REVISION and a matching -beta asset.
// An upstream three-component installed version has no fork revision yet.
fn release_number(version: &str) -> Result<[u64; 4]> {
    let number = version.strip_suffix("-beta").unwrap_or(version);
    let mut result = [0; 4];
    let components: Vec<_> = number
        .strip_prefix('v')
        .context("invalid core release")?
        .split('.')
        .collect();
    ensure!((3..=4).contains(&components.len()), "invalid core version {version}");
    for (field, value) in result.iter_mut().zip(components) {
        *field = value.parse().context("invalid numeric core version")?;
    }
    Ok(result)
}

fn is_staging_file(name: &str, core_file: &str) -> bool {
    let Some(id) = name
        .strip_prefix(&format!(".{core_file}."))
        .and_then(|s| s.strip_suffix(".tmp"))
    else {
        return false;
    };
    id.split_once('.')
        .is_some_and(|(pid, generation)| pid.parse::<u32>().is_ok() && generation.parse::<u64>().is_ok())
}

pub(crate) async fn cleanup() -> Result<()> {
    let _lock = UPGRADE_LOCK.lock().await;
    let config = Config::verge().await.latest_arc();
    let selected = config.get_valid_clash_core();
    for &core in IVerge::VALID_CLASH_CORES {
        let target = managed_core_path(core)?;
        remove(&pending(core)?.with_extension("downloading"))?;
        if config.auto_check_core_update != Some(true) || core != selected {
            remove(&pending(core)?)?;
        }
        let Some(directory) = target.parent() else { continue };
        let core_file = target
            .file_name()
            .and_then(|s| s.to_str())
            .context("invalid core name")?;
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if is_staging_file(&entry.file_name().to_string_lossy(), core_file) {
                remove(&entry.path())?;
            }
        }
        let old = target.with_extension("old");
        let rollback = target.with_file_name(format!(".{core}.rollback"));
        if (old.exists() || rollback.exists()) && read_core_version(&target).is_ok() {
            remove(&old)?;
            remove(&rollback)?;
        }
    }
    Ok(())
}

pub(crate) async fn prepare() -> Result<()> {
    let config = Config::verge().await.latest_arc();
    if config.auto_check_core_update != Some(true) {
        return Ok(());
    }
    let core = config.get_valid_clash_core();
    let target = managed_core_path(&core)?;
    let cached = pending(&core)?;
    let alpha = core.ends_with("-alpha");
    let (proxy, latest, release) = resolve_latest_version(alpha).await?;
    if release_number(&latest)? <= release_number(&read_core_version(&target)?)? {
        return remove(&cached);
    }
    if cached.exists()
        && read_package(&cached)
            .is_ok_and(|(version, package)| version == latest && verify(&package, &release, alpha, &latest).is_ok())
    {
        return Ok(());
    }
    let package = download_package(proxy, alpha, &latest).await?;
    verify(&package, &release, alpha, &latest)?;
    let _config_write = Config::lock_config_write().await;
    let config = Config::verge().await.latest_arc();
    if config.auto_check_core_update == Some(true)
        && config.get_valid_clash_core() == core
        && release_number(&latest)? > release_number(&read_core_version(&target)?)?
    {
        let writing = cached.with_extension("downloading");
        {
            use std::io::Write as _;
            let mut file = fs::File::create(&writing)?;
            writeln!(file, "{latest}")?;
            file.write_all(&package)?;
            file.sync_all()?;
        }
        fs::rename(writing, &cached).context("saving verified core update")?;
    }
    Ok(())
}

pub(crate) async fn check_on_startup(app: &tauri::AppHandle) {
    if let Err(error) = on_startup(app).await {
        logging!(warn, Type::Core, "Mihomo startup update: {error:#}");
    }
}

async fn on_startup(app: &tauri::AppHandle) -> Result<()> {
    let config = Config::verge().await.latest_arc();
    if config.auto_check_core_update != Some(true) {
        return Ok(());
    }
    let core = config.get_valid_clash_core();
    let target = managed_core_path(&core)?;
    let cached = pending(&core)?;
    if !cached.try_exists()? {
        return Ok(());
    }
    let (cached_version, package) = match read_package(&cached) {
        Ok(cache) => cache,
        Err(error) => {
            remove(&cached)?;
            return Err(error);
        }
    };
    if release_number(&cached_version)? <= release_number(&read_core_version(&target)?)? {
        return remove(&cached);
    }
    if config.auto_check_update.unwrap_or(true) && SilentUpdater::check(app).await?.is_some() {
        return remove(&cached);
    }
    let alpha = core.ends_with("-alpha");
    let (_, latest, release) = resolve_latest_version(alpha).await?;
    if cached_version != latest || verify(&package, &release, alpha, &latest).is_err() {
        // The offered release changed, or the cached archive is incomplete.
        return remove(&cached);
    }
    let title = clash_verge_i18n::t!("notifications.updateReady.coreTitle");
    if !SilentUpdater::ask_user_to_install(app, latest.trim_start_matches('v'), &title).await {
        return Ok(());
    }
    let _lock = UPGRADE_LOCK.lock().await;
    let config = Config::verge().await.latest_arc();
    if config.auto_check_core_update != Some(true) || config.get_valid_clash_core() != core {
        return Ok(());
    }
    let installed = read_core_version(&target)?;
    if release_number(&latest)? <= release_number(&installed)? {
        return remove(&cached);
    }
    let result = async {
        let staged = stage_core(&target, &package, &latest)?;
        install_staged(&core, &target, &installed, staged).await
    }
    .await;
    if let Err(error) = result {
        Handle::notice(NoticeStatus::Info, format!("Mihomo {latest}: {error:#}"));
        return Err(error);
    }
    remove(&cached)
}

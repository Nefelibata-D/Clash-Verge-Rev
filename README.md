## Custom fork integration

The development baseline is the official stable `main` branch, currently
`v2.5.10` at `c83f7ad529c7c9cb3d2d54a0955ef8c1b22a411b`. The local checkout is
single-branch and shallow, tracking `upstream/main`, not `dev`. Source version
files keep the official stable version unchanged, currently `2.5.10`.
Only GitHub build checkouts use `2.5.10-Plus`; do not commit the CI suffix.

Fetch official updates with `git fetch upstream main`. Commit your custom work
before merging the next stable release; if a merge base is missing, deepen the
history with `git fetch --deepen=50 upstream main` as needed. Do not reset or
replace the custom tree with the upstream tree. Moving to a new stable release
still requires resolving conflicts and validating the custom changes.

The custom `Release Plus` workflow (`release-plus.yml`) runs on pushes to `main` or manual dispatch
on `main`. It reads the official version from the checked-out `package.json`,
appends `-Plus`, and updates all four version files only in each build checkout.
Version discovery does not query upstream releases. Merge and verify the next
stable source before publishing its version. CI version changes are not pushed back.

Only Windows AMD64 NSIS and Linux AMD64 DEB/RPM installers are built. All three
packages and signatures must exist before publication. The release is marked
stable despite the SemVer `-Plus` suffix, and contains `latest.json` for the
client updater. Official workflow files and helpers are retained. Their entry
jobs require `github.repository == 'clash-verge-rev/clash-verge-rev'`; dependent
jobs follow the existing dependency chain. The generated AI review workflow is
changed through its Markdown source and recompiled. In this fork official jobs
are skipped, including code checks; only `release-plus.yml` runs, restricted to
`Nefelibata-D/Clash-Verge-Rev` on `main`. No manual GitHub disabling is needed.
GitHub may still display skipped workflow runs. When syncing upstream, retain
these entry conditions and add them to any new independent official workflows.
An already published version is skipped rather than replacing installed update
artifacts; a draft from the same commit can be retried. Updating the source's
`package.json` version produces a new `vX.Y.Z-Plus` release on the next run.

This checkout uses `Nefelibata-D/Mihomo` for bundled and in-app core updates,
and `Nefelibata-D/Clash-Verge-Rev` for application updates.

- Core releases follow the local Mihomo `build.yml` contract: the latest release
  tag is the stable version; diagnostic builds add `-beta` to that version in
  the same release. Assets are `pass-windows-amd64-<version>.zip` and
  `pass-linux-amd64-<version>.gz`. The existing Alpha core slot selects the
  diagnostic beta build. Other platforms need matching custom core artifacts
  and download mappings before they can be built or upgraded.
- Subscription YAML remains a generic mapping. `weighted-sticky` and
  `site-blacklist` reach the selected core, which performs the real `-t` check.
  The editor also accepts the additional strategy. Use a current custom core;
  old builds may reject the newer blacklist `rules` format.
- Application updates use `releases/latest/download/latest.json` from the custom
  repository. `scripts/updater-plus.mjs` assembles it from the signed build artifacts.
  The custom signing public key is configured in `src-tauri/tauri.conf.json`
  and each `src-tauri/webview2.*.json`. Keep the matching private key and password
  outside the repository, and configure the release repository secrets
  `TAURI_PRIVATE_KEY` and `TAURI_KEY_PASSWORD`; never commit them. This source
  change does not install a new client or replace a running core.

<h1 align="center">
  <img src="./src-tauri/icons/icon.png" alt="Clash" width="128" />
  <br>
  Continuation of <a href="https://github.com/zzzgydi/clash-verge">Clash Verge</a>
  <br>
</h1>

<h3 align="center">
A Clash Meta GUI based on <a href="https://github.com/tauri-apps/tauri">Tauri</a>.
</h3>

<p align="center">
  Languages:
  <a href="./docs/README_en.md">English</a> ·
  <a href="./docs/README_es.md">Español</a> ·
  <a href="./docs/README_fa.md">فارسی</a> ·
  <a href="./docs/README_ja.md">日本語</a> ·
  <a href="./docs/README_ko.md">한국어</a> ·
  <a href="./docs/README_pt.md">Português</a> ·
  <a href="./docs/README_ru.md">Русский</a> ·
  <a href="./README.md">简体中文</a>
</p>

## Preview

| Dark                             | Light                             |
| -------------------------------- | --------------------------------- |
| ![预览](./docs/preview_dark.png) | ![预览](./docs/preview_light.png) |

## Install

请到发布页面下载对应的安装包：[Release page](https://github.com/clash-verge-rev/clash-verge-rev/releases)<br>
Go to the [Release page](https://github.com/clash-verge-rev/clash-verge-rev/releases) to download the corresponding installation package<br>
Supports Windows (x64/x86), Linux (x64/arm64) and macOS 11+ (intel/apple).
支持 Windows (x64/x86)、Linux (x64/arm64) 和 macOS 11+ (intel/apple)。

#### 我应当怎样选择发行版

| 版本        | 特征                                     | 链接                                                                                   |
| :---------- | :--------------------------------------- | :------------------------------------------------------------------------------------- |
| Stable      | 正式版，高可靠性，适合日常使用。         | [Release](https://github.com/clash-verge-rev/clash-verge-rev/releases)                 |
| Alpha(废弃) | 测试发布流程。                           | [Alpha](https://github.com/clash-verge-rev/clash-verge-rev/releases/tag/alpha)         |
| AutoBuild   | 滚动更新版，适合测试反馈，可能存在缺陷。 | [AutoBuild](https://github.com/clash-verge-rev/clash-verge-rev/releases/tag/autobuild) |

#### 安装说明和常见问题，请到 [文档页](https://clash-verge-rev.github.io/) 查看

### TG 频道: [@clash_verge_rev](https://t.me/clash_verge_re)

---

## Promotion

### ✈️ [AI云边 -- 全新架构机场 ClaudeBorder](https://cruise.54678999.xyz/#/register?code=58q5UJZc)

🔥热销中使用本链接注册即送 **3 天免费试用**，每日 **1GB 流量**：👉 [点此注册](https://cruise.54678999.xyz/#/register?code=58q5UJZc)

#### AI云边 -- 全新架构机场。

- 💻 多次**技术迭代后**全新亮相。
- 🗺 全**高速稳定**正价节点。
- 🌏 **海外团队**，不跑路
- 🚀 线路**冗余**设计，**自动化运维**对抗各类封锁
- 👨‍🦲 团队架构师为**大厂**网络架构师
- 💰 极致**稳定**，亲民价**价格**
- 🌐 全面支持**流媒体及各AI访问**
- 🙋 7*12小时真人客服。解决您的各类问题。

🌐 官网：👉 [https://www.claudeborder.com](https://cruise.54678999.xyz/#/register?code=58q5UJZc)

### 🤖 [GPTKefu —— 与 Crisp 深度整合的 AI 智能客服平台](https://gptkefu.com)

- 🧠 深度理解完整对话上下文 + 图片识别，自动给出专业、精准的回复，告别机械式客服。
- ♾️ **不限回答数量**，无额度焦虑，区别于其他按条计费的 AI 客服产品。
- 💬 售前咨询、售后服务、复杂问题解答，全场景轻松覆盖，真实用户案例已验证效果。
- ⚡ 3 分钟极速接入，零门槛上手，即刻提升客服效率与客户满意度。
- 🎁 高级套餐免费试用 14 天，先体验后付费：👉 [立即试用](https://gptkefu.com)
- 📢 智能客服TG 频道：[@crisp_ai](https://t.me/crisp_ai)

---

## Features

- 基于性能强劲的 Rust 和 Tauri 2 框架
- 内置[Clash.Meta(mihomo)](https://github.com/MetaCubeX/mihomo)内核，并支持切换 `Alpha` 版本内核。
- 简洁美观的用户界面，支持自定义主题颜色、代理组/托盘图标以及 `CSS Injection`。
- 配置文件管理和增强（Merge 和 Script），配置文件语法提示。
- 系统代理和守卫、`TUN(虚拟网卡)` 模式。
- 可视化节点和规则编辑
- WebDav 配置备份和同步

### FAQ

Refer to [Doc FAQ Page](https://clash-verge-rev.github.io/faq/windows.html)

### Donation

[捐助Clash Verge Rev的开发](https://github.com/sponsors/clash-verge-rev)

## Development

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.

To run the development server, execute the following commands after all prerequisites for **Tauri** are installed:

```shell
pnpm i
pnpm run prebuild
pnpm dev
```

`pnpm dev` preserves the Development Channel's installed service state: an
existing service is used, while a previously uninstalled service remains
uninstalled and the app starts in Sidecar mode. Use `pnpm dev:service` to
explicitly install or update the isolated development service before launch,
or `pnpm dev:sidecar` to force the unprivileged Sidecar workflow.

## Contributions

Issue and PR welcome!

## Acknowledgement

Clash Verge rev was based on or inspired by these projects and so on:

- [zzzgydi/clash-verge](https://github.com/zzzgydi/clash-verge): A Clash GUI based on tauri. Supports Windows, macOS and Linux.
- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [Dreamacro/clash](https://github.com/Dreamacro/clash): A rule-based tunnel in Go.
- [MetaCubeX/mihomo](https://github.com/MetaCubeX/mihomo): A rule-based tunnel in Go.
- [Fndroid/clash_for_windows_pkg](https://github.com/Fndroid/clash_for_windows_pkg): A Windows/macOS GUI based on Clash.
- [vitejs/vite](https://github.com/vitejs/vite): Next generation frontend tooling. It's fast!

## Privacy

Clash Verge Rev 不收集任何用户数据，配置与日志仅保存在本地。详见[隐私政策](./PRIVACY.md)。

Clash Verge Rev does not collect any user data; configuration and logs stay on
your own device. See the [Privacy Policy](./PRIVACY.md) for details.

## License

GPL-3.0 License. See [License here](./LICENSE) for details.

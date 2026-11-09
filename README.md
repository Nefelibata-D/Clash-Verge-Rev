# Clash Verge Rev Plus

基于 [Clash Verge Rev](https://github.com/clash-verge-rev/clash-verge-rev) 官方稳定版的个人定制客户端，搭配 [定制 Mihomo](https://github.com/Nefelibata-D/Mihomo) 使用。

沿用上游的 Rust、Tauri、React 界面及配置管理能力，增加定制内核配置支持和独立更新来源。

## 下载与平台

安装包发布在[本仓库 Releases](https://github.com/Nefelibata-D/Clash-Verge-Rev/releases)。

| 平台 | 架构 | 安装包 |
| --- | --- | --- |
| Windows | x86-64 / AMD64 | NSIS `.exe` |
| Linux | x86-64 / AMD64 | `.deb` |

Debian/Ubuntu 系统可用 `sudo apt install ./安装包.deb` 安装；需要系统满足包依赖。这里只提供下载包，不提供 APT 软件源。

当前发布流程只构建上述平台，不构建 macOS、ARM、32 位 x86 或内置 WebView2 版本。

## 定制内容

- 使用 `Nefelibata-D/Mihomo` 提供内置内核与应用内内核更新。
- 保留订阅中的 `weighted-sticky`、`site-blacklist` 等定制字段，并为编辑器补充策略识别。
- 配置实际由选中的 Mihomo 内核执行 `-t` 校验；需要支持这些字段的定制内核。
- 客户端更新来自 `Nefelibata-D/Clash-Verge-Rev`，使用签名安装包和 `latest.json` 更新清单。
- 保留上游主题、配置增强、系统代理、TUN、节点与规则管理、WebDAV 备份等功能。

定制内核的普通版和诊断 beta 版位于同一个 Release。客户端原有的 Alpha 内核入口对应诊断 beta 版；其包名为 `mihomo-windows-amd64-<version>.zip` 或 `mihomo-linux-amd64-<version>.gz`。新版黑名单 `rules` 字段需要较新的定制内核。

## 本地开发

工具版本见 [`.tool-versions`](.tool-versions) 和 [`rust-toolchain.toml`](rust-toolchain.toml)，环境说明见 [CONTRIBUTING.md](CONTRIBUTING.md)。

```shell
pnpm install --frozen-lockfile
pnpm prebuild
pnpm dev
```

`pnpm prebuild` 从定制 Mihomo 仓库下载内核，需要存在对应 Release 资产。`pnpm dev` 还需要同级的 `clash-verge-service-ipc` 开发源码，具体要求见贡献指南。只调试前端可运行 `pnpm web:dev`。

`pnpm dev:service` 用于安装或更新隔离的开发服务；`pnpm dev:sidecar` 强制使用 Sidecar 模式。构建或类型检查成功不等于实际代理连接已验收。

## Git 与同步上游

本地使用浅克隆，两个远端分别用于：

| 远端 | 用途 |
| --- | --- |
| `origin` | 自己的 `Nefelibata-D/Clash-Verge-Rev` 仓库，接收提交 |
| `upstream` | 官方 `clash-verge-rev/clash-verge-rev` 仓库，获取稳定版更新 |

日常提交后正常推送即可：

```shell
git push origin main
```

本工作区已配置默认推送到 `origin`，因此 `git push` 也指向自己的仓库。`main` 的拉取跟踪仍为 `upstream/main`；`git pull` 获取的是官方更新，不是自己 GitHub 仓库上的改动。

同步官方更新前先提交本地工作，再执行：

```shell
git fetch upstream main
git merge upstream/main
```

若浅历史不足以找到共同祖先，先按需加深，再合并：

```shell
git fetch --deepen=50 upstream main
```

合并时处理冲突，保留定制修改、公钥与工作流仓库判断，并核对源码版本。不要使用整仓覆盖或强制重置替代合并。

## 上游与许可

感谢 [Clash Verge Rev](https://github.com/clash-verge-rev/clash-verge-rev)、[Clash Verge](https://github.com/zzzgydi/clash-verge)、[Mihomo](https://github.com/MetaCubeX/mihomo)、[Tauri](https://github.com/tauri-apps/tauri) 和 [Vite](https://github.com/vitejs/vite) 等上游项目。

本项目遵循 [GPL-3.0 许可证](LICENSE)。隐私说明见 [PRIVACY.md](PRIVACY.md)。

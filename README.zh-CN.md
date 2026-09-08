<div align="center">

# tauri 中文文档

[![原项目](https://img.shields.io/badge/原项目-tauri--apps--tauri-blue?style=flat-square&logo=github)](https://github.com/tauri-apps/tauri)
[![中文简介](https://img.shields.io/badge/中文简介-README.md-orange?style=flat-square)](README.md)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

</div>

> 本文件是 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 官方 README 的中文翻译，内容以英文原文为准。

## 简介

Tauri 是一个用于为所有主流操作系统构建体积极小、速度极快的二进制应用的框架。开发者可以集成任何能编译为 HTML、JS 和 CSS 的前端框架来构建用户界面。应用的后端是一个基于 Rust 的二进制程序，并提供一套可供前端调用的 API。

Tauri 应用的用户界面目前在 macOS、Windows、Linux、Android 和 iOS 上使用 [`tao`](https://docs.rs/tao) 作为窗口管理库。在渲染方面，Tauri 使用 [WRY](https://github.com/tauri-apps/wry)，它为系统 WebView 提供统一接口：在 macOS 和 iOS 上使用 WKWebView，在 Windows 上使用 WebView2，在 Linux 上使用 WebKitGTK，在 Android 上使用 Android System WebView。

想深入了解各个组件如何协同工作，请查阅 [ARCHITECTURE.md](https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md) 文档。

## 快速开始

如果你有兴趣开发 Tauri 应用，请访问[官方文档网站](https://tauri.app)。

最快的上手方式是先为你的系统安装[前置依赖](https://v2.tauri.app/start/prerequisites/)，再使用 [`create-tauri-app`](https://github.com/tauri-apps/create-tauri-app/#usage) 创建新项目。例如使用 `npm`：

```sh
npm create tauri-app@latest
```

创建项目后，进入目录安装依赖，即可用 `tauri dev` 启动开发模式、用 `tauri build` 构建发布包。

## 特性

Tauri 的特性清单包括但不限于：

- 内置应用打包器，可生成 `.app`、`.dmg`、`.deb`、`.rpm`、`.AppImage` 等格式，以及 `.exe`（经 NSIS）和 `.msi`（经 WiX）等 Windows 安装包。
- 内置自更新机制（仅桌面端）
- 系统托盘图标
- 原生通知
- 原生 WebView 协议（Tauri 不会启动本地 http(s) 服务器来提供 WebView 内容）
- 用于简化 CI 的 GitHub Action
- VS Code 扩展

### 支持平台

Tauri 目前支持在以下平台上进行开发和分发：

| 平台       | 版本                                                                                                            |
| :--------- | :-------------------------------------------------------------------------------------------------------------- |
| Windows    | 7 及以上                                                                                                        |
| macOS      | 10.15 及以上                                                                                                    |
| Linux      | Tauri v1 需要 webkit2gtk 4.0（例如 Ubuntu 18.04）；Tauri v2 需要 webkit2gtk 4.1（例如 Ubuntu 22.04）。          |
| iOS/iPadOS | 9 及以上                                                                                                        |
| Android    | 7 及以上（目前要求 8 及以上）                                                                                    |

## 参与贡献

在动手开发某项功能之前，最好先检查是否已经存在相关 issue。也建议先到 Discord 服务器与团队确认这个想法是否合理，或者是否已有人在做。

提交 Pull Request 之前，请务必阅读[贡献指南](./.github/CONTRIBUTING.md)。

感谢每一位为 Tauri 做出贡献的人！

### 文档

在一个多语言系统中维护文档是件麻烦事。为此，我们更倾向于在 Rust 与 JS 源代码中尽量使用内联文档。文档站托管仓库的更多信息请见：<https://github.com/tauri-apps/tauri-docs>

## 合作伙伴

<a href="https://crabnebula.dev" target="_blank">CrabNebula</a>

完整赞助者名单请访问我们的[官网](https://tauri.app#sponsors)和 [Open Collective](https://opencollective.com/tauri)。

## 组织

Tauri 希望成为以可持续自由开源社区原则为指引的可持续集体。为此，它已成为 [Commons Conservancy](https://commonsconservancy.org/) 旗下的一个项目（Programme），你可以通过 [Open Collective](https://opencollective.com/tauri) 提供资金支持。

## 许可证

代码：(c) 2015 - 至今 - The Tauri Programme within The Commons Conservancy。

采用 MIT 许可证，在适用的情况下采用 MIT / Apache 2.0 双许可证。

Logo：CC-BY-NC-ND

- Tauri 原始 Logo 设计者为 [Alve Larsson](https://alve.io/)、[Daniel Thompson-Yvetot](https://github.com/nothingismagick) 和 [Guillaume Chau](https://github.com/akryum)。

---

## 相关链接

- 原项目：<https://github.com/tauri-apps/tauri>
- 官方文档：<https://tauri.app>

---

> 本仓库为 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 的中文翻译版本，所有代码版权归原项目作者所有，遵循其原始许可证（MIT / Apache 2.0）。
>
> **代部署 / 定制服务 / 技术咨询 请添加微信：uaycar**
>
> **如果觉得有用，请给原项目点个 Star！** ⭐

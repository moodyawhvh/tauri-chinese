> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri

 <img align="right" src="https://github.com/tauri-apps/tauri/raw/dev/.github/icon.png" height="128" width="128">

[![status](https://img.shields.io/badge/status-stable-blue.svg)](https://github.com/tauri-apps/tauri/tree/dev)
[![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg)](https://opencollective.com/tauri)
[![test core](https://img.shields.io/github/actions/workflow/status/tauri-apps/tauri/test-core.yml?label=test%20core&logo=github)](https://github.com/tauri-apps/tauri/actions/workflows/test-core.yml)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Ftauri-apps%2Ftauri.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Ftauri-apps%2Ftauri?ref=badge_shield)
[![Chat Server](https://img.shields.io/badge/chat-discord-7289da.svg)](https://discord.gg/SpmNs4S)
[![website](https://img.shields.io/badge/website-tauri.app-purple.svg)](https://tauri.app)
[![https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg](https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg)](https://good-labs.github.io/greater-good-affirmation)
[![support](https://img.shields.io/badge/sponsor-Open%20Collective-blue.svg)](https://opencollective.com/tauri)

| 组件  | 版本                                                                                           |
| --------- | ---------------------------------------------------------------------------------------------- |
| tauri     | [![](https://img.shields.io/crates/v/tauri?style=flat-square)](https://crates.io/crates/tauri) |

## 关于 Tauri

Tauri 是一个多语言、通用的系统,组合性极强,可以让工程师构建种类繁多的应用。它将 Rust 工具与渲染在 WebView 中的 HTML 相结合,用于构建桌面应用程序。用 Tauri 构建的应用可以附带任意数量的可选 JS API / Rust API 组件,使 WebView 能够通过消息传递来控制系统。事实上,开发者可以用自己的功能扩展默认 API,轻松打通 WebView 与基于 Rust 的后端。

Tauri 应用可以拥有自定义菜单和托盘类界面。它们可以被更新,并由用户操作系统按预期方式进行管理。它们体积非常小,因为使用的是系统自带的 WebView。它们不附带运行时,因为最终二进制由 Rust 编译而成。这也使得对 Tauri 应用的逆向并非易事。

## 本模块

这是把所有东西粘合在一起的核心 crate。它将运行时、宏、工具和 API 汇集为一个最终产品。它在编译期读取 `tauri.conf.json` 文件,以启用特性并对应用进行实际配置(甚至会处理项目目录中的 `Cargo.toml` 文件)。它在运行时处理脚本注入(用于 polyfill / 原型修订),承载与系统交互的 API,甚至管理更新。

要深入了解各部分如何协同工作,请查阅 [ARCHITECTURE.md](https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md) 文档。

## 语义化版本

**tauri** 遵循[语义化版本 2.0](https://semver.org/)。

## 许可证

代码:(c) 2019 - 2021 - The Tauri Programme within The Commons Conservancy。

适用范围内采用 MIT 或 MIT/Apache 2.0 双许可。

Logo:CC-BY-NC-ND

- Tauri 原始 Logo 设计:[Daniel Thompson-Yvetot](https://github.com/nothingismagick) 和 [Guillaume Chau](https://github.com/akryum)

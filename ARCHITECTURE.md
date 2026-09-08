> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri 架构

<https://tauri.app>

<https://github.com/tauri-apps/tauri>

> 📝 注:本文件超过 10000 字符,以下仅翻译核心章节(Introduction、What Tauri is NOT、Major Components、External Crates);其余章节(Additional tooling、Tauri Plugins、Workflows、License)请参阅[英文原版](https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md)。

## 简介

Tauri 是一个多语言、通用的工具集,组合性极强,可以让工程师构建种类繁多的应用。它将 Rust 工具与渲染在 WebView 中的 HTML 相结合,用于构建桌面应用程序。用 Tauri 构建的应用可以附带任意数量的可选 JS API / Rust API 组件,使 WebView 能够通过消息传递来控制系统。事实上,开发者可以用自己的功能扩展默认 API,轻松打通 WebView 与基于 Rust 的后端。

Tauri 应用可以拥有自定义菜单和托盘类界面。它们可以被更新,并由用户操作系统按预期方式进行管理。它们体积非常小,因为使用的是操作系统自带的 WebView。它们不附带运行时,因为最终二进制由 Rust 编译而成。这也使得对 Tauri 应用的逆向并非易事。

## Tauri 不是什么

- Tauri 不是轻量级内核封装……而是直接使用 [WRY](#wry) 和 [TAO](#tao) 来完成与操作系统进行系统调用的重活。
- Tauri 不是虚拟机或虚拟化环境……而是一个用于构建 WebView 系统应用的应用工具集。

## 主要组件

以下章节简要描述 Tauri 各个部分的职责。

### Tauri Core [STABLE RUST]

#### [tauri](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri)

这是把所有东西粘合在一起的核心 crate。它将运行时、宏、工具和 API 汇集为一个最终产品。它在编译期读取 `tauri.conf.json` 文件,以启用特性并对应用进行实际配置(甚至会处理项目目录中的 `Cargo.toml` 文件)。它在运行时处理脚本注入(用于 polyfill / 原型修订),承载与系统交互的 API,甚至管理更新。

#### [tauri-build](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-build)

在构建期应用这些宏,以便装配 `cargo` 所需的一些特殊功能。

#### [tauri-codegen](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-codegen)

- 嵌入、哈希并压缩资源,包括应用图标和系统托盘图标。
- 在编译期解析 `tauri.conf.json` 并生成 Config 结构体。

#### [tauri-macros](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-macros)

借助 `tauri-codegen` crate,为上下文(context)、处理器(handler)和命令(command)创建宏。

#### [tauri-runtime](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-runtime)

这是 tauri 本身与底层 WebView 库之间的粘合层。

#### [tauri-runtime-wry](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-runtime-wry)

这个 crate 专门为 WRY 开放直接的系统级交互,例如打印、显示器检测以及其他窗口相关任务。它是 `tauri-runtime` 针对 WRY 的实现。

#### [tauri-utils](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-utils)

这是在许多地方复用的公共代码,提供诸如解析配置文件、检测平台三元组、注入 CSP 以及管理资源等实用工具。

### Tauri 工具链

#### [@tauri-apps/api](https://github.com/tauri-apps/tauri/tree/dev/packages/api) [TS -> JS]

一个 TypeScript 库,为你生成可导入前端框架的 `cjs` 与 `esm` JavaScript 入口,使 WebView 能够调用并监听后端活动。我们也同时发布纯 TypeScript 版本,因为对某些框架而言那样更优。它利用 WebView 向其宿主传递消息的机制工作。

#### [bundler](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-bundler) [RUST / SHELL]

bundler 是一个为它检测到/被指定的平台三元组构建 Tauri 应用的库。目前支持 macOS、Windows 和 Linux——不久的将来也将支持移动平台。它也可以在 Tauri 项目之外使用。

#### [@tauri-apps/cli](https://github.com/tauri-apps/tauri/tree/dev/packages/cli) [JS]

它是 [tauri-cli](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-cli) 的封装,使用 [napi-rs](https://github.com/napi-rs/napi-rs) 为每个平台生成 NPM 包。

#### [tauri-cli](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-cli) [RUST]

这个 Rust 可执行程序为 CLI 所需的全部操作提供完整接口。它可运行于 macOS、Windows 和 Linux。

#### [create-tauri-app](https://github.com/tauri-apps/create-tauri-app) [JS]

这是一个工具集,让工程团队能够使用自己选择的前端框架(前提是已被配置支持)快速搭建新的 tauri-apps 项目。

# 外部 Crate

Tauri-Apps 组织维护着两个 Tauri 的"上游"crate:用于创建和管理应用窗口的 TAO,以及与窗口内 WebView 交互的 WRY。

## [TAO](https://github.com/tauri-apps/tao)

用 Rust 编写的跨平台应用窗口创建库,支持 Windows、macOS、Linux、iOS 和 Android 等所有主要平台。它是 [winit](https://github.com/rust-windowing/winit) 的一个分支,我们为自身需求做了扩展,例如菜单栏和系统托盘。

## [WRY](https://github.com/tauri-apps/wry)

WRY 是用 Rust 编写的跨平台 WebView 渲染库,支持 Windows、macOS、Linux 等所有主要桌面平台。
Tauri 使用 WRY 作为抽象层,负责决定使用哪种 WebView(以及如何进行交互)。

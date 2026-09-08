<div align="center">

# tauri 中文翻译版

**[中文版] tauri — 用 Web 前端构建更小、更快、更安全的桌面与移动应用的开源框架**

[![原项目](https://img.shields.io/badge/原项目-tauri-apps--tauri-blue?style=flat-square&logo=github)](https://github.com/tauri-apps/tauri)
[![中文文档](https://img.shields.io/badge/中文文档-README.zh--CN.md-orange?style=flat-square)](README.zh-CN.md)
[![GitHub Stars](https://img.shields.io/github/stars/tauri-apps/tauri?style=flat-square&label=原项目Stars)](https://github.com/tauri-apps/tauri/stargazers)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

</div>

---

> 这是 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 的中文翻译版本。
> 完整源代码请访问原项目：https://github.com/tauri-apps/tauri

**代部署 / 定制服务 / 技术咨询 请添加微信：uaycar**

---

## 📖 项目简介

Tauri 是一个开源框架，帮助开发者用任意前端技术（凡是能编译为 HTML、JS、CSS 的框架都可以）搭配 Rust 后端，构建体积更小、速度更快、更安全的桌面与移动应用。它直接复用操作系统自带的 WebView（Windows 上的 WebView2、macOS/iOS 上的 WKWebView、Linux 上的 WebKitGTK、Android 上的系统 WebView），无需打包整个浏览器，产物体积和内存占用远小于传统方案。项目由 Commons Conservancy 旗下的可持续开源集体维护，采用 MIT / Apache 2.0 许可证。

## ✨ 主要特性

- 前端自由：支持任何可编译为 HTML、JS、CSS 的前端框架（React、Vue、Svelte 等）
- Rust 后端：编译为原生二进制，体积小、启动快、性能高
- 使用系统原生 WebView 渲染界面，不捆绑浏览器，更安全更轻量
- 全平台覆盖：Windows、macOS、Linux、iOS、Android
- 内置应用打包器：一键产出 `.app`、`.dmg`、`.deb`、`.rpm`、`.AppImage`，以及 `.exe`（NSIS）和 `.msi`（WiX）等 Windows 安装包
- 桌面端内置自更新机制
- 系统托盘图标与原生通知
- 原生 WebView 协议，不启动本地 http(s) 服务器来提供 WebView 内容
- 官方 GitHub Actions CI 集成与 VS Code 扩展

## 📁 文件说明

| 文件 | 说明 |
|:-----|:-----|
| README.md | 本文件（中文简介） |
| README.zh-CN.md | 详细中文文档（完整汉化） |

## 🚀 快速开始

1. 安装前置依赖：Rust 工具链及各平台系统依赖，详见官方文档 [Prerequisites](https://v2.tauri.app/start/prerequisites/)。
2. 使用官方脚手架创建新项目（以 npm 为例）：

```sh
npm create tauri-app@latest
```

3. 进入项目目录并安装依赖：

```sh
cd my-tauri-app
npm install
```

4. 以开发模式运行：

```sh
npm run tauri dev
```

5. 构建发布安装包：

```sh
npm run tauri build
```

完整源代码与最新版本请访问原项目：https://github.com/tauri-apps/tauri

## 📞 联系方式

**代部署 / 定制服务 / 技术咨询 请添加微信：uaycar**

---

本项目为 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 的中文翻译版本，所有代码版权归原项目作者所有，遵循其原始许可证。

**如果觉得有用，请给原项目点个 Star！** ⭐

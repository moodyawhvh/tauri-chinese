> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri 事件系统规范

## 事件发送方(Emitters)

发送方可以向任意目标的所有监听器发送事件。

- `App` 和 `AppHandle`
- `Window`
- `Webview`
- `WebviewWindow`
- 任何实现了 `Manager` trait 的类型。

## 发送函数

- `emit`:向所有监听器发送事件。
- `emit_to`:向指定目标发送事件。
- `emit_filter`:根据过滤回调向目标发送事件。

## 监听器(Listeners)

发送方可以向任意目标的所有监听器发送事件。

- `App` 和 `AppHandle`
- `Window`
- `Webview`
- `WebviewWindow`
- 任何实现了 `Manager` trait 的类型,但仅限使用 `listen_any/once_any`。

## 监听函数

- `listen`:只监听以本监听器类型为目标的所有事件。
- `once`:只监听一次以本监听器类型为目标的事件。
- `listen_any`(仅通过 `Manager` trait 提供):监听发往任意目标的所有事件(即事件嗅探器)。
- `once_any`(仅通过 `Manager` trait 提供):监听一次发往任意目标的事件(即事件嗅探器)。

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// 🌐 本文件注释由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 汉化,代码逻辑保持原样。

// 在 release 构建下隐藏 Windows 额外弹出的控制台窗口,切勿删除!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 定义一个 Tauri 命令:greet。
// #[tauri::command] 宏会生成必要的胶水代码,
// 让前端通过 invoke("greet", { name }) 直接调用这个 Rust 函数。
#[tauri::command]
fn greet(name: &str) -> String {
  // 使用 format! 拼接问候语并返回给前端
  format!("Hello {name}, You have been greeted from Rust!")
}

fn main() {
  tauri::Builder::default()
    // 注册命令处理器:只有列在这里的命令才能被前端调用
    .invoke_handler(tauri::generate_handler![greet])
    // generate_context! 在编译期读取 tauri.conf.json,
    // 生成应用上下文(窗口配置、图标、资源等)
    .run(tauri::generate_context!(
      "../../examples/helloworld/tauri.conf.json"
    ))
    // 运行失败时 panic 并输出错误信息
    .expect("error while running tauri application");
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// 🌐 本文件注释由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 汉化,代码逻辑保持原样。

// 在 release 构建下隐藏 Windows 额外弹出的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 定义一个 Tauri 命令:ping,无参数、无返回值。
// 该示例配合 isolation 模式,演示前端与后端的最简通信。
#[tauri::command]
fn ping() {
  // 打印当前时间戳,用于在前端调用时观察后端响应
  println!("ping: {:?}", std::time::Instant::now());
}

fn main() {
  tauri::Builder::default()
    // 注册命令处理器:让前端可以 invoke("ping")
    .invoke_handler(tauri::generate_handler![ping])
    // 编译期加载 isolation 示例的应用上下文(配置、图标、资源等)
    .run(tauri::generate_context!(
      "../../examples/isolation/tauri.conf.json"
    ))
    // 运行失败时 panic 并输出错误信息
    .expect("error while running tauri application");
}

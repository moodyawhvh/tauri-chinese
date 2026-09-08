> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri Bundler

将 Rust 可执行文件封装为特定操作系统的应用包。

## 关于

本项目是出色的 [cargo-bundle](https://github.com/burtonageo/cargo-bundle) 的一个分支,改造为库后供 [Tauri CLI](../tauri-cli) 使用。

### 稳定性

由于它主要供 `tauri-cli` 使用,其公开 API 并不严格遵循 SemVer。例如,minor 版本发布可能会新增结构体字段,或修改、删除 Error 枚举变体。

## 配置

Tauri 会自动从 `tauri.conf.json > bundle` 对象加载配置,但本库并不依赖它,非 Tauri 应用也可以使用。

### 通用设置

这些设置适用于所有(或大多数)操作系统的打包。

- `name`:构建出的应用名称。若未提供,则使用 `Cargo.toml` 文件中的 `name` 值。
- `identifier`:[必需] 唯一标识你的应用的字符串,采用反向 DNS 形式(例如 `"com.example.appname"` 或 `"io.github.username.project"`)。在 OS X 和 iOS 上,它用作 bundle 的 `CFBundleIdentifier` 值;在 Windows 上,它会被哈希以生成应用 GUID。
- `icon`:[可选] 应用使用的图标。它应当是一个文件路径或通配符(glob)数组(包含各种尺寸/格式的图片);`tauri-bundler` 会根据不同平台的需要自动转换图片格式。支持的格式包括 ICNS、ICO、PNG,以及任何能被 [`image`](https://crates.io/crates/image) crate 解码的格式。面向高分辨率(如 Retina)显示屏的图标,文件名应在扩展名前带 `@2x`(见下方示例)。
- `version`:[可选] 应用版本。若未提供,则使用 `Cargo.toml` 文件中的 `version` 值。
- `resources`:[可选] 会被复制到 bundle 资源区的文件或目录列表。支持通配符。
- `copyright`:[可选] 与你的应用关联的版权字符串。
- `category`:[可选] 应用的类别。可以是人类可读的字符串(如 `"Puzzle game"`)、Mac OS X 的 LSApplicationCategoryType 值(如 `"public.app-category.puzzle-games"`),或 GNOME desktop 文件的类别名(如 `"LogicGame"`),`tauri-bundler` 会根据不同平台自动转换。
- `short_description`:[可选] 应用的简短单行描述。若未提供,则使用 `Cargo.toml` 文件中的 `description` 值。
- `long_description`:[可选] 应用较长、可多行的描述。

### Debian 专属设置

这些设置仅在打包 `deb` 包时使用。

- `depends`:字符串列表,指明本包安装时所依赖的其他包(例如共享库)。若提供,将构成 `deb` 包控制文件中的 `Depends:` 字段。

### Mac OS X 专属设置

这些设置仅在打包 `app` 和 `dmg` 包时使用。

- `frameworks`:字符串列表,指明需要随应用一起打包的 Mac OS X 框架。每个字符串可以是框架名(不带 `.framework` 扩展名,如 `"SDL2"`),此时 `tauri-bundler` 会在标准安装位置(`~/Library/Frameworks/`、`/Library/Frameworks/` 和 `/Network/Library/Frameworks/`)搜索该框架;也可以是指向特定框架 bundle 的路径(如 `./data/frameworks/SDL2.framework`)。注意,该设置只是让 `tauri-bundler` 把指定框架复制进 OS X 应用包(位于 `Foobar.app/Contents/Frameworks/` 下);你仍需自行负责:(1) 让编译出的二进制链接这些框架(例如在 `build.rs` 脚本中输出 `cargo:rustc-link-lib=framework=SDL2` 这样的行),以及 (2) 在二进制中嵌入正确的 rpath(例如编译后运行 `install_name_tool -add_rpath "@executable_path/../Frameworks" path/to/binary`)。
- `minimum_system_version`:版本字符串,指明打包后的应用支持的最低 Mac OS X 版本(如 `"10.11"`)。如果使用此配置字段,你可能还想让 `build.rs` 脚本输出 `cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11`(或你想要的任意版本号),以确保编译出的二进制具有相同的最低版本要求。
- `license`:DMG 包许可证文件的路径。
- `exception_domain`:macOS .app 包使用的例外域名(exception domain)。允许与外界通信,例如你随应用分发的 Web 服务器。
- `provider_short_name`:如果你的 Apple ID 关联了多个团队,必须指定你要用于公证应用的团队 provider short name。参见[自定义公证工作流](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow)并搜索 `--list-providers`,了解如何获取你的 provider short name。

### `tauri.conf.json` 示例:

```json
{
  "productName": "Your Awesome App",
  "version": "0.1.0",
  "identifier": "com.my.app",
  "app": {},
  "bundle": {
    "active": true,
    "shortDescription": "",
    "longDescription": "",
    "copyright": "Copyright (c) You 2021. All rights reserved.",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": ["./assets/**/*.png"],
    "deb": {
      "depends": ["debian-dependency1", "debian-dependency2"]
    },
    "macOS": {
      "frameworks": [],
      "minimumSystemVersion": "10.11",
      "license": "./LICENSE"
    },
    "externalBin": ["./sidecar-app"]
  }
}
```

## 许可证

(c) 2017 - 至今,George Burton,Tauri-Apps Organization

本程序采用 [Apache Software License](http://www.apache.org/licenses/LICENSE-2.0) 或 [MIT License](https://opensource.org/licenses/MIT) 之一授权。

-> 注意:对于 bundle_dmg,我们收录了一个 BSD 3 许可的二进制 `seticon`。
https://github.com/sveinbjornt/osxiconutils/blob/master/seticon.m
`tools/rust/cargo-tauri-bundle/src/bundle/templates/seticon`

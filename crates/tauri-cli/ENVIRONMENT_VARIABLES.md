> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

### Tauri 的环境变量

本文档收录 tauri 核心 crate 和 tauri CLI 使用的全部环境变量。

### Tauri CLI

这些环境变量是 CLI 的输入,其中许多都有等价的 CLI 标志(flag)。

> 如果同时使用了环境变量和 CLI 标志,CLI 标志优先。

- `CI` — 若已设置,CLI 将以 CI 模式运行,不要求任何用户交互。
- `TAURI_CLI_CONFIG_DEPTH` — 向上遍历查找 tauri 配置文件的层级数。
- `TAURI_CLI_PORT` — CLI 内置开发服务器使用的端口。
- `TAURI_CLI_WATCHER_IGNORE_FILENAME` — 一个 `.gitignore` 风格文件的文件名,用于控制 `dev` 命令中 CLI 应监视哪些文件。CLI 会在每个目录中查找该文件名。
- `TAURI_CLI_NO_DEV_SERVER_WAIT` — 跳过等待前端开发服务器启动,直接构建 tauri 应用。
- `TAURI_LINUX_AYATANA_APPINDICATOR` — 将此变量设为 `true` 或 `1`,可在 Linux 上强制使用 `libayatana-appindicator` 作为系统托盘。
- `TAURI_BUNDLER_WIX_FIPS_COMPLIANT` — 指定 bundler 的 WiX `FipsCompliant` 选项。
- `TAURI_BUNDLER_TOOLS_GITHUB_MIRROR` — 指定一个 GitHub 镜像,用于下载 tauri bundler 所需的文件和工具。
- `TAURI_BUNDLER_TOOLS_GITHUB_MIRROR_TEMPLATE` — 指定一个 GitHub 镜像模板,用于下载 tauri bundler 所需的文件和工具,例如:`https://mirror.example.com/<owner>/<repo>/releases/download/<version>/<asset>`。
- `TAURI_BUNDLER_DMG_IGNORE_CI` — 禁用 `.dmg` bundler 中对 `CI: true` 的检查。
- `TAURI_SKIP_SIDECAR_SIGNATURE_CHECK` — 跳过对 sidecar 的签名。
- `TAURI_SIGNING_PRIVATE_KEY` — 用于给应用包签名的私钥。对于 `build` 和 `bundle` 命令,可以是字符串本身,也可以是文件路径;对于 `signer sign` 命令,必须是密钥字符串本身。
- `TAURI_SIGNING_PRIVATE_KEY_PATH` — `signer sign` 命令使用的私钥文件路径。使用 signer 命令时与 `TAURI_SIGNING_PRIVATE_KEY` 互斥。
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — 签名私钥的密码,参见 `TAURI_SIGNING_PRIVATE_KEY`。
- `TAURI_SIGNING_RPM_KEY` — 用于给 RPM 包签名的私有 GPG 密钥,需导出为 ASCII-armored 格式。
- `TAURI_SIGNING_RPM_KEY_PASSPHRASE` — `TAURI_SIGNING_RPM_KEY` 的 GPG 密钥口令(如需要)。
- `TAURI_WINDOWS_SIGNTOOL_PATH` — 指定 Windows 上用于应用代码签名的 `signtool.exe` 路径。
- `STATIC_VCRUNTIME` — 已废弃。请改用 `tauri.conf.json` 中的 `build > windows > staticVCRuntime`。
- `VCTOOLS_REDIST_DIR` — 覆盖在启用 `bundle > windows > bundleVCRuntime` 时使用的 Visual C++ 运行库根目录。若未设置,Tauri 会使用自带的 `vswhere.exe` 定位 Visual Studio 并推导出该目录。
- `APPLE_CERTIFICATE` — 用于代码签名的 `.p12` 证书的 Base64 编码。获取方法:运行 `openssl base64 -in MyCertificate.p12 -out MyCertificate-base64.txt`。
- `APPLE_CERTIFICATE_PASSWORD` — 导出证书时使用的密码。
- `APPLE_ID` — 用于公证(notarize)应用的 Apple ID。如果提供此环境变量,则必须同时设置 `APPLE_PASSWORD` 和 `APPLE_TEAM_ID`。也可以改用 `APPLE_API_KEY` 和 `APPLE_API_ISSUER` 进行认证。
- `APPLE_PASSWORD` — 用于应用公证认证的 Apple 密码。指定了 `APPLE_ID` 时必填。可以使用 App 专用密码。除了以明文输入密码,也可以使用 `@keychain:` 或 `@env:` 前缀,后跟钥匙串密码项名称或环境变量名称。
- `APPLE_TEAM_ID`:开发者团队 ID。要查找你的 Team ID,请前往 Apple 开发者网站的 [Account](https://developer.apple.com/account) 页面,查看你的会员详情。
- `APPLE_API_KEY` — 使用 JWT 进行公证认证时,可替代 `APPLE_ID` 和 `APPLE_PASSWORD`。同时也可用于自动管理 iOS 证书和描述文件。
  - 详情参见[创建 API 密钥](https://developer.apple.com/documentation/appstoreconnectapi/creating_api_keys_for_app_store_connect_api)。
- `API_PRIVATE_KEYS_DIR` — 指定 AuthKey 文件所在目录。参见 `APPLE_API_KEY`。
- `APPLE_API_ISSUER` — Issuer ID。指定了 `APPLE_API_KEY` 时必填。
- `APPLE_API_KEY_PATH` — API 密钥 `.p8` 文件的路径。若未指定,对于 macOS 应用,bundler 会按顺序在以下目录中搜索名为 `AuthKey\_<api_key>.p8` 的私钥文件:`./private_keys`、`~/private_keys`、`~/.private_keys`、`~/.appstoreconnect/private_keys`。**对于 iOS 应用,此变量为必填**。
- `APPLE_SIGNING_IDENTITY` — 用于代码签名的身份(identity)。会覆盖 `tauri.conf.json > bundle > macOS > signingIdentity`。若两者都未设置,则在提供 `APPLE_CERTIFICATE` 时由其推导。
- `APPLE_PROVIDER_SHORT_NAME` — 如果你的 Apple ID 关联了多个团队,必须指定你要用于公证应用的团队 provider short name。会覆盖 `tauri.conf.json > bundle > macOS > providerShortName`。
- `APPLE_DEVELOPMENT_TEAM` — 在 iOS 上用于代码签名的团队 ID。会覆盖 `tauri.conf.json > bundle > iOS > developmentTeam`。可在 https://developer.apple.com/account#MembershipDetailsCard 查看。
- `TAURI_WEBVIEW_AUTOMATION` — 启用 WebView 自动化(仅限 Linux)。
- `TAURI_ANDROID_PROJECT_PATH` — tauri Android 项目的路径,通常为 `<project>/src-tauri/gen/android`。
- `TAURI_IOS_PROJECT_PATH` — tauri iOS 项目的路径,通常为 `<project>/src-tauri/gen/ios`。

### Tauri CLI 钩子命令环境变量

这些环境变量会为每个钩子命令(`beforeDevCommand`、`beforeBuildCommand` 等)设置,可用于按条件构建前端或执行特定操作。

- `TAURI_ENV_DEBUG` — `dev` 命令或 `build --debug` 时为 `true`,否则为 `false`。
- `TAURI_ENV_TARGET_TRIPLE` — CLI 正在构建的目标三元组(target triple)。
- `TAURI_ENV_ARCH` — 目标架构,如 `x86_64`、`aarch64` 等。
- `TAURI_ENV_PLATFORM` — 目标平台,如 `windows`、`darwin`、`linux` 等。
- `TAURI_ENV_PLATFORM_VERSION` — 构建平台版本。
- `TAURI_ENV_FAMILY` — 目标平台家族:`unix` 或 `windows`。

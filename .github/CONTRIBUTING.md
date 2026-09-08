> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri 贡献指南

你好!我们(维护者团队)非常高兴你有兴趣为 Tauri 做贡献。不过在提交贡献之前,请先花点时间阅读[行为准则](/.github/CODE_OF_CONDUCT.md),以及与你打算做的贡献类型对应的章节:

- [Issue 报告指南](#issue-报告指南)
- [Pull Request 指南](#pull-request-指南)
- [开发指南](#开发指南)
- [AI 工具政策](#ai-工具政策)

## Issue 报告指南

- 本仓库的 issue 列表**仅**用于 bug 报告和功能请求。不符合规范的 issue 会被立即关闭。

- 如果你有疑问,可以在 [Tauri Discord 聊天室](https://discord.gg/SpmNs4S)快速获得解答。

- 请先搜索一下你的 issue,它可能已经被回答过,甚至已经在开发分支(`dev`)中修复。

- 检查该问题在最新稳定版 Tauri 上是否可以复现。如果你使用的是预发布版本,请注明你所用的具体版本。

- 你**必须**清楚描述复现该问题所需的具体步骤。虽然我们非常乐意尽可能帮助用户,但在没有清晰复现步骤的情况下诊断问题极其耗时,根本不可持续。

- 请只使用复现该异常行为所需的最少代码。一份好的 bug 报告应当隔离出表现异常的具体方法,并精确定义预期是如何被打破的:你原本期望这些方法做什么,实际观察到的行为又有什么不同?问题隔离得越精确,我们排查得越快。

- 没有清晰复现步骤的 issue 不会被分诊。如果带有 "need repro" 标签的 issue 超过 5 天没有收到作者的进一步反馈,就会被关闭。

- 如果你的问题已解决但 issue 仍然开着,请果断关闭它。如果是你自己找到了解决方案,说明一下修复方法会对其他人很有帮助。

- 最重要的是,恳请你保持耐心:团队必须在你的请求与众多其他事务之间取得平衡——修复其他 bug、回答其他问题、开发新功能、编写新文档等等。issue 列表不是付费技术支持,我们无法保证你的问题多快能被解决。

## Pull Request 指南

- 你必须[对提交进行签名](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)。

- 在 PR 开发过程中包含多个小提交没有问题——合并前我们会让 GitHub 自动 squash。

- 如果是新增功能:
  - 请给出添加该功能的充分理由。理想情况下,你应当先开一个建议类 issue 并获得许可,再动手实现。

- 如果是修复 bug:
  - 如果你解决的是某个特定 issue,请在 PR 标题中加上 `(fix: #xxxx[,#xxx])`(#xxxx 是 issue 编号),以便生成更好的发布日志,例如 `fix: update entities encoding/decoding (fix #3899)`。
  - 请在 PR 中详细描述该 bug,或链接到一个已详细描述的 issue。

- 如果该 PR 需要随版本发布,请按照 `.changes/readme.md` 中的说明记录你的变更,即 [readme.md](https://github.com/tauri-apps/tauri/blob/dev/.changes/README.md)。

## 开发指南

**注意:如有任何疑问,欢迎随时在我们的 Discord 服务器里提问。我们会尽量保持本指南的时效性,但如果哪一步走不通,请告诉我们。**

### 通用环境准备

首先,[加入我们的 Discord 服务器](https://discord.gg/SpmNs4S),让我们知道你想做贡献。这样我们可以为你指明方向,并确保你的贡献尽可能有帮助。

要将你的机器配置为开发环境,请按照 [Tauri 环境搭建指南](https://v2.tauri.app/start/prerequisites/)安装开发 Tauri 应用所需的全部工具。你可能还需要的唯一额外工具是 [PNPM](https://pnpm.io/),只有在开发 Node CLI 或 API 包(`packages/cli` 和 `packages/api`)时才需要它。

接下来,[fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo) 并克隆[本仓库](https://github.com/tauri-apps)。
开发流程因你贡献的 Tauri 部分而异,各包的具体说明见下方指南。

部分 Tauri 包会在运行某个示例时自动构建;另一些则需要预先构建。作为初始化,请在仓库根目录执行以下命令:

```bash
pnpm install
pnpm build
```

### 总览

各包的整体情况请参阅[架构文档](/ARCHITECTURE.md#major-components)。

### 开发 Tauri Core 及相关组件(Rust API、Macros、Codegen 和 Utils)

Rust crate 的代码,包括 Core、Macros、Utils、WRY 运行时等,位于[主 Tauri 仓库](https://github.com/tauri-apps/tauri/tree/dev/crates)。

测试改动最简单的方法是使用 [helloworld](https://github.com/tauri-apps/tauri/tree/dev/examples/helloworld) 示例应用。它会自动重新构建并使用你本地的 Tauri core 包副本。修改代码后只需运行 `cargo run --example helloworld` 即可验证。

要针对你自己的应用测试本地改动,只需让 Tauri 指向你本地仓库。在 `src-tauri/Cargo.toml` 中把:

`tauri = { version = "2.1.1" }`

改为:

`tauri = { path = "path/to/local/tauri/crates/tauri" }`

如果还有其他 crate 依赖 Tauri,你也需要把它们指向本地仓库。

### 开发 Tauri Bundler 与 Rust CLI

bundler 的代码位于 [crates/tauri-bundler](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-bundler),Rust CLI 的代码位于 [tauri-cli](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-cli)。
在 Rust CLI 目录中运行 `cargo install --path .`,即可在任何地方使用更新后的 bundler 和 cli 来执行 `cargo tauri build` 和 `cargo tauri dev`。这两个包每次改动后都需要重新执行该命令。
你可以使用 `cargo install --path . --debug` 来加快测试构建速度。

### 开发 Node.js CLI(`@tauri-apps/cli`)

[`@tauri-apps/cli`](https://github.com/tauri-apps/tauri/tree/dev/packages/cli) 只是 `tauri-cli` 的一个薄封装,因此大多数改动都应发生在 Rust CLI 中(见上文)。

#### 本地构建文档

你可以在本地运行以下脚本来构建 Rust 文档:

```bash
$ cargo +nightly doc --all-features --open
```

### 开发 JS API

JS API 在开发者运行于 WebView 中的 JS 与用 Rust 编写的内置 Tauri API 之间提供绑定。其代码位于 [/packages/api](https://github.com/tauri-apps/tauri/tree/dev/packages/api)。
修改代码后,运行 `pnpm build` 进行构建。要测试你的改动,我们推荐使用 API 示例应用,位于 [/examples/api](https://github.com/tauri-apps/tauri/tree/dev/examples/api)。它会自动使用你本地的 JS API 副本,并提供一个便于测试各种命令的 UI。

## AI 工具政策

审阅一个 Pull Request 需要大量时间,而用 AI 工具生成一个看似合理实则毫无意义 的 PR 却非常容易。
让其他贡献者和审阅者把大量时间耗在这上面是不公平的,因此定下以下规则:

1. 提交前必须审阅并测试所有 LLM 生成的内容,责任在你,而不是 AI。
2. 不要用 AI 回复审阅意见(翻译除外)。

如果你不遵守这些规则,我们将给 PR 打上 `ai-slop` 标签并关闭。

## 资金贡献

Tauri 是一个采用 MIT 许可证的开源项目。你可以通过 [GitHub Sponsors](https://github.com/sponsors/tauri-apps) 或 [Open Collective](https://opencollective.com/tauri) 支持它的持续开发。我们更推荐 GitHub Sponsors,因为通过配捐计划,捐款金额会被翻倍。

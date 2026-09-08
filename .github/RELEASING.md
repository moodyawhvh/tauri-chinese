> 🌐 本文档由 [tauri-apps/tauri](https://github.com/tauri-apps/tauri) 翻译,英文原版见原项目。

# Tauri 发布手册

本手册包含我们的发布流水线信息以及常见问题的处理方法。
本文档主要面向负责维护本项目的团队成员。

- [Covector](#covector)
- [版本 Pull Request](#版本-pull-request)
- [发布与推送](#发布与推送)
- [推送失败了怎么办?](#推送失败了怎么办)

## Covector

我们使用 [`covector`](https://github.com/jbolda/covector) 来管理版本号升级和发布流水线。
它通过 [`.changes/config.json`](../.changes/config.json) 配置,其中定义了每个包应如何逐步发布。

有些包无法直接通过 `covector` 发布,因为它们需要在多平台矩阵上构建,
例如 `tauri-cli` 预编译二进制,通过 [publish-cli-rs.yml](./workflows/publish-cli-rs.yml) 发布;
以及 `@tauri-apps/cli` 的原生 Node.js 模块,通过 [publish-cli-js.yml](./workflows/publish-cli-js.yml) 发布。
两者都在 `covector` 为它们创建 GitHub Release 之后触发,参见 [covector-version-or-publish.yml](./workflows/covector-version-or-publish.yml) 中的
`Trigger @tauri-apps/cli publishing workflow` 和 `Trigger tauri-cli publishing workflow` 步骤。

## 版本 Pull Request

每次 pull request 被合并时,[covector-version-or-publish.yml](./workflows/covector-version-or-publish.yml) 工作流都会运行,并且:

当 `.changes` 目录中存在变更文件、且它们并未全部包含在 `pre.json` 中时(通常只有在我们处于 `-alpha` 到 `-rc` 阶段时才会出现这种情况),它会创建/更新一个名为 `Apply Version Updates From Current Changes` 的 PR(例如 https://github.com/tauri-apps/tauri/pull/11029 ),基于现有变更文件升级所有包的版本,并生成 `CHANGELOG.md` 条目。参见 [covector-version-or-publish.yml](./workflows/covector-version-or-publish.yml) 中的 `Create Pull Request With Versions Bumped` 步骤。

否则,covector 将开始发布在 [`.changes/config.json`](../.changes/config.json) 中配置的各个包。

## 发布与推送

发布可以简单到只需合并版本 PR,但请遵循以下清单:

- [ ] 再次确认每个包的版本升级都正确,没有意外发布 major 或 minor 版本——除非那确实是本意。
- [ ] 确保没有处于等待或未完成状态的 [covector-version-or-publish.yml](./workflows/covector-version-or-publish.yml) 工作流运行。
- [ ] 批准并合并版本 pull request

## 推送失败了怎么办?

由于诸多因素,一个或多个包的发布有可能失败。不必惊慌,我们可以修复。

是所有包都发布失败了吗?

- 是?
  - [ ] `git checkout -b revert-branch`
  - [ ] `git revert HEAD~1`
- 不是?
  - [ ] `git checkout -b revert-branch`
  - [ ] `git revert HEAD~1 --no-commit`
  - [ ] 编辑该提交,只回滚与发布失败的包相关的改动
  - [ ] `git revert --continue`

然后:

- [ ] 用回滚后的改动创建一个 PR,获得批准并合并
- [ ] 在另一个 PR 中修复导致发布失败的问题,获得批准并合并
- [ ] 重新执行一遍发布流程。

# VSCode 插件开发

## 说明

- 本页记录 `VSCode` 插件的初始化, 发布和常见排错要点.
- 适合作为从零创建扩展或准备发布时的最小操作清单.

## 初始化项目

```sh
npm config set registry https://registry.npmmirror.com
npm install -g yo generator-code
yo code
```

初始化时建议先明确:

- 插件是命令型, 语言服务型, 还是 UI 面板型.
- 是否需要 Webview, TreeView 或 LSP 支持.

## 插件发布

参考: <https://juejin.cn/post/7076649162653040647>

```sh
yarn global add vsce
```

### 权限准备

1. 申请 Azure DevOps 的个人令牌.
2. 创建一个发布人 `publisher`.
3. 执行 `vsce login <publisher name>` 并输入 token.
4. 执行 `vsce publish`.

令牌说明参考:

- <https://learn.microsoft.com/zh-cn/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate?view=azure-devops&tabs=Windows>

## 常见问题

- 插件发布后安装成功但无法激活, 常见原因是依赖被放在 `devDependencies`.
- 这类运行期依赖需要移动到正式依赖中, 再提升版本重新发布.
- 若命令已注册但看不到, 先检查 `package.json` 中的 `contributes` 配置.
- 若调试时不生效, 先确认 Extension Development Host 是否加载了当前工作区插件.

## 使用建议

- 先做一个只注册命令的最小插件, 再逐步加复杂能力.
- 发布前最好本地打包一次, 确认产物和依赖完整.
- 若后续继续整理, 可补 Webview, LSP, 配置项和命令面板示例.

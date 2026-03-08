# Trae 插件安装记录

## 说明

- 本页记录在 `Trae` 中安装或手动下载扩展包的最小方法.
- 当前内容主要用于处理“商店无法直接安装”或“需要固定插件版本”这类场景.

## 参考资料

- 官方说明: <https://docs.trae.cn/ide/manage-extensions>
- VSCode Marketplace: <https://marketplace.visualstudio.com/>

## 手动拼接插件下载地址

通用格式:

```text
https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension}/{version}/vspackage
```

参数含义:

- `publisher`: 发布者名称.
- `extension`: 插件标识.
- `version`: 目标版本号.

## Dioxus 插件示例

- 插件页: <https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus>
- `publisher`: `DioxusLabs`
- `extension`: `dioxus`
- `version`: `0.6.0`

最终下载地址:

```text
https://marketplace.visualstudio.com/_apis/public/gallery/publishers/DioxusLabs/vsextensions/dioxus/0.6.0/vspackage
```

## 使用建议

- 先从插件详情页确认 `publisher`, `extension` 和版本号.
- 若只是为了临时安装, 建议优先使用官方商店流程.
- 若需要离线分发或锁定版本, 再使用手动下载地址保存本地包.
- 插件更新后若地址失效, 优先回到插件详情页确认最新版本号.

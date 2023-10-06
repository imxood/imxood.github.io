## vscode 编辑器

CodeLLDB 插件

launch.json 中的配置:

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Cargo launch",
    "cargo": {
        // -C 参数 指定 Cargo.toml 位置
        "args": [
            "-Z",
            "unstable-options",
            "-C",
            "${workspaceFolder}/ProjectName",
            "build"
        ]
    },
    "args": ["ProjectArg1", "ProjectArg2"]
}
```

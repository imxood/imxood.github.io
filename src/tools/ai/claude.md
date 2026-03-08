# Claude

## 说明

- 本页记录本地使用 `Claude` CLI 时的高频命令, MCP 配置和插件安装方式.
- 适合作为“先跑起来, 再逐步接入工具”的操作速记页.

## 运行 Claude

```sh
claude --dangerously-skip-permissions
```

使用提醒:

- 该参数会跳过一部分权限确认流程.
- 仅在你明确理解当前环境和风险时再使用.

## 配置文件

常见配置文件位置:

```text
~/.claude.json
```

如果命令行添加 MCP 失败, 可以考虑直接编辑配置文件.

## 添加 MCP

### `pdf-reader-mcp`

```sh
pnpm add @sylphlab/pdf-reader-mcp
claude mcp add --transport stdio pdf-reader-mcp -- npx @sylphlab/pdf-reader-mcp
```

### `context7`

```sh
claude mcp add -s user context7 -- npx -y @upstash/context7-mcp
```

### `mcp-server-time`

```sh
npx @michaellatman/mcp-get@latest install mcp-server-time
claude mcp add --transport stdio time -- uvx mcp-server-time
```

### `KiCAD-MCP-Server`

命令行示例:

```sh
claude mcp add --transport stdio KiCAD-MCP-Server -e "PYTHONPATH=C:\Program Files\KiCad\9.0\lib\python3\dist-packages" -e "NODE_ENV=production" -e "LOG_LEVEL=info" -- node E:\git\python\KiCAD-MCP-Server\dist\index.js
```

如果命令行参数不好处理, 可以直接修改 `~/.claude.json`:

```json
{
  "mcpServers": {
    "kicad": {
      "command": "node",
      "args": ["E:\git\python\KiCAD-MCP-Server\dist\index.js"],
      "env": {
        "PYTHONPATH": "C:\Program Files\KiCad\9.0\lib\python3\dist-packages",
        "NODE_ENV": "production",
        "LOG_LEVEL": "info"
      }
    }
  }
}
```

## 插件安装

### 安装 LSP 插件

```sh
/plugin marketplace add boostvolt/claude-code-lsps
/plugin install rust-analyzer@claude-code-lsps
```

### 安装 `planning-with-files`

```sh
/plugin marketplace add OthmanAdi/planning-with-files
/plugin install planning-with-files@planning-with-files
```

## 使用建议

- 先保证基础 CLI 可运行, 再逐步添加 MCP 和插件.
- 每添加一个 MCP 后, 先单独验证其是否可正常启动.
- 若多个工具一起接入, 建议优先把路径, 环境变量和日志级别写进配置文件统一管理.
- 若后续继续整理, 可补“常见报错”, “配置迁移” 和 “工具组合方案”笔记.

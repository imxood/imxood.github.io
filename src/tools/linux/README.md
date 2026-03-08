# Linux 总览

## 说明

- 本目录收录 Linux 日常工具, 发行版记录, 桌面环境, Homebrew 与兼容层相关笔记.
- 若问题偏通用命令或排障, 可先看 `linux工具` / `linux常用命令` / `Linux 综合笔记`; 若偏发行版, 再看 `Ubuntu` / `Deepin` / `Manjaro`.

## 阅读路径

- 通用命令与工具链问题, 优先看 `Linux 工具` 和 `Linux 常用命令`.
- 系统级杂项排查, 优先看 `Linux 综合笔记` 与 `Linux 常见问题`.
- 发行版差异问题, 再看 `Ubuntu`, `Deepin`, `Manjaro`.
- 包管理器补装工具场景, 可继续看 `Homebrew`.
- 兼容运行 Windows 软件时, 再看 `Wine 笔记`.

## 常用入口

- [Linux 工具](./linux工具.md)
- [Linux 常用命令](./linux常用命令.md)
- [Linux 综合笔记](./Linux笔记.md)
- [Linux 常见问题](./issues.md)
- [Homebrew](./homebrew.md)
- [OpenWrt / LEDE 编译](../openwrt/leda编译.md)

## 发行版与桌面

- [Ubuntu](./ubuntu.md)
- [Deepin](./deepin.md)
- [Manjaro 笔记](./manjaro笔记.md)
- [KDE](./kde.md)

## 兼容层与扩展

- [Wine 笔记](./wine/wine笔记.md)
- `make_system/` 下保留了一批历史脚本, 更适合视为资料归档, 不建议直接照搬执行.

## 适合如何使用

- 先用总览页判断问题属于“命令排查”, “发行版差异”, 还是“兼容层 / 桌面环境”问题.
- 遇到安装失败或环境异常时, 先查对应发行版页, 再回到更底层的命令或日志排查.
- 历史页面里有不少版本敏感命令, 新机器执行前要先确认系统版本和镜像源状态.

## 目录边界

- 本目录主要关注 Linux 系统环境与桌面工具层面的知识.
- 若问题已经落到具体语言, 框架或数据库, 通常应回到对应专题目录继续看.
- 若后续继续治理, 可逐步把“通用命令速查”和“发行版初始化记录”拆成更清晰的两层入口.

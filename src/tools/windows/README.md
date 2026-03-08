# Windows 总览

## 说明

- 本目录收录 Windows 系统配置, 包管理, 服务封装, 远程访问和二进制分析等记录.
- 若问题偏日常环境与系统设置, 优先看 [Windows 笔记](./windows.md) 和 [Windows 工具](./windows工具.md).
- 若问题偏包管理或开发依赖, 可继续看 `Chocolatey` 和 `vcpkg`.

## 常用入口

- [Windows 笔记](./windows.md)
- [Windows 工具](./windows工具.md)
- [WSL](./wsl.md)
- [Windows 11 安卓子系统](../android/android.md)
- [Chocolatey](./chocolatey.md)
- [vcpkg](./vcpkg.md)
- [NSSM](./nssm.md)
- [Windows 安装 OpenSSH Server](./安装openssh%20server.md)
- [查看 dll 或 lib 文件函数定义](./dll%20lib%20文件接口查询.md)

## 使用建议

- 系统级操作尽量先确认管理员权限, 服务状态和环境变量再执行.
- 和 Rust / C++ 工具链联动时, 常见入口是 `Chocolatey`, `vcpkg`, `OpenSSH Server` 和二进制导出查看工具.

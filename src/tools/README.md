# 工具总览

## 说明

- 本目录收录开发环境, 系统工具, 容器, 数据库, AI 工具等使用记录.
- 常用主题优先从本页和 `SUMMARY` 进入, 再深入具体笔记.
- 零散旧文档会逐步并入更稳定的专题页.

## 阅读路径

- 构建链和调试链问题, 优先看 `编译工具`, `OpenOCD 与 GDB`, `Make`, `CMake`, `mdBook`.
- 系统环境问题, 优先看 `Linux 总览`, `Ubuntu`, `Windows 总览`, `Shell 笔记`.
- 容器与服务部署问题, 优先看 `Docker 总览`, `OpenResty`, `阿里云服务`, `VPS 运维笔记`.
- 数据与 AI 相关问题, 优先看 `SQL 总览`, `MongoDB`, `AI 工具总览`, `RAGFlow`.

## 构建与文档

- [Git](./git.md)
- [Markdown](./markdown.md)
- [LaTeX 笔记](./latex笔记.md)
- [mdBook](./mdbook.md)
- [Make](./make/make笔记.md)
- [CMake 总览](./cmake/README.md)
- [CMake 笔记](./cmake/cmake笔记.md)
- [编译与调试总览](./compile/README.md)
- [编译工具](./compile/编译工具.md)
- [OpenOCD 与 GDB](./compile/openocd%20gdb%20笔记.md)

## 开发环境与媒体

- [VSCode](./vscode/vscode.md)
- [VSCode 插件开发](./vscode/vscode插件开发.md)
- [Trae 插件安装记录](./trae.md)
- [FFmpeg](./ffmpeg.md)
- [qemu_stm32](./qemu_stm32/qemu_stm32.md)
- [Bochs 上手指南](./bochs/新手指南.md)
- [Go 环境总览](./go/README.md)
- [Ubuntu 安装 Go](./go/ubuntu安装go.md)
- [主板记录](./主板.md)
- [AutoCAD](./autocad.md)

## 系统与环境

- [Linux 总览](./linux/README.md)
- [Linux 工具](./linux/linux工具.md)
- [Linux 常用命令](./linux/linux常用命令.md)
- [Shell 笔记](./shell笔记.md)
- [Linux 综合笔记](./linux/Linux笔记.md)
- [Linux 常见问题](./linux/issues.md)
- [Ubuntu](./linux/ubuntu.md)
- [Deepin](./linux/deepin.md)
- [Manjaro 笔记](./linux/manjaro笔记.md)
- [KDE](./linux/kde.md)
- [OpenWrt 总览](./openwrt/README.md)
- [OpenWrt / LEDE 编译](./openwrt/leda编译.md)
- [Homebrew](./linux/homebrew.md)
- [VirtualBox 使用指南](./virtualbox.md)
- [Wine 笔记](./linux/wine/wine笔记.md)
- [Raspberry Pi 笔记](./raspberry笔记.md)
- [Windows 总览](./windows/README.md)
- [Windows 笔记](./windows/windows.md)
- [Windows 工具](./windows/windows工具.md)
- [WSL](./windows/wsl.md)
- [Windows 11 安卓子系统](./android/android.md)
- [MSYS2 笔记](./msys2.md)
- [Chocolatey](./windows/chocolatey.md)
- [vcpkg](./windows/vcpkg.md)
- [NSSM](./windows/nssm.md)
- [Windows 安装 OpenSSH Server](./windows/安装openssh%20server.md)
- [查看 dll 或 lib 文件函数定义](./windows/dll%20lib%20文件接口查询.md)

## 容器与服务

- [Docker 总览](./docker/README.md)
- [Docker](./docker/docker笔记.md)
- [Dockerfile](./docker/Dockerfile用法.md)
- [docker-compose](./docker/docker-compose用法.md)
- [部署自动化环境](./docker/部署自动化环境.md)
- [GitLab 与 Taiga 部署记录](./docker/gitlab/Readme.md)
- [Nginx 笔记](./nginx笔记.md)
- [OpenResty](./openresty.md)
- [iceoryx](./iceoryx/iceoryx.md)
- [阿里云服务](./aliyun服务.md)
- [VPS 运维笔记](./vps.md)

## 数据与 AI

- [SQL 总览](./sql/README.md)
- [AI 工具总览](./ai/README.md)
- [Claude](./ai/claude.md)
- [大模型工作流](./ai/大模型工作流.md)
- [AutoGen Studio + Ollama 本地部署](./ai/AutoGen_Ollama_本地部署.md)
- [RAGFlow](./ai/ragflow.md)
- [Chandra OCR](./ai/chandra_ocr.md)
- [MySQL](./sql/mysql.md)
- [PostgreSQL](./sql/PostgreSQL使用.md)
- [MongoDB](./mongodb.md)

## 网络与远程协作

- [SSH](./ssh.md)
- [Wireshark](./wireshark.md)
- [RustDesk](./rustdesk.md)
- [Clash](./clash.md)
- [Squid 用法](./squid/说明.md)

## 其他常用

- [历史工具清单](./tools.md)

- [uv](./uv.md)
- [技术线索](./technology.md)

## 目录边界

- `tools/` 主要放工具使用, 环境搭建, 运维和平台操作记录.
- `frontend/`, `program/`, `embedded/` 中若出现与具体技术栈强相关的内容, 优先留在各自专题目录中.
- 若某页只是旧路径说明或历史兼容入口, 后续会继续收缩, 避免重复导航.

# Mesa Windows 构建记录

## 说明

- 本页记录在 Windows 上构建 `Mesa` 的一次实验流程.
- 当前内容更偏“软件栈准备 + Meson/Ninja 构建链路”速记.
- 因为文件位于 Rust 目录下, 这里可把它理解为图形栈与桌面图形相关依赖的周边记录.

## Python 与构建环境

通过 `uv` 安装 Python:

```sh
uv python install 3.13
uv add fastapi --default-index https://mirrors.aliyun.com/pypi/simple/
```

可通过环境变量控制镜像源:

- `UV_DEFAULT_INDEX`: Python 包索引.
- `UV_PYTHON_INSTALL_MIRROR`: Python 安装镜像, 例如 `https://mirror.nju.edu.cn/github-release/indygreg/python-build-standalone/`.

## 额外工具

### winflexbison

- 下载: https://github.com/lexxmark/winflexbison/releases
- 解压后把 `win_bison.exe`, `win_flex.exe` 重命名为 `bison.exe`, `flex.exe`.
- 再把它们加入环境变量.

### rc.exe

将下面路径加入 `PATH`:

```text
C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64
```

## 准备构建目录

```powershell
mkdir mesa_builds
cd mesa_builds
uv venv --python 3.13
.\.venv\Scripts\activate
uv pip install meson ninja packaging mako cmake pyyaml
```

## 下载源码

```sh
git clone --branch mesa-24.3.4 --depth=1 https://gitlab.freedesktop.org/mesa/mesa.git mesa-24.3
git clone --branch mesa-25.1.7 --depth=1 https://gitlab.freedesktop.org/mesa/mesa.git mesa-25.1.7
```

## Meson 配置与构建

```sh
cd mesa-24.3

uv run meson setup builddir --buildtype=release -Dplatforms=windows -Dgallium-drivers="softpipe,llvmpipe" -Dvulkan-drivers= -Dllvm=enabled -Degl=enabled -Dgallium-opencl=disabled -Dshared-glapi=enabled -Dprefix=E:\builds\mesa_builds\install

uv run meson setup builddir
uv run meson setup builddir --backend=vs
uv run meson configure --backend=vs
uv run ninja -C builddir -j22
uv run ninja -C builddir install
```

## 当时记录到的配置结果

- `Platform: windows`
- `OpenGL: YES`
- `LLVM: YES`
- `Gallium Drivers: llvmpipe softpipe zink d3d12`
- `Vulkan Drivers: NO`

## 使用建议

- 真正开始构建前, 先把 `Python`, `Meson`, `Ninja`, `Bison/Flex`, `rc.exe` 这些前置项一次配好.
- 如果只是为了桌面图形栈实验, 通常先跑通 `llvmpipe` / `softpipe` 会更稳.
- 切版本时, 建议固定 Mesa tag, LLVM 版本和 Meson 参数, 避免排查成本失控.

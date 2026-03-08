# Wine 笔记

## 说明

- 本页记录 `Wine` 在 `LinuxMint` / `Deepin` 场景下的安装和中文字体配置.
- 适合作为“先把 Windows 程序跑起来, 再继续处理字体和兼容问题”的速记页.

## 参考

- Arch Wiki: <https://wiki.archlinux.org/index.php/Wine_(%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87)>

## LinuxMint 19 安装 Wine

```sh
sudo dpkg --add-architecture i386
wget -nc https://dl.winehq.org/wine-builds/winehq.key
sudo apt-key add winehq.key
sudo add-apt-repository 'deb https://dl.winehq.org/wine-builds/ubuntu/ focal main'

sudo apt update
sudo apt install --install-recommends winehq-devel
```

初始化 Wine 环境:

```sh
winecfg
```

- 初次启动时会提示安装 `mono` 与 `gecko`, 按向导完成即可.

## 中文字体配置

```sh
sudo apt install fonts-wqy-microhei
cp /usr/share/fonts/truetype/wqy/wqy-microhei.ttc ~/.wine/drive_c/windows/Fonts
```

Deepin Wine 兼容路径记录:

```sh
cp /usr/share/fonts/truetype/wqy/wqy-microhei.ttc ~/.deepinwine/Deepin-WeChat/drive_c/windows/Fonts
cp /usr/share/fonts/truetype/wqy/wqy-microhei.ttc ~/.deepinwine/Deepin-QQ/drive_c/windows/Fonts
```

## 注册表字体映射

`wqy-microhei.reg` 内容:

```reg
REGEDIT4

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink]
"Lucida Sans Unicode"="wqy-microhei.ttc"
"Microsoft Sans Serif"="wqy-microhei.ttc"
"Microsoft YaHei"="wqy-microhei.ttc"
"微软雅黑"="wqy-microhei.ttc"
"MS Sans Serif"="wqy-microhei.ttc"
"Tahoma"="wqy-microhei.ttc"
"Tahoma Bold"="wqy-microhei.ttc"
"SimSun"="wqy-microhei.ttc"
"Arial"="wqy-microhei.ttc"
"Arial Black"="wqy-microhei.ttc"
"宋体"="wqy-microhei.ttc"
"新細明體"="wqy-microhei.ttc"
```

导入注册表:

```sh
wine regedit wqy-microhei.reg
LC_ALL=zh_CN.UTF-8 wine regedit wqy-microhei.reg
```

Deepin Wine 记录:

```sh
deepin-wine6-stable regedit wqy-microhei.reg
```

## Deepin Wine 启动命令

编辑 `/opt/deepinwine/tools/run.sh`, 调整为:

```sh
WINE_CMD="LC_ALL=zh_CN.UTF-8 deepin-wine"
```

## 使用建议

- 先完成基础安装和 `winecfg`, 再处理字体和语言环境.
- 如果程序能启动但中文乱码, 优先检查字体文件与注册表映射.
- 如果程序无法运行, 再回到 32 位依赖, 运行库和兼容层版本排查.
- 若后续继续整理, 可补 `winetricks`, 常见运行库安装和图形驱动兼容记录.

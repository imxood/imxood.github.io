# Ubuntu

## 说明

- 本页记录 `Ubuntu` 下较常见的桌面与系统配置项, 当前重点是远程音频和合盖行为调整.
- 适合作为日常开发机, 远程桌面机器或实验机的系统配置速查页.

## ToDesk 音频依赖

```sh
sudo apt install pulseaudio
```

参考:

- <https://github.com/shivasiddharth/PulseAudio-System-Wide>

适用场景:

- 远程桌面连接后没有声音.
- 需要检查系统是否具备最基本的音频服务组件.

## 合盖功能

配置文件:

```text
/etc/systemd/logind.conf
```

示例配置:

```conf
; 使用电池供电时, 合盖挂起
HandleLidSwitch=suspend
; 使用外部电源时, 合盖忽略
HandleLidSwitchExternalPower=ignore
; 使用扩展坞时, 合盖忽略
HandleLidSwitchDocked=ignore
```

## 常见使用场景

- 外接显示器时希望合盖后继续运行.
- 远程开发时希望合盖不影响任务执行.
- 笔记本单独使用时仍希望保留默认挂起行为.

## 使用建议

- 修改 `logind.conf` 后, 记得重启相关服务或直接重启系统再验证行为.
- 做远程开发或外接显示器场景时, 通常需要重点确认“接电源合盖是否继续运行”.
- 远程桌面无声音时, 优先检查 `PulseAudio` 是否存在, 服务状态是否正常, 以及远程工具自身的音频设置.
- 若后续继续整理, 可补软件源, 驱动, Docker, 开发工具链和桌面远程运维常见问题.

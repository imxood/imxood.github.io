
## todesk

sudo apt install pulseaudio

https://github.com/shivasiddharth/PulseAudio-System-Wide

## 合盖功能

/etc/systemd/logind.conf

```conf
; 使用电池供电时, 合盖挂起
HandleLidSwitch=suspend
; 使用外部电源时, 合盖忽略
HandleLidSwitchExternalPower=ignore
; 使用扩展坞时, 合盖忽略
HandleLidSwitchDocked=ignore
```
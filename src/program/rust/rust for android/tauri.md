# Tauri for Android

## 说明

- 记录 `Tauri` 项目接入 Android 的初始化步骤和 Gradle 代理配置.
- 当前内容以 Windows 环境为主, 命令更接近早期 `Tauri v2 alpha` 记录.

## 前置环境

安装 Android Studio, 并设置环境变量:

```powershell
ANDROID_HOME=E:\Android\Sdk
NDK_HOME=%ANDROID_HOME%\ndk\23.2.8568313
PATH=%PATH%;C:\Program Files\Android\Android Studio\jre\bin;%ANDROID_HOME%\platform-tools
```

## 初始化命令

```sh
yarn create tauri-app --alpha
cd <project>
yarn
yarn tauri android init
yarn tauri android dev
```

- 如果下载速度较慢, 最好先配置代理.

## Android 项目代理

### 项目内生效

修改 `src-tauri\gen\android\test_tauri_v2\gradle.properties`, 添加:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=1080
systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=1080
systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=1080
```

### 全局生效

在用户目录下 `.gradle\gradle.properties` 中添加:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=7890
systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=7890
systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=7890
```

## 问题记录

- Windows 10 设备调试相关设置可参考: https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development

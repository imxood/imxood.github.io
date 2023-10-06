
安装 Android Studio.

Windows 设置环境变量:

ANDROID_HOME=E:\Android\Sdk
NDK_HOME=%ANDROID_HOME%\ndk\23.2.8568313

PATH=%PATH%;C:\Program Files\Android\Android Studio\jre\bin;%ANDROID_HOME%\platform-tools

## 命令

yarn create tauri-app --alpha

进入项目路径后:

yarn

yarn tauri android init

yarn tauri android dev (最好先设置代理, 不然速度特别慢)

## 设置 android项目 代理

### 项目内生效

修改 src-tauri\gen\android\test_tauri_v2\gradle.properties, 添加:

systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=7890

systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=7890

systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=7890

### 全局生效

用户目录下 .gradle文件夹 中 gradle.properties, 或新建, 或打开, 添加:

systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=7890

systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=7890

systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=7890

## 编译问题

windows 10, See https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development




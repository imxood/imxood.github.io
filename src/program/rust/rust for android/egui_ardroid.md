1. 下载 android studio

    使用 android studio 下载 SDK / NDK

2. 设置环境变量

```sh
ANDROID_SDK_ROOT=D:\programs\Android\Sdk
ANDROID_NDK_ROOT=%ANDROID_SDK_ROOT%\ndk\23.1.7779620
```

3. 安装编译工具

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

4. 下载测试代码

```sh
git clone https://github.com/rib/android-activity.git

cd examples/agdk-egui
```

5. 编译

```sh
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build

./gradlew build

```

native 编译

```sh
cargo apk build
cargo apk run

```

# 网络不好的话, 需要配置代理

编辑 gradle.properties 文件, 追加内容:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=1080

systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=1080

systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=1080
```

```sh
./gradlew build
```

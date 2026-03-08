# Java 开发环境

## 说明

- 本页整理本地 `Java` 开发环境搭建的最小步骤.
- 当前重点是 `JDK + Maven` 的安装, 验证和镜像配置.

## JDK 下载

- Adoptium Temurin: <https://adoptium.net/temurin/releases>
- Microsoft Build of OpenJDK: <https://learn.microsoft.com/zh-cn/java/openjdk/download>

选择建议:

- 只做通用开发时, 先选 `LTS` 版本.
- 团队项目要优先跟随项目已锁定的 `JDK` 主版本.

## 环境变量

常见配置项:

- `JAVA_HOME`: 指向 JDK 根目录.
- `PATH`: 需要包含 `JAVA_HOME/bin`.
- `JAVA_TOOL_OPTIONS=-Dfile.encoding=UTF-8`: 可减少默认编码不一致问题.

## 验证命令

```sh
java -version
javac -version
```

如果以上命令不可用, 优先检查 `JAVA_HOME` 和 `PATH`.

## Maven

- 官网: <https://maven.apache.org/>
- 安装后需把 `bin` 目录加入 `PATH`.

示例路径:

```text
E:/programs/apache-maven-3.9.2/bin
```

验证命令:

```sh
mvn -v
```

## Maven 镜像配置

修改 `settings.xml`:

```xml
<mirror>
    <id>aliyunmaven</id>
    <mirrorOf>*</mirrorOf>
    <name>阿里云公共仓库</name>
    <url>https://maven.aliyun.com/repository/public</url>
</mirror>
```

适用场景:

- 国内网络环境下提升依赖下载速度.
- 新机器初始化时减少首次拉取依赖的失败率.

## 最小项目验证

```sh
mvn archetype:generate
mvn package
```

建议至少完成一次“创建项目 -> 编译 -> 打包”闭环验证.

## 常见问题

- `java` 与 `javac` 版本不一致: 说明环境变量可能指向了不同安装目录.
- 中文乱码: 优先检查项目编码, 终端编码和 `JAVA_TOOL_OPTIONS`.
- `mvn` 无法下载依赖: 先确认网络, 再检查镜像源和 `settings.xml` 位置.

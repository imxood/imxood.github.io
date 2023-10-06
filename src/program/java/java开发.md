## jdk 下载

https://adoptium.net/temurin/releases

https://learn.microsoft.com/zh-cn/java/openjdk/download

## maven

下载地址: https://maven.apache.org/

添加路径到环境变量PATH中:

E:\programs\apache-maven-3.9.2\bin

## 在maven中添加aliyun源

    修改 "E:\programs\apache-maven-3.9.2\conf\settings.xml"

    <mirror>
        <id>aliyunmaven</id>
        <mirrorOf>*</mirrorOf>
        <name>阿里云公共仓库</name>
        <url>https://maven.aliyun.com/repository/public</url>
    </mirror>


## 设置 jdk默认编码 为 UTF-8

https://www.cnblogs.com/LinKinSJ/p/9096921.html

添加环境变量:

变量名: JAVA_TOOL_OPTIONS

变量值：-Dfile.encoding=UTF-8
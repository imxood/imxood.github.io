# 使用技巧

## 正则替换

aabbccdd

正则搜索: ([A-Za-z0-9]{2})
替换为: \x$1

结果为: \xaa\xbb\xcc\xdd

## install vscode

curl https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg

sudo install -o root -g root -m 644 packages.microsoft.gpg /usr/share/keyrings/

sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/vscode stable main" > /etc/apt/sources.list.d/vscode.list'

sudo apt update && sudo apt install -y code-insiders

## 安装 remote develop 插件

Linux 下, 避免多次输入密码:

    ~/.ssh/config

    Host *
    ControlMaster auto
    ControlPath  ~/.ssh/%r@%h-%p
    ControlPersist  600

Windows 下, 拷贝公钥到 ssh server 中的 authorized_keys 文件

如果 Remote Host 有代理, 如果 wget 连接失败, 考虑下面的方法

    ~/.wgetrc

    use_proxy=on
    http_proxy=http://myproxy.proxy:xxxx
    https_proxy=http://myproxy.proxy:xxxx

## is unable to watch for file changes in this large workspace

sudo sh -c "echo 'fs.inotify.max_user_watches=524288' >> /etc/sysctl.conf"
sudo sysctl -p

## 无法加载文件....ps1, 因为在此系统上禁止运行脚本

    以管理员身份运行vscode
    get-ExecutionPolicy, 显示Restricted, 表示状态是禁止的
    set-ExecutionPolicy RemoteSigned
    get-ExecutionPolicy, 就显示RemoteSigned

## vscode 中 python multiprocessing

    import multiprocessing
    multiprocessing.set_start_method('spawn', True)

## 插件开发

### 安装环境

    sudo apt install -y node npm
    sudo npm install -g cnpm --registry=https://registry.npm.taobao.org
    sudo cnpm install -g n
    sudo n latest
    sudo cnpm install -g yarn yo generator-code vsce

    yarn config set registry https://registry.npm.taobao.org

### 开始

    yo code, 使用TypeScript, Yarn, 生成一个vscode插件的初始化环境

    cd ProjectName
    yarn install

    f5
    vsce package
    code --install-extension ***.vsix

    npm run COMMAND, 运行package.json中的scripts命令

## 自动设置编码

    "files.autoGuessEncoding": true

## 正则替换

    https://www.jianshu.com/p/7935fdcb17d0

    把 print 'abc' 替换为 print('abc')
    搜索: print (.*)
    替换: print($1)

    \, 单斜杠为转义
    (), 括号为模式匹配

## 调试时 16 进制 显示

    在 watch 窗口， 右键表达式 -> 编辑表达式: 表达式，h

##

vscode 导出插件列表:

    code --list-extensions > vscode-extensions.txt

VSCodium 安装插件列表:

    Get-Content vscode-extensions.txt | ForEach-Object { codium --install-extension $_ }

# git笔记

## 第一次提交代码

    git init

    git remote add origin git@gitee.com:imxood/stm32h750_rt_app.git
    git pull origin master

    git branch --set-upstream-to=origin/master

    git add .
    git commit -m "."
    git push


## warning: CRLF will be replaced by LF

    git config --global core.autocrlf false


## 执行 git status 时 中文路径乱码解决:

    git config --global core.quotepath false

## 创建空的新分支

    git checkout --orphan main, 创建没有commits的孤儿分支
    git add . && git commit -m "."
    git push --set-upstream origin main

## git remote

    git remote -v, 显示当前远程仓库信息
    git remote set-url origin https://github.com/imxood/mdbook-katex.git, 设置远程仓库


## workflow

参考: [GitHub Actions 的元数据语法](https://docs.github.com/cn/actions/creating-actions/metadata-syntax-for-github-actions)

参考: [GitHub Actions 的工作流程语法](https://docs.github.com/cn/actions/reference/workflow-syntax-for-github-actions)


## github reset api

参考: [getting-started-with-the-rest-api](https://docs.github.com/cn/rest/guides/getting-started-with-the-rest-api)

参考: [Resources in the REST API](https://docs.github.com/en/rest/overview/resources-in-the-rest-api)

    不使用认证将只有每小时60次请求的限制
        curl -i https://api.github.com/users/imxood

    使用认证将有每小时5000次请求
        curl -i -u username:$token https://api.github.com/users/octocat

    Shell - Get latest release from GitHub
        curl --silent "https://api.github.com/repos/$1/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")'

## github actions

### 环境文件

    \$GITHUB_PATH, 用于添加系统路径

        GITHUB_PATH 这个环境变量表示一个环境变量文件, 所有后续操作将会将这个文件中的值添加到系统Path中.

        echo "{path}" >> \$GITHUB_PATH
        或者是 打开 \$GITHUB_PATH 文件, 写路径

    \$GITHUB_ENV, 用于设置环境变量

        跟上面是类似的.
        echo "{name}={value}" >> \$GITHUB_ENV

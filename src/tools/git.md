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

## 创建新分支并删除历史commits

    git checkout --orphan main, 创建没有commits的孤儿分支
    git add . && git commit -m "."
    git push --set-upstream origin main


## workflow

[GitHub Actions 的元数据语法](https://docs.github.com/cn/actions/creating-actions/metadata-syntax-for-github-actions)

[GitHub Actions 的工作流程语法](https://docs.github.com/cn/actions/reference/workflow-syntax-for-github-actions)


## github reset api

[getting-started-with-the-rest-api](https://docs.github.com/cn/rest/guides/getting-started-with-the-rest-api)

[Resources in the REST API](https://docs.github.com/en/rest/overview/resources-in-the-rest-api)

    不使用认证将只有每小时60次请求的限制
        curl -i https://api.github.com/users/imxood

    使用认证将有每小时5000次请求
        curl -i -u username:$token https://api.github.com/users/octocat

    Shell - Get latest release from GitHub
        curl --silent "https://api.github.com/repos/$1/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")'

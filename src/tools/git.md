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



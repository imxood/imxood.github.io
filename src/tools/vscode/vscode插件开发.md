##

npm config set registry https://registry.npmmirror.com

npm install -g yo generator-code

yo code

## 插件发布

参考: https://juejin.cn/post/7076649162653040647

yarn global add vsce

### 权限

(1) 申请令牌, 参考:

https://learn.microsoft.com/zh-cn/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate?view=azure-devops&tabs=Windows

在 Azure DevOps Services 中 添加一个 权限

(2) 创建一个发布人

(3) `vsce login <publisher name>`, 并输入 token

(4) vsce publish

### 问题

插件发布后 再安装后 无法激活插件, 打开 开发者模式, 看到 某些模块未安装.

将这些模块从 dev 依赖 移动到 正常依赖.

改一下小版本号, 重新发布就 ok 了

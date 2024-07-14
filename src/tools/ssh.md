## 复制公钥到 远程服务器上

ssh-copy-id USERAME@HOSTNAME


## 错误 ssh Permission denied (publickey)

https://www.cnblogs.com/lsgxeva/p/16851306.html

### 可能的原因 之一是: 默认关闭密码登录.

修改 /etc/ssh/sshd_config 文件:

PasswordAuthentication yes

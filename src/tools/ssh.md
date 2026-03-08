# SSH

## 说明

- 本页整理 `SSH` 的常见登录, 公钥投递和权限排错方法.
- 适合新机器初始化和远程连接失败时快速排查.

## 常用命令

### 复制公钥到远程服务器

```sh
ssh-copy-id USERNAME@HOSTNAME
```

### 直接登录

```sh
ssh USERNAME@HOSTNAME
```

## 首次初始化建议

1. 先确认服务器可通过密码或控制台方式进入.
2. 再使用 `ssh-copy-id` 投递公钥.
3. 验证公钥登录成功后, 再考虑关闭密码登录.

## `Permission denied (publickey)`

参考: <https://www.cnblogs.com/lsgxeva/p/16851306.html>

### 一个常见原因

- 服务器默认关闭了密码登录, 导致初始化阶段无法完成公钥投递.

可检查 `/etc/ssh/sshd_config` 中是否包含:

```conf
PasswordAuthentication yes
```

### 其他常见原因

- 用户家目录或 `~/.ssh` 权限不正确.
- `authorized_keys` 内容未正确写入目标用户.
- 登录用户名错误, 实际写入到了别的账号目录.

权限可参考:

```sh
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys
```

## 使用建议

- 优先使用公钥登录, 再按需关闭密码登录.
- 若公钥已配置仍失败, 再检查用户目录权限, `~/.ssh` 权限和 `authorized_keys` 内容.
- 多台服务器时, 建议配合 `~/.ssh/config` 做主机别名管理.

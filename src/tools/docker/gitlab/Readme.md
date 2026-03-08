# GitLab 与 Taiga 部署记录

## 说明

- 本目录保留了一份历史 `docker-compose.yml`, 用于同时部署 `GitLab`, `Taiga` 和 `PostgreSQL`.
- 内容更适合视为旧部署样例, 不能直接视为当前最佳实践.

## 当前 compose 涉及的服务

- `gitlab`: 使用 `gitlab/gitlab-ce:latest`.
- `taiga`: 使用 `benhutchins/taiga`.
- `postgres`: 为 `Taiga` 提供数据库.

## 适合如何使用

- 回看历史多服务 compose 的组织方式.
- 对照理解早期自建协作平台的部署思路.
- 在真正重建环境前, 先提取其中仍有参考价值的端口, 卷和依赖关系设计.

## 使用前重点检查

- `external_url`, SMTP, 端口映射和卷挂载路径都需要按当前环境重新核对.
- 原文件中保留了多段注释和备选配置, 使用前建议先做精简.
- 示例中的密码和主机名仅用于历史记录, 实际部署必须替换.
- 若用于公网部署, 还需要额外补反向代理, TLS, 备份和升级策略.

## 更合理的整理方式

1. 把 `GitLab` 和 `Taiga` 拆成各自独立的部署文档.
2. 单独整理数据库与备份策略.
3. 再把反向代理, 邮件和公网访问规则抽成公共章节.
4. 最后再保留本页作为“历史组合部署样例”.

## Redmine 插件记录

主题:

- <https://github.com/Nitrino/flatly_light_redmine>

代码评审插件:

- <https://github.com/haru/redmine_code_review>

常用命令:

```sh
bundle install
bundle exec rake redmine:plugins:migrate RAILS_ENV=production
```

## 整理建议

- 若后续继续治理 Docker 目录, 可把本页进一步压缩成“历史样例说明页”, 具体命令迁入更明确的专题页.

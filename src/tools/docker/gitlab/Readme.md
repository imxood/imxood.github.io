# 自动环境配置

## redmine插件

主题: https://github.com/Nitrino/flatly_light_redmine

review插件: https://github.com/haru/redmine_code_review

bundle install

bundle exec rake redmine:plugins:migrate RAILS_ENV=production

# 说明

[dramatiq项目](https://github.com/Bogdanp/dramatiq)


### 安装redis
sudo apt install redis

### 安装
pip install --user dramatiq[redis,watch]


### redis 配置为外网可访问

vim /etc/redis/redis.conf

注释掉这行: bind 127.0.0.1

重启: sudo systemctl restart redis.service



# Nginx 笔记

## 说明

- 本页记录 Nginx 的基础文件服务配置示例.
- 适合作为静态目录暴露与自动索引配置的快速参考.

## 一个简单的文件服务

    在conf.d目录中,新建一个.conf文件.
    
    autoindex on;# 显示目录
    autoindex_exact_size on;# 显示文件大小
    autoindex_localtime on;# 显示文件时间

    server {
        listen       8080 default_server;
        listen       [::]:8080 default_server;
        server_name  _;
        #root         /usr/share/nginx/html;
        root         /data/;
        location / {
        }
        error_page 404 /404.html;
            location = /40x.html {
        }
        error_page 500 502 503 504 /50x.html;
            location = /50x.html {
        }
    }

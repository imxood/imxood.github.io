#/bin/bash

# -t
# 查看压缩包的内容
tar -tvf File.tar

# -x
# 后面跟随指定文件, 解压指定文件
tar -xvf File.tar 8.0/aaa.sh

# --strip-components=1 忽略1级目录, 解压子目录的内容到当前路径
tar -xvf File.tar --strip-components=1

# -C Directory, 改变到指定目录, 即解压到指定目录
tar -xvf File.tar --strip-components=1 -C

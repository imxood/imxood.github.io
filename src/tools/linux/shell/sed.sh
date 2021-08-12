#!/bin/bash

FILENAME=$(cd `dirname $0`;pwd)/test.txt

echo "FILENAME: $FILENAME"

if [ ! -f $FILENAME ]; then
cat > $FILENAME << eof
1
2
3
原字符串
5
6
7
原字符串
eof
fi

ls $FILENAME

# sed替换的基本语法为:
sed 's/2/3/' $FILENAME

# 要处理的字符包含单引号
sed "s/原字符串包含'/替换字符串包含'/" $FILENAME

# 注意这里的'&'符号, 如果没有'&', 就会直接将匹配到的字符串替换掉
# 结尾加'g',则替换所有的,否则只替换第一个
sed 's/^/添加的头部&/g' $FILENAME				#在所有行首添加
sed 's/$/&添加的尾部/g' $FILENAME				#在所有行末添加
sed '2s/原字符串/替换字符串/g' $FILENAME		#替换第2行
sed '$s/原字符串/替换字符串/g' $FILENAME		#替换最后一行
sed '2,6s/原字符串/替换字符串/g' $FILENAME		#替换2到5行
sed '2,$s/原字符串/替换字符串/g' $FILENAME		#替换2到最后一行

# 同时执行两个替换规则
sed 's/^/添加的头部&/g；s/$/&添加的尾部/g'

# 将分隔符换成问号”@”:
# sed处理过的输出是直接输出到屏幕上的, 使用参数”i”直接在文件中替换
# sed -i 's@原字符串@替换字符串@g' $FILENAME

rm -f $FILENAME

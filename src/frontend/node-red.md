## 源码编译

git clone https://github.com/node-red/node-red.git
cd node-red

<!-- 使用 yarn 进行 install 会失败 -->

npm --registry=https://registry.npm.taobao.org install

npm run build

npm start

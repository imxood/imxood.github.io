# Clash

## 说明

- 本页记录 `Clash Verge` 中通过脚本动态追加规则的方式.
- 适合需要为少量域名单独指定直连或代理策略的场景.

## Clash Verge 自定义规则脚本

打开配置目录后, 修改 `profiles\Script.js`:

```js
/**
 * 配置中的规则 `config.rules` 是一个数组, 通过新旧数组合并来添加.
 * @param prependRule 添加的数组
 */
const prependRule = [
  // 配置为直连
  "DOMAIN,auto.startravel.top,DIRECT",

  // 配置为国外流量
  "DOMAIN-SUFFIX,trae.ai,🔰国外流量",
];

function main(config) {
  let oldrules = config["rules"];
  config["rules"] = prependRule.concat(oldrules);
  return config;
}
```

## 这个脚本做了什么

- 在原有规则数组前面插入自定义规则.
- 让命中的域名优先走你手工追加的策略.
- 常见用法包括指定域名直连, 代理或分流到特定策略组.

## 使用建议

- 自定义规则通常应放在原规则前面, 才能确保优先匹配.
- 修改后注意在客户端里重新加载配置.
- 如果规则越来越多, 更建议按域名分类整理, 避免脚本文件过于混乱.

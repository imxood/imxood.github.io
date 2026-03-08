# 前端工具库

## 说明

- 本页整理前端项目里高频使用的通用工具库.
- 当前重点覆盖日期处理, 数据加工和模拟数据生成三类常见需求.

## 日期处理

### moment.js

- 官网: <http://momentjs.cn/>

```ts
yarn add moment
import * as moment from 'moment'

moment('20230319', 'YYYYMMDD')
moment().format('YYYYMMDD')
```

适用场景:

- 快速做日期解析和格式化.
- 维护旧项目时兼容已有时间处理逻辑.

## 数据加工

### lodash

- 官网: <https://www.lodashjs.com/>

```ts
yarn add -D @types/lodash
import * as _ from 'lodash'

_.debounce(fn, 300)
_.cloneDeep(data)
_.get(user, 'profile.name')
```

适用场景:

- 深拷贝, 防抖节流, 路径读取和集合转换.
- 处理老代码时统一常见工具函数风格.

## 模拟数据

### faker

- 官网: <https://fakerjs.dev/api/>

```ts
yarn add -D @faker-js/faker
import { faker } from '@faker-js/faker'

const item = {
  id: faker.string.uuid(),
  name: faker.person.fullName(),
}
```

适用场景:

- 开发阶段快速生成表格, 表单和接口模拟数据.
- 联调前搭本地假数据环境.

## 选型建议

- 新项目应优先考虑体积, Tree Shaking 和替代方案的维护状态.
- 若只是简单日期格式化, 可评估更轻量方案.
- 若团队已有统一工具封装, 尽量优先复用团队内部工具层.

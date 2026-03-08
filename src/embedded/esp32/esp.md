# ESP-IDF 常用命令

## 说明

- 本文记录 `ESP-IDF` 开发中常用的构建, 清理和信息查看命令.
- 更偏板级和具体开发板问题, 可参考 [nanoESP32-C3](./nanoESP32-C3.md).

## 常用命令

### 查看组件体积

```sh
idf.py size-components
```

### 重新生成配置与构建文件

```sh
idf.py reconfigure
```

### 常见配套命令

```sh
idf.py set-target esp32c3
idf.py menuconfig
idf.py build
idf.py flash
idf.py monitor
idf.py fullclean
```

## 使用建议

- 目标芯片切换前, 先执行 `set-target`, 避免残留旧配置.
- 遇到配置缓存异常时, 可尝试 `reconfigure` 或 `fullclean`.
- 关注 `size-components` 输出, 便于定位占空间较大的组件.

## 相关文档

- [nanoESP32-C3](./nanoESP32-C3.md)
- [nanoESP32-C3 开发板](./esp32c3.md)
- [PlatformIO Arduino 环境](../platformio.md)

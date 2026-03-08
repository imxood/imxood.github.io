# serde_json

## 说明

- `serde_json` 是 Rust 里最常见的 JSON 序列化与反序列化库之一.
- 本页当前记录“带标签枚举风格”的结构化消息示例.

## 基本示例

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "req")]
struct DefineSerial {
    serial: String,
    port: String,
    baudrate: String,
    databits: String,
    flowcontrol: String,
    parity: String,
    stopbits: String,
}

let v = json!({
    "req": "DefineSerial",
    "serial": "DeviceName00",
    "port": "COM0",
    "baudrate": "2000000",
    "databits": "",
    "flowcontrol": "",
    "parity": "",
    "stopbits": "",
});
```

## 这个例子说明了什么

- `#[serde(tag = "req")]` 适合把结构体或枚举消息编码成带类型标签的 JSON.
- `json!(...)` 适合快速构造测试数据或临时消息体.
- 这种模式常用于 IPC, WebSocket, 串口协议桥接和前后端消息交互.

## 使用建议

- 若消息类型较多, 更推荐用枚举统一表达协议而不是散落多个结构体.
- 若字段需要兼容老协议, 可继续配合 `rename`, `default`, `skip_serializing_if` 等属性使用.

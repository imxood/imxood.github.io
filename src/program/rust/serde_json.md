##

```rs

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

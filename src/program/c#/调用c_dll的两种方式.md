# 调用 C DLL 的两种方式

## 说明

- 本页整理 `C#` 调用原生 `C DLL` 时最常见的两类方式.
- 一个偏静态声明, 一个偏动态加载, 适合根据项目形态做取舍.

## 方式一: `DllImport`

- 适合函数签名稳定, 调用关系明确的场景.
- 由运行时自动完成符号绑定, 写法最直接.

```csharp
[DllImport("demo.dll", CallingConvention = CallingConvention.Cdecl)]
public static extern int Add(int a, int b);
```

优点:

- 声明简单.
- 可读性高.
- 适合固定接口和少量导出函数.

## 方式二: 动态加载函数指针

- 适合插件化, 可选加载或运行时决定 DLL 路径的场景.
- 常见做法是 `LoadLibrary` + `GetProcAddress`, 再转换为委托.

典型思路:

1. 动态加载 DLL.
2. 获取目标导出函数地址.
3. 通过 `Marshal.GetDelegateForFunctionPointer` 转为委托.
4. 调用委托.

## 选择建议

- 接口固定且数量不多时, 优先用 `DllImport`.
- 需要运行时切换版本, 动态探测能力或做插件系统时, 再考虑函数指针方式.
- 如果目标是跨语言长期维护, 要尽量把导出接口做得简单稳定.

## 联调关注点

- `CallingConvention` 必须和原生侧一致.
- 字符串编码, 结构体布局和内存释放责任要提前约定.
- `x86` / `x64` 架构不一致时, 往往在加载阶段就会失败.
- 复杂对象尽量不要直接跨边界传递, 优先用基本类型或序列化数据.

## 相关文档

- [C# 生成动态库](./生成动态库.md)
- [C# 笔记](./笔记.md)

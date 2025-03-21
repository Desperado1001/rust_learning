# Rust 示例代码

这个目录包含各种 Rust 特性和功能的示例代码。这些示例展示了如何在实际场景中使用 Rust 的各种功能。

## 示例列表

- **并发编程**：线程、互斥锁、通道等
- **错误处理**：Result、Option、panic 等
- **文件操作**：读写文件、目录操作等
- **网络编程**：TCP/IP、HTTP 客户端和服务器等
- **数据序列化/反序列化**：使用 serde 处理 JSON、YAML 等
- **数据库操作**：连接和查询各种数据库
- **异步编程**：使用 async/await、Future 等

## 如何运行示例

大多数示例都可以使用 `rustc` 单独编译和运行：

```bash
rustc example_file.rs
./example_file
```

一些依赖外部库的示例需要在 Cargo 项目中运行，会有特定的说明。
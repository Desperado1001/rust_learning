# Rust 示例程序

这个目录包含完整的 Rust 示例程序，展示如何将 Rust 的各种特性应用到实际项目中。

## 示例列表

1. **简易计算器** (`01_simple_calculator.rs`)
   - 一个命令行计算器应用程序
   - 展示用户输入处理、错误处理和控制流程概念
   - 使用 `match`、`loop` 和函数组织代码
   - 演示 `Result` 类型进行错误处理

## 运行示例

你可以使用 `rustc` 直接编译和运行这些示例：

```bash
# 编译
rustc examples/01_simple_calculator.rs

# 运行
./01_simple_calculator
```

或者，你可以使用 Cargo 运行它们。在项目根目录的 `Cargo.toml` 文件中添加：

```toml
[[bin]]
name = "calculator"
path = "examples/01_simple_calculator.rs"
```

然后运行：

```bash
cargo run --bin calculator
```

## 学习目标

这些示例旨在帮助你：

- 了解如何将 Rust 的概念应用到实际问题中
- 学习常见的代码组织和项目结构模式
- 练习错误处理和用户输入处理
- 见识不同 Rust 功能的实际使用场景

每个示例都有详细的注释，解释代码的功能和使用的 Rust 特性。建议先阅读基础部分的内容，再研究这些示例。

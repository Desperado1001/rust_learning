# Rust 项目练习

这个目录包含使用 Rust 实现的各种小型项目。通过实际项目来应用 Rust 的概念和特性，是提高 Rust 编程能力的最佳方式。

## 项目列表

1. **命令行计算器**：简单的命令行计算器应用
2. **文件统计工具**：统计文件字数、行数的工具
3. **HTTP 客户端**：使用 Rust 发起 HTTP 请求
4. **简单 Web 服务器**：使用 Rust 实现基本的 HTTP 服务器
5. **数据结构实现**：使用 Rust 实现常见数据结构（链表、二叉树等）

## 如何运行

每个项目都是一个独立的 Cargo 项目。进入项目目录，然后使用以下命令运行：

```bash
cargo run
```

## 项目创建步骤

使用以下步骤创建新项目：

```bash
# 创建新的 Cargo 项目
cargo new project_name

# 添加依赖到 Cargo.toml
# [dependencies]
# reqwest = { version = "0.11", features = ["json"] }
# tokio = { version = "1", features = ["full"] }

# 构建并运行
cd project_name
cargo run
```
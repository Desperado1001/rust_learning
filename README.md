# Rust 学习笔记

这个仓库记录了我学习 Rust 编程语言的旅程。Rust 是一门系统级编程语言，专注于安全性、并发和性能。

## 目录结构

- `basics/`: Rust 基础语法和概念
- `projects/`: 小型 Rust 项目和练习
- `examples/`: 各种 Rust 功能的示例代码
- `notes/`: 学习笔记和重要概念

## 学习资源

- [Rust 官方文档](https://www.rust-lang.org/zh-CN/learn)
- [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/) (中文版)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## 安装 Rust

```bash
# 使用 rustup 安装 Rust（推荐方式）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 检查安装版本
rustc --version
cargo --version
```

## 基本命令

```bash
# 创建新项目
cargo new project_name

# 构建项目
cargo build

# 运行项目
cargo run

# 构建发布版本
cargo build --release

# 检查代码但不生成可执行文件
cargo check

# 运行测试
cargo test
```

## 学习进度

- [x] 基础语法和概念
  - [x] Hello World 和打印
  - [x] 变量和基本数据类型
  - [x] 控制流（if/else, 循环, match）
  - [x] 函数和方法
- [ ] 所有权和借用系统
- [ ] 结构体和枚举
- [ ] 错误处理
- [ ] 泛型和特性
- [ ] 生命周期
- [ ] 测试
- [ ] 并发编程
- [ ] 智能指针
- [ ] 宏和元编程

## 小项目计划

1. [ ] 命令行工具
2. [ ] 文件处理程序
3. [ ] Web API 服务器
4. [ ] 数据结构实现
5. [ ] 小型游戏

## 最近更新

- 添加了函数和方法章节，详细介绍了 Rust 的函数定义、参数传递和方法实现
- 实现了一个学生管理系统示例，展示了函数、方法和结构体的综合应用
- 添加了详细的函数和方法笔记，解释了 Rust 函数系统的特点和使用技巧
- 添加了控制流内容，包括 if/else, 循环和 match 表达式
- 实现了一个简单的命令行计算器示例，展示基本控制流和错误处理
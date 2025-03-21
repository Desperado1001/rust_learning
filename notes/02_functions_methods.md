# Rust 函数和方法

函数是 Rust 程序的基础构建块，用于组织代码、实现特定功能和提高代码复用性。Rust 中的函数具有清晰的语法和严格的类型系统，这有助于编写安全、可靠的代码。

## 函数基础

### 函数定义

Rust 中的函数使用 `fn` 关键字定义：

```rust
fn 函数名(参数1: 类型1, 参数2: 类型2, ...) -> 返回类型 {
    // 函数体
}
```

例如：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 函数命名规范

Rust 社区约定使用蛇形命名法（snake_case）命名函数：

```rust
fn calculate_total_price() {
    // 函数体
}
```

### 参数和返回值

- 参数必须显式声明类型
- 返回值类型使用箭头 `->` 后跟类型来声明
- 如果函数不返回值，可以省略返回类型（相当于返回 `()`，即空元组或"单元"类型）

```rust
// 无参数、无返回值
fn say_hello() {
    println!("Hello!");
}

// 有参数、无返回值
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数、有返回值
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## 函数体和表达式

### 语句与表达式

Rust 是一种基于表达式的语言，区分两种类型的代码结构：

- **语句**：执行操作但不返回值，以分号结束
- **表达式**：求值并返回结果，没有分号

函数体由一系列语句组成，可以以一个表达式结束。这个表达式的值会成为函数的返回值。

```rust
fn increment(x: i32) -> i32 {
    // 这是表达式，没有分号，会作为返回值
    x + 1
}

// 等价于
fn increment_explicit(x: i32) -> i32 {
    return x + 1;
}
```

### 提前返回

可以使用 `return` 关键字在函数任意位置返回：

```rust
fn absolute_value(x: i32) -> i32 {
    if x >= 0 {
        return x;
    }
    -x  // 隐式返回
}
```

### 函数表达式块

可以使用代码块作为表达式：

```rust
let result = {
    let x = 5;
    let y = 10;
    x + y  // 没有分号，返回 x + y 的值
};
// result 现在是 15
```

## 高级函数特性

### 函数作为参数

Rust 支持高阶函数，函数可以接受其他函数作为参数：

```rust
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = apply(double, 5);  // 结果是 10
}
```

### 闭包与函数

Rust 支持闭包（匿名函数），可以捕获环境中的变量：

```rust
fn main() {
    let x = 5;
    
    // 定义闭包
    let add_x = |y| x + y;
    
    let result = add_x(10);  // 结果是 15
}
```

## 方法

方法是与特定类型关联的函数。在 Rust 中，可以为自定义类型（如结构体、枚举）实现方法。

### 方法定义

使用 `impl` 块为类型定义方法：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法定义，self 参数表示调用方法的实例
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### self 参数

方法的第一个参数通常是 `self`，表示调用该方法的实例：

- `&self`：不可变引用，方法不会修改实例
- `&mut self`：可变引用，方法可能修改实例
- `self`：所有权转移，方法消耗实例

```rust
impl Rectangle {
    // 不可变引用
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 可变引用
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    
    // 所有权转移
    fn destroy(self) -> (u32, u32) {
        (self.width, self.height)
    }
}
```

### 关联函数

不接受 `self` 参数的函数称为关联函数（相当于其他语言的静态方法）：

```rust
impl Rectangle {
    // 关联函数，常用作构造器
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 调用关联函数
let rect = Rectangle::new(10, 20);
```

## 函数和方法的最佳实践

1. **职责单一**：每个函数应该只完成一个明确的任务
2. **命名明确**：函数名应清晰描述其功能
3. **适当注释**：对复杂逻辑添加注释
4. **参数合理**：避免过多参数，考虑使用结构体传递多个相关参数
5. **错误处理**：使用 `Result` 类型返回可能失败的操作结果
6. **代码复用**：提取常用逻辑为独立函数
7. **测试覆盖**：为函数编写单元测试

## 总结

函数和方法是 Rust 程序的核心构建块。Rust 的函数系统简洁明了，同时提供了强大的表达能力。理解函数的表达式特性、返回值机制以及方法实现是掌握 Rust 编程的关键步骤之一。
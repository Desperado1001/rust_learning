# Rust 所有权系统

所有权（Ownership）是 Rust 最独特且重要的特性之一，它使 Rust 能够在不需要垃圾回收的情况下保证内存安全。理解所有权系统对于掌握 Rust 至关重要。

## 所有权规则

Rust 的所有权系统遵循三条基本规则：

1. Rust 中的每个值都有一个被称为其**所有者**（owner）的变量。
2. 值在任一时刻只能有一个所有者。
3. 当所有者离开作用域，这个值将被丢弃。

这些规则构成了 Rust 内存管理的基础，让程序员能够在编译时就处理内存问题，而不是在运行时。

## 变量作用域

作用域是一个项（item）在程序中有效的范围：

```rust
{                      // s 在这里无效，它尚未声明
    let s = "hello";   // s 从这里开始有效
    // 可以使用 s
}                      // 此作用域结束，s 不再有效
```

## 内存分配与所有权

### 堆栈分配

- **栈**：速度快，存储固定大小的数据，如整数、浮点数等。
- **堆**：速度较慢，可存储大小在编译时未知或可能变化的数据，如字符串、数组等。

### String 类型与堆内存

`String` 类型是堆上分配的，展示了所有权的重要性：

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权移动到 s2
// println!("{}", s1);  // 错误! s1 已失效
```

上例中，s1 "移动"给了 s2 后，s1 不再有效。这种设计避免了"双重释放"的内存安全问题。

### 克隆

当需要深度复制堆数据而不是移动，可以使用 `clone` 方法：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 数据被深度复制
println!("s1 = {}, s2 = {}", s1, s2);  // 两者都有效
```

### 栈上数据：Copy 特性

实现了 `Copy` 特性的类型，在赋值时自动复制而不是移动：

```rust
let x = 5;
let y = x;  // x 被复制到 y，x 仍然有效
println!("x = {}, y = {}", x, y);
```

以下类型实现了 `Copy` 特性：

- 所有整数类型（`i32`, `u32` 等）
- 布尔类型（`bool`）
- 浮点类型（`f64`, `f32`）
- 字符类型（`char`）
- 包含上述类型的元组，例如 `(i32, i32)`

## 所有权与函数

函数调用同样会影响所有权：

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s 的值移动到函数中...
    // println!("{}", s);  // 错误! s 已无效

    let x = 5;
    makes_copy(x);  // x 的值被复制，x 仍然有效
    println!("{}", x);  // 正常工作
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string 离开作用域并被丢弃

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer 离开作用域，不会发生特别的事情
```

### 函数返回值与所有权

函数也可以返回值来转移所有权：

```rust
fn main() {
    let s1 = gives_ownership();  // gives_ownership 将返回值移动给 s1
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 被移动到函数中，函数返回值移动到 s3
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回 a_string，所有权被移动到调用函数
}
```

## 引用与借用

为避免频繁转移所有权，Rust 提供了"引用"（reference）机制，允许使用值但不获取其所有权：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 传递 s1 的引用，不转移所有权
    println!("'{}' 的长度为 {}", s1, len);  // s1 仍然有效
}

fn calculate_length(s: &String) -> usize {  // s 是 String 的引用
    s.len()
}  // s 离开作用域，但它仅是引用，不会释放任何内容
```

这种方式称为"借用"（borrowing）—— 函数从所有者那里借用值但不获取所有权。

### 可变引用

默认情况下，引用是不可变的。如需修改，需使用可变引用：

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);  // 传递可变引用
    println!("{}", s);  // 输出: hello, world
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 引用规则

Rust 对引用有以下重要限制：

1. 在任意给定时刻，要么只能有一个可变引用，要么只能有多个不可变引用。
2. 引用必须总是有效的 —— 不能有悬垂引用（dangling references）。

这些规则防止了数据竞争（data race）问题。

```rust
// 正确: 在不同作用域里多个可变引用
let mut s = String::from("hello");
{
    let r1 = &mut s;
    // 使用 r1
}  // r1 离开作用域
let r2 = &mut s;  // 没问题

// 正确: 不可变引用用完后再使用可变引用
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);  // r1 和 r2 不再使用
let r3 = &mut s;  // 没问题
```

## 切片类型（Slice）

切片允许引用集合中的一段连续元素序列，而非整个集合：

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // "hello" 的切片
let world = &s[6..11];  // "world" 的切片
```

### 字符串切片

字符串切片的类型是 `&str`，它是对 UTF-8 字符串的引用的一部分：

```rust
let s = String::from("hello world");
let word = first_word(&s);
println!("first word: {}", word);  // 输出: hello

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

字符串字面值实际上就是存储在程序二进制中的字符串切片：

```rust
let s = "Hello, world!";  // s 的类型是 &str
```

### 其它类型的切片

切片也可用于其它集合类型：

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // 类型是 &[i32]
```

## 所有权的意义

Rust 的所有权系统提供了几个关键优势：

1. **内存安全**：避免了悬垂指针、双重释放等内存安全问题。
2. **无垃圾回收**：不需要垃圾回收器，仍能保证内存安全。
3. **性能优化**：所有权规则在编译时强制执行，对运行时没有开销。
4. **并发安全**：所有权和借用规则也有助于避免数据竞争。

理解所有权是掌握 Rust 的核心，虽然起初可能感到复杂，但这套机制是 Rust 提供安全性、性能和并发性的基础。

## 实践建议

1. 优先使用引用，避免不必要的所有权转移。
2. 当需要修改数据时，明确使用可变引用（`&mut`）。
3. 尽量使用函数参数和返回值来传递和恢复所有权，而不是使用复杂的元组返回多个值。
4. 使用字符串切片（`&str`）作为函数参数，增加函数的通用性，同时避免不必要的堆分配。
5. 在设计结构体和其它数据结构时，考虑生命周期参数，使得引用可以安全存储在结构体中。

掌握所有权系统需要实践和经验，但这是使用 Rust 高效编程的基础。
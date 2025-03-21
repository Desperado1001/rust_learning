// Rust 的入门程序：Hello, World!

fn main() {
    // println! 是一个宏，用于向标准输出打印文本
    println!("Hello, World!");
    
    // 格式化输出示例
    let name = "Rust"; 
    println!("Hello, {}!", name); // 使用 {} 作为占位符
    
    // 可以使用编号占位符来重复使用变量
    println!("{0}是一门{1}的语言，{0}非常高效。", name, "安全");
    
    // 可以使用命名参数
    println!("{language}很有趣!", language=name);
}

// 编译并运行此程序：
// $ rustc 01_hello_world.rs
// $ ./01_hello_world

// 或者使用 Cargo：
// 创建新项目：$ cargo new hello_world
// 在项目中修改 src/main.rs
// 构建并运行：$ cargo run
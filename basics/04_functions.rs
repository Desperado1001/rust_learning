// Rust 函数和方法的基础知识

// 基本函数定义：fn 函数名(参数列表) -> 返回类型 { 函数体 }
fn main() {
    // 调用函数
    say_hello();
    
    // 使用参数
    greet("张三");
    
    // 多个参数
    print_sum(5, 7);
    
    // 带返回值的函数
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
    
    // 使用函数表达式
    let result = {
        let x = 5;
        x + 1  // 注意：没有分号，这是一个表达式，将返回x+1的值
    };
    println!("表达式结果: {}", result);
    
    // 使用带条件的返回值
    let x = 5;
    let y = 10;
    println!("较大的数是: {}", get_max(x, y));
    
    // 调用接收函数作为参数的高阶函数
    let numbers = [1, 2, 3, 4, 5];
    let sum = apply_to_array(numbers, sum_array);
    println!("数组总和: {}", sum);
    
    // 使用结构体和方法
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形面积: {}", rect.area());
    println!("矩形是否是正方形: {}", rect.is_square());
    
    // 静态方法（关联函数）调用
    let square = Rectangle::create_square(25);
    println!("正方形面积: {}", square.area());
}

// 无参数、无返回值的简单函数
fn say_hello() {
    println!("你好，世界！");
}

// 带参数的函数
fn greet(name: &str) {
    println!("你好，{}！", name);
}

// 多个参数
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// 带返回值的函数
fn add(a: i32, b: i32) -> i32 {
    // 最后一个表达式的值会被隐式返回（不需要 return 关键字）
    a + b
}

// 也可以使用 return 关键字显式返回
fn get_max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;  // 提前返回
    }
    b  // 隐式返回
}

// 定义一个将另一个函数应用到数组的高阶函数
fn apply_to_array(arr: [i32; 5], f: fn([i32; 5]) -> i32) -> i32 {
    // 调用参数传递的函数
    f(arr)
}

// 求和函数，被传递给 apply_to_array
fn sum_array(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for &item in arr.iter() {
        sum += item;
    }
    sum
}

// 结构体定义
struct Rectangle {
    width: u32,
    height: u32,
}

// 为结构体实现方法
impl Rectangle {
    // self 参数表示调用方法的实例（类似其他语言的 this）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 另一个方法示例
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // 静态方法（关联函数）没有 self 参数
    // 通常用作构造函数
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Rust 函数注意事项：
// 1. 使用蛇形命名法 (snake_case)
// 2. 参数必须声明类型
// 3. 如果函数返回值，则必须声明返回类型
// 4. 最后一个表达式的值会被隐式返回（没有分号）
// 5. 也可以使用 return 关键字提前返回
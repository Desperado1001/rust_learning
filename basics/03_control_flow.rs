// Rust 控制流：if/else, loops, match 等

fn main() {
    // if 表达式
    let number = 7;
    
    // 基本 if/else 结构
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    
    // if 是表达式，可以用于赋值
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("result 的值是: {}", result);
    
    // 多重条件使用 else if
    let number = 6;
    if number % 4 == 0 {
        println!("number 能被 4 整除");
    } else if number % 3 == 0 {
        println!("number 能被 3 整除");
    } else if number % 2 == 0 {
        println!("number 能被 2 整除");
    } else {
        println!("number 不能被 4、3 或 2 整除");
    }
    
    // 循环：loop
    // loop 创建无限循环，除非手动退出
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            // break 可以返回值
            break counter * 2;
        }
    };
    println!("loop 循环的结果: {}", result);
    
    // while 循环：每次迭代前都检查条件
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射!");
    
    // for 循环：最常用于遍历集合
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值为: {}", element);
    }
    
    // 使用 Range
    for number in 1..4 {  // 1..4 创建从1到3的Range
        println!("{}!", number);
    }
    println!("发射!");
    
    // 使用 Range 反向迭代
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("发射!");
    
    // match 控制流运算符
    let number = 7;
    match number {
        // 匹配单个值
        1 => println!("一"),
        // 匹配多个值
        2 | 3 => println!("二或三"),
        // 匹配范围
        4..=7 => println!("四到七"),
        // 捕获值
        n @ 8..=10 => println!("8到10之间的值: {}", n),
        // 默认情况
        _ => println!("其他值"),
    }
    
    // match 是表达式，可以用于赋值
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("binary 的值是: {}", binary);
    
    // if let 简化版 match，用于只有一个模式的情况
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("值为三!");
    }
}

// 注意：
// 1. if 条件不需要括号，但必须是布尔表达式
// 2. if 的分支必须返回相同类型的值
// 3. match 必须穷尽所有可能（常用 _ 通配符）
// 4. for 循环比 while 循环更安全，不易出错

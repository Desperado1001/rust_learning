// 一个简单的命令行计算器，展示控制流的使用
use std::io;

fn main() {
    println!("简易计算器");
    println!("支持的操作: +, -, *, /, %");
    println!("输入 'q' 退出");
    
    loop {
        // 获取第一个数字
        let first_number = match get_number("请输入第一个数字: ") {
            Some(num) => num,
            None => break,  // 退出程序
        };
        
        // 获取操作符
        let operator = match get_operator() {
            Some(op) => op,
            None => break,  // 退出程序
        };
        
        // 获取第二个数字
        let second_number = match get_number("请输入第二个数字: ") {
            Some(num) => num,
            None => break,  // 退出程序
        };
        
        // 执行计算
        match calculate(first_number, operator, second_number) {
            Ok(result) => println!("结果: {}", result),
            Err(e) => println!("错误: {}", e),
        }
        
        println!("\n----------------------------\n");
    }
    
    println!("感谢使用计算器!");
}

// 获取用户输入的数字
fn get_number(prompt: &str) -> Option<f64> {
    loop {
        println!("{}", prompt);
        
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 去除两端空白
                let input = input.trim();
                
                // 检查是否要退出
                if input.to_lowercase() == "q" {
                    return None;
                }
                
                // 尝试将输入解析为数字
                match input.parse::<f64>() {
                    Ok(num) => return Some(num),
                    Err(_) => println!("无效的数字，请重新输入"),
                }
            }
            Err(e) => {
                println!("无法读取输入: {}", e);
                continue;
            }
        }
    }
}

// 获取用户输入的操作符
fn get_operator() -> Option<char> {
    loop {
        println!("请输入操作符 (+, -, *, /, %): ");
        
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 去除两端空白
                let input = input.trim();
                
                // 检查是否要退出
                if input.to_lowercase() == "q" {
                    return None;
                }
                
                // 检查操作符是否有效
                if input.len() != 1 {
                    println!("请输入单个字符作为操作符");
                    continue;
                }
                
                let op = input.chars().next().unwrap();
                
                match op {
                    '+' | '-' | '*' | '/' | '%' => return Some(op),
                    _ => println!("无效的操作符，请重新输入"),
                }
            }
            Err(e) => {
                println!("无法读取输入: {}", e);
                continue;
            }
        }
    }
}

// 执行计算操作
fn calculate(a: f64, op: char, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(String::from("除数不能为零"))
            } else {
                Ok(a / b)
            }
        },
        '%' => {
            if b == 0.0 {
                Err(String::from("模数不能为零"))
            } else {
                Ok(a % b)
            }
        },
        _ => Err(String::from("不支持的操作符")),
    }
}

// 要运行此程序:
// 1. 保存为 simple_calculator.rs
// 2. 编译: rustc simple_calculator.rs
// 3. 运行: ./simple_calculator

// 或者使用 Cargo:
// 1. 在 Cargo.toml 中添加此文件为 bin 目标
// 2. 运行: cargo run --bin simple_calculator

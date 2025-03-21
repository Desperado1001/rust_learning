use std::env;
use std::process;

// 简单的命令行计算器实现
fn main() {
    // 收集命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查参数数量
    if args.len() != 4 {
        eprintln!("用法: {} <数字> <操作符> <数字>", args[0]);
        eprintln!("支持的操作符: +, -, *, /, %");
        process::exit(1);
    }
    
    // 解析第一个数字
    let first_number = match args[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("错误: '{}' 不是有效的数字", args[1]);
            process::exit(1);
        }
    };
    
    // 获取操作符
    let operator = &args[2];
    
    // 解析第二个数字
    let second_number = match args[3].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("错误: '{}' 不是有效的数字", args[3]);
            process::exit(1);
        }
    };
    
    // 根据操作符计算结果
    let result = calculate(first_number, operator, second_number);
    
    // 根据运算结果打印信息
    match result {
        Ok(value) => println!("结果: {}", value),
        Err(e) => {
            eprintln!("计算错误: {}", e);
            process::exit(1);
        }
    }
}

// 计算函数，返回 Result 类型的结果
fn calculate(first: f64, operator: &str, second: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(first + second),
        "-" => Ok(first - second),
        "*" => Ok(first * second),
        "/" => {
            if second == 0.0 {
                Err(String::from("除数不能为零"))
            } else {
                Ok(first / second)
            }
        },
        "%" => {
            if second == 0.0 {
                Err(String::from("模数不能为零"))
            } else {
                Ok(first % second)
            }
        },
        _ => Err(format!("不支持的操作符: {}", operator))
    }
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        let result = calculate(2.0, "+", 3.0);
        assert_eq!(result, Ok(5.0));
    }
    
    #[test]
    fn test_subtraction() {
        let result = calculate(5.0, "-", 2.0);
        assert_eq!(result, Ok(3.0));
    }
    
    #[test]
    fn test_multiplication() {
        let result = calculate(4.0, "*", 3.0);
        assert_eq!(result, Ok(12.0));
    }
    
    #[test]
    fn test_division() {
        let result = calculate(10.0, "/", 2.0);
        assert_eq!(result, Ok(5.0));
    }
    
    #[test]
    fn test_division_by_zero() {
        let result = calculate(10.0, "/", 0.0);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_modulo() {
        let result = calculate(10.0, "%", 3.0);
        assert_eq!(result, Ok(1.0));
    }
    
    #[test]
    fn test_unsupported_operator() {
        let result = calculate(10.0, "^", 2.0);
        assert!(result.is_err());
    }
}

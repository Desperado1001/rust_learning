// Rust 变量和基本数据类型示例

fn main() {
    // 变量声明：默认情况下变量是不可变的
    let x = 5;
    println!("x的值是: {}", x);
    
    // 尝试修改不可变变量会导致编译错误
    // x = 6; // 这一行会导致错误！
    
    // 使用 mut 关键字声明可变变量
    let mut y = 5;
    println!("y的初始值是: {}", y);
    y = 6;
    println!("y的新值是: {}", y);
    
    // 常量：使用 const 关键字，必须指定类型
    // 命名惯例是全大写加下划线
    const MAX_POINTS: u32 = 100_000;
    println!("常量值: {}", MAX_POINTS);
    
    // 变量遮蔽 (shadowing) - 可以重新声明同名变量
    let z = 5;
    let z = z + 1;        // 这里z变成了6
    let z = z * 2;        // 这里z变成了12
    println!("z的值是: {}", z);
    
    // 使用遮蔽可以改变变量的类型
    let spaces = "   ";            // 字符串类型
    let spaces = spaces.len();    // 数字类型
    println!("空格数: {}", spaces);
    
    // 基本数据类型
    // 整数类型
    let a: i32 = -42;     // 有符号32位整数
    let b: u64 = 100;     // 无符号64位整数
    let c = 0xff;         // 十六进制
    let d = 0o77;         // 八进制
    let e = 0b1111_0000;  // 二进制（可用下划线分隔提高可读性）
    println!("整数示例: {}, {}, {}, {}, {}", a, b, c, d, e);
    
    // 浮点数类型
    let f = 2.0;      // f64类型（默认）
    let g: f32 = 3.0; // f32类型（需明确指定）
    println!("浮点数示例: {}, {}", f, g);
    
    // 布尔类型
    let is_active = true;
    let is_greater: bool = 5 > 3;
    println!("布尔值示例: {}, {}", is_active, is_greater);
    
    // 字符类型：使用单引号
    let letter: char = 'A';
    let emoji = '😊';    // Rust的char是Unicode字符
    println!("字符示例: {}, {}", letter, emoji);
    
    // 复合类型
    // 元组：固定长度，可包含不同类型
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    // 可以通过解构来获取元组中的值
    let (tup_x, tup_y, tup_z) = tup;
    println!("元组解构: {}, {}, {}", tup_x, tup_y, tup_z);
    // 也可以通过索引访问
    println!("元组索引: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // 数组：固定长度，元素必须相同类型
    let arr = [1, 2, 3, 4, 5];
    // 可以指定类型和长度 [类型; 长度]
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // 创建包含相同值的数组
    let arr3 = [3; 5]; // 等同于 [3, 3, 3, 3, 3]
    
    println!("数组第一个元素: {}", arr[0]);
    println!("arr2长度: {}", arr2.len());
    println!("arr3: {:?}", arr3);  // 使用 {:?} 可以打印整个数组
}

// 注意：Rust 是强类型语言，但编译器通常可以推断类型，所以很多时候不需要显式指定类型
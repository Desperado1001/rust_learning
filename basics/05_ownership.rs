// Rust 所有权系统示例

fn main() {
    // 所有权规则演示
    ownership_basics();
    
    // 引用和借用
    references_and_borrowing();
    
    // 切片类型
    slices();
    
    // 所有权与函数
    ownership_with_functions();
}

fn ownership_basics() {
    println!("=== 所有权基础 ===");
    
    // 变量作用域
    {
        let s = "hello"; // s 从这里开始有效
        println!("字符串字面值: {}", s);
    } // 作用域结束，s 不再有效
    
    // String 类型演示所有权
    let s1 = String::from("hello"); // s1 是堆上分配的字符串
    println!("s1: {}", s1);
    
    // 移动(Move)语义
    let s2 = s1; // s1 的所有权移动到 s2，s1 不再有效
    // println!("s1: {}", s1); // 这行会导致编译错误！
    println!("s2 (从s1移动而来): {}", s2);
    
    // 克隆(Clone)
    let s3 = String::from("hello");
    let s4 = s3.clone(); // 深拷贝，s3 仍然有效
    println!("s3: {}, s4: {}", s3, s4);
    
    // 栈上数据的复制(Copy)
    let x = 5;
    let y = x; // 基本类型实现了Copy特性，x 仍然有效
    println!("x: {}, y: {}", x, y);
    
    // 带有所有权和作用域的例子
    let s = String::from("hello");  // s 进入作用域
    
    takes_ownership(s);  // s 的值移动到函数中...
                         // ...因此 s 在这里不再有效
    
    let x = 5;           // x 进入作用域
    
    makes_copy(x);       // x 应该移动到函数中，
                         // 但 i32 是 Copy，所以可以继续使用 x
    
    println!("x 在传递给函数后仍然可用: {}", x);
    // println!("s: {}", s); // 这会导致编译错误，因为s已经被移动
}

// 接受所有权的函数
fn takes_ownership(some_string: String) {
    println!("接管了字符串的所有权: {}", some_string);
} // 函数结束，some_string 被丢弃

// 接受复制类型的函数
fn makes_copy(some_integer: i32) {
    println!("接收了整数的副本: {}", some_integer);
} // 函数结束，some_integer 被丢弃

fn references_and_borrowing() {
    println!("\n=== 引用和借用 ===");
    
    // 引用
    let s1 = String::from("hello");
    
    // 传递引用而不是所有权
    let len = calculate_length(&s1);
    println!("'{}'的长度为 {}.", s1, len); // s1 仍然有效
    
    // 可变引用
    let mut s = String::from("hello");
    
    change(&mut s);
    println!("修改后的字符串: {}", s);
    
    // 在同一作用域中，对同一数据最多只能有一个可变引用
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        println!("可变引用1: {}", r1);
    } // r1 在这里离开作用域，所以可以创建新的引用
    
    let r2 = &mut s;
    println!("可变引用2: {}", r2);
    
    // 不能同时拥有可变引用和不可变引用
    let mut s = String::from("hello");
    
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("不可变引用: {} 和 {}", r1, r2);
    // r1 和 r2 在这里就不再使用了
    
    let r3 = &mut s; // 没问题，因为之前的引用不再使用
    println!("可变引用: {}", r3);
    
    // 悬垂引用(Rust 的编译器能够确保不会出现)
    // let reference_to_nothing = dangle(); // 这不会编译
}

// 引用作为参数的函数
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 函数结束，s 离开作用域，但它只是一个引用，不拥有所有权，所以不会丢弃数据

// 可变引用作为参数的函数
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 会导致悬垂引用的函数（Rust 不允许这样做）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回一个指向 s 的引用，但 s 会在函数结束时离开作用域
// } // s 在这里离开作用域并被丢弃，其引用将指向无效的内存

fn slices() {
    println!("\n=== 切片类型 ===");
    
    // 字符串切片
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("切片: '{}' '{}'", hello, world);
    
    // 切片简写
    let s = String::from("hello");
    
    let slice1 = &s[0..2];
    let slice2 = &s[..2]; // 等价于上面
    
    let len = s.len();
    let slice3 = &s[3..len];
    let slice4 = &s[3..]; // 等价于上面
    
    let slice5 = &s[0..len];
    let slice6 = &s[..]; // 等价于上面
    
    println!("切片简写: '{}' '{}' '{}' '{}' '{}' '{}'", 
             slice1, slice2, slice3, slice4, slice5, slice6);
    
    // 使用切片的示例函数
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("第一个单词: {}", word);
    
    // 直接使用字符串字面值
    let word = first_word("hello world");
    println!("字面值的第一个单词: {}", word);
    
    // 其他类型的切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片长度: {}", slice.len());
    for i in slice {
        print!("{} ", i);
    }
    println!();
}

// 返回字符串中第一个单词的切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn ownership_with_functions() {
    println!("\n=== 所有权与函数 ===");
    
    // 返回所有权
    let s1 = gives_ownership();
    println!("获得所有权: {}", s1);
    
    // 移动所有权
    let s2 = String::from("hello");
    println!("调用前 s2: {}", s2);
    
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {}", s2); // 错误！s2的所有权已移动
    println!("获得新的所有权 s3: {}", s3);
    
    // 使用引用避免所有权转移
    let s4 = String::from("hello");
    let (s4, len) = calculate_length_and_return(s4);
    println!("字符串 '{}' 的长度为: {}", s4, len);
    
    // 更优雅的方式是使用引用
    let s5 = String::from("hello");
    let len = calculate_length(&s5);
    println!("使用引用计算 '{}' 的长度: {}", s5, len);
}

// 返回所有权的函数
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回值将所有权移动给调用者
}

// 接受并返回所有权的函数
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回值将所有权移动给调用者
}

// 接受所有权并返回所有权和计算结果
fn calculate_length_and_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回多个值，包括字符串所有权
}

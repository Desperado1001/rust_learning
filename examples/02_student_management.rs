// 学生管理系统示例
// 展示结构体、函数、方法和所有权的实际应用

use std::io;
use std::collections::HashMap;

// 定义学生结构体
struct Student {
    id: u32,
    name: String,
    age: u8,
    grades: HashMap<String, f32>,
}

// 为 Student 实现方法
impl Student {
    // 关联函数（静态方法）：创建新学生
    fn new(id: u32, name: String, age: u8) -> Student {
        Student {
            id,
            name,
            age,
            grades: HashMap::new(),
        }
    }
    
    // 添加课程成绩
    fn add_grade(&mut self, subject: String, score: f32) {
        self.grades.insert(subject, score);
    }
    
    // 计算平均分
    fn average_grade(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = self.grades.values().sum();
        sum / (self.grades.len() as f32)
    }
    
    // 显示学生信息
    fn display(&self) {
        println!("学生ID: {}", self.id);
        println!("姓名: {}", self.name);
        println!("年龄: {}", self.age);
        println!("课程成绩:");
        
        if self.grades.is_empty() {
            println!("  尚无课程成绩");
        } else {
            for (subject, score) in &self.grades {
                println!("  {}: {:.1}", subject, score);
            }
            println!("平均分: {:.2}", self.average_grade());
        }
    }
}

// 学生管理系统结构体
struct StudentManagement {
    students: HashMap<u32, Student>,
    next_id: u32,
}

// 为 StudentManagement 实现方法
impl StudentManagement {
    // 创建新的学生管理系统
    fn new() -> StudentManagement {
        StudentManagement {
            students: HashMap::new(),
            next_id: 1,
        }
    }
    
    // 添加学生
    fn add_student(&mut self, name: String, age: u8) -> u32 {
        let id = self.next_id;
        let student = Student::new(id, name, age);
        
        self.students.insert(id, student);
        self.next_id += 1;
        
        id  // 返回新添加学生的ID
    }
    
    // 获取学生，返回可变引用
    fn get_student_mut(&mut self, id: u32) -> Option<&mut Student> {
        self.students.get_mut(&id)
    }
    
    // 获取学生，返回不可变引用
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }
    
    // 删除学生
    fn remove_student(&mut self, id: u32) -> bool {
        if self.students.contains_key(&id) {
            self.students.remove(&id);
            true
        } else {
            false
        }
    }
    
    // 显示所有学生的基本信息
    fn list_all_students(&self) {
        if self.students.is_empty() {
            println!("系统中尚无学生记录");
            return;
        }
        
        println!("{:-^50}", " 学生列表 ");
        println!("{:<5} | {:<15} | {:<5} | {:<10}", "ID", "姓名", "年龄", "平均分");
        println!("{:-^50}", "");
        
        for student in self.students.values() {
            println!(
                "{:<5} | {:<15} | {:<5} | {:<10.2}",
                student.id,
                student.name,
                student.age,
                student.average_grade()
            );
        }
    }
}

// 主函数 - 程序入口
fn main() {
    let mut system = StudentManagement::new();
    
    loop {
        // 显示菜单
        display_menu();
        
        // 获取用户选择
        let choice = get_user_choice();
        
        match choice {
            1 => add_student_flow(&mut system),
            2 => view_student_flow(&system),
            3 => add_grade_flow(&mut system),
            4 => system.list_all_students(),
            5 => remove_student_flow(&mut system),
            0 => {
                println!("感谢使用学生管理系统！再见！");
                break;
            }
            _ => println!("无效选项，请重新选择"),
        }
        
        println!("\n按回车键继续...");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    }
}

// 辅助函数 - 显示菜单
fn display_menu() {
    println!("\n{:=^50}", " 学生管理系统 ");
    println!("1. 添加学生");
    println!("2. 查看学生信息");
    println!("3. 添加课程成绩");
    println!("4. 列出所有学生");
    println!("5. 删除学生");
    println!("0. 退出系统");
    println!("{:=^50}", "");
    print!("请选择操作: ");
}

// 辅助函数 - 获取用户输入的整数
fn get_user_choice() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 999, // 返回一个无效的选择，将在 main 函数中处理
    }
}

// 辅助函数 - 获取用户输入的字符串
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("刷新输出失败");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    
    input.trim().to_string()
}

// 辅助函数 - 添加学生流程
fn add_student_flow(system: &mut StudentManagement) {
    println!("\n{:-^50}", " 添加新学生 ");
    
    let name = get_user_input("请输入学生姓名: ");
    
    let age: u8 = loop {
        let age_str = get_user_input("请输入学生年龄: ");
        match age_str.parse() {
            Ok(age) => break age,
            Err(_) => println!("无效年龄，请输入1-150之间的数字"),
        }
    };
    
    let id = system.add_student(name.clone(), age);
    println!("学生 {} 已添加，ID为: {}", name, id);
}

// 辅助函数 - 查看学生信息流程
fn view_student_flow(system: &StudentManagement) {
    println!("\n{:-^50}", " 查看学生信息 ");
    
    let id_str = get_user_input("请输入学生ID: ");
    let id: u32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("无效ID，请输入数字");
            return;
        }
    };
    
    match system.get_student(id) {
        Some(student) => student.display(),
        None => println!("未找到ID为 {} 的学生", id),
    }
}

// 辅助函数 - 添加成绩流程
fn add_grade_flow(system: &mut StudentManagement) {
    println!("\n{:-^50}", " 添加课程成绩 ");
    
    let id_str = get_user_input("请输入学生ID: ");
    let id: u32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("无效ID，请输入数字");
            return;
        }
    };
    
    let subject = get_user_input("请输入课程名称: ");
    
    let score: f32 = loop {
        let score_str = get_user_input("请输入分数 (0-100): ");
        match score_str.parse() {
            Ok(score) => {
                if score >= 0.0 && score <= 100.0 {
                    break score;
                } else {
                    println!("分数必须在0到100之间");
                }
            }
            Err(_) => println!("无效分数，请输入数字"),
        }
    };
    
    match system.get_student_mut(id) {
        Some(student) => {
            student.add_grade(subject.clone(), score);
            println!("已为学生ID {} 添加课程 {} 的成绩: {}", id, subject, score);
        }
        None => println!("未找到ID为 {} 的学生", id),
    }
}

// 辅助函数 - 删除学生流程
fn remove_student_flow(system: &mut StudentManagement) {
    println!("\n{:-^50}", " 删除学生 ");
    
    let id_str = get_user_input("请输入要删除的学生ID: ");
    let id: u32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("无效ID，请输入数字");
            return;
        }
    };
    
    let confirm = get_user_input(format!("确认删除ID为 {} 的学生记录? (y/n): ", id).as_str());
    
    if confirm.to_lowercase() == "y" {
        if system.remove_student(id) {
            println!("学生ID {} 已成功删除", id);
        } else {
            println!("未找到ID为 {} 的学生", id);
        }
    } else {
        println!("已取消删除操作");
    }
}

// 运行此程序:
// 1. 保存为 examples/student_management.rs
// 2. cargo run --bin student_management
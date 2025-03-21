// 文本分析器示例
// 演示所有权、引用、借用和切片等概念的实际应用

use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() {
    println!("{:-^60}", " 文本分析器 ");
    println!("这个程序可以分析文本内容，统计单词数量、行数、字符数量等信息。");
    println!("您可以输入文本，或从文件读取内容进行分析。");
    println!();
    
    let text = get_text_input();
    
    if let Some(content) = text {
        analyze_text(&content);
    } else {
        println!("未能获取任何文本内容。程序退出。");
    }
}

// 获取用户输入的文本
fn get_text_input() -> Option<String> {
    println!("请选择输入方式：");
    println!("1. 直接输入文本");
    println!("2. 从文件中读取（暂不支持）");
    print!("请选择 [1/2]: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("读取选择失败");
    
    match choice.trim() {
        "1" => {
            println!("\n请输入文本内容（输入一行 ### 表示结束）：");
            let mut content = String::new();
            let mut line = String::new();
            
            loop {
                line.clear();
                io::stdin().read_line(&mut line).expect("读取文本失败");
                
                if line.trim() == "###" {
                    break;
                }
                
                content.push_str(&line);
            }
            
            if content.trim().is_empty() {
                println!("未输入任何文本内容。");
                None
            } else {
                Some(content)
            }
        },
        "2" => {
            println!("暂不支持从文件读取，请直接输入文本。");
            None
        },
        _ => {
            println!("无效的选择。请重新运行程序。");
            None
        }
    }
}

// 分析文本内容
fn analyze_text(text: &str) {
    println!("\n{:-^60}", " 分析结果 ");
    
    // 基本统计
    let char_count = count_characters(text);
    let line_count = count_lines(text);
    let word_count = count_words(text);
    
    println!("字符数量: {}", char_count);
    println!("行数: {}", line_count);
    println!("单词数量: {}", word_count);
    
    // 词频统计
    let word_frequencies = analyze_word_frequency(text);
    
    println!("\n{:-^60}", " 词频统计（top 10）");
    
    let mut word_vec: Vec<(&String, &u32)> = word_frequencies.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut count = 0;
    for (word, frequency) in word_vec.iter() {
        if count >= 10 {
            break;
        }
        println!("{:<20}: {}", word, frequency);
        count += 1;
    }
    
    // 每行长度统计
    println!("\n{:-^60}", " 每行长度 ");
    let line_lengths = analyze_line_lengths(text);
    
    let mut max_line_length = 0;
    let mut min_line_length = usize::MAX;
    let mut total_length = 0;
    
    for (i, &length) in line_lengths.iter().enumerate() {
        println!("行 {:2}: {} 字符", i + 1, length);
        
        max_line_length = max_line_length.max(length);
        min_line_length = min_line_length.min(length);
        total_length += length;
    }
    
    if !line_lengths.is_empty() {
        let avg_length = total_length as f64 / line_lengths.len() as f64;
        
        println!("\n最长行: {} 字符", max_line_length);
        println!("最短行: {} 字符", min_line_length);
        println!("平均长度: {:.2} 字符", avg_length);
    }
    
    // 查找最长的单词
    if let Some(longest_word) = find_longest_word(text) {
        println!("\n最长的单词: \"{}\" ({} 字符)", longest_word, longest_word.len());
    } else {
        println!("\n未找到任何单词。");
    }
    
    // 统计标点符号
    let punctuation_count = count_punctuation(text);
    println!("\n标点符号数量: {}", punctuation_count);
}

// 统计字符数量（不计空格和换行符）
fn count_characters(text: &str) -> usize {
    text.chars().filter(|&c| !c.is_whitespace()).count()
}

// 统计行数
fn count_lines(text: &str) -> usize {
    // 如果文本为空，返回0；否则，按换行符分割，计算行数
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

// 统计单词数量
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// 分析单词频率
fn analyze_word_frequency(text: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        // 去除标点符号并转换为小写
        let clean_word = word
            .chars()
            .filter(|&c| c.is_alphanumeric() || c == '-' || c == '\'')
            .collect::<String>()
            .to_lowercase();
        
        if !clean_word.is_empty() {
            *word_counts.entry(clean_word).or_insert(0) += 1;
        }
    }
    
    word_counts
}

// 统计每行的长度
fn analyze_line_lengths(text: &str) -> Vec<usize> {
    text.lines()
        .map(|line| line.chars().count())
        .collect()
}

// 查找最长的单词
fn find_longest_word(text: &str) -> Option<String> {
    text.split_whitespace()
        .map(|word| {
            // 清理单词（去除标点符号）
            word.chars()
                .filter(|&c| c.is_alphanumeric() || c == '-' || c == '\'')
                .collect::<String>()
        })
        .filter(|word| !word.is_empty())
        .max_by_key(|word| word.len())
}

// 统计标点符号数量
fn count_punctuation(text: &str) -> usize {
    text.chars().filter(|&c| c.is_ascii_punctuation()).count()
}

// 这个示例展示了 Rust 所有权系统在实际应用中的使用：
// 1. 通过引用（&str）传递字符串切片，避免不必要的所有权转移
// 2. 使用 mut 关键字在需要时标记可变引用
// 3. 使用 String 类型管理堆上分配的文本内容
// 4. 利用所有权规则在处理集合（如 HashMap）时保持内存安全
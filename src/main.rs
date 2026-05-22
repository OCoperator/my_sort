use std::io;
use sort_algorithms::{
    crumsort,
    crumsort_i32,
    crumsort_u32,
    crumsort_i64,
    crumsort_u64,
    crumsort_str,
    crumsort_generic,
    fluxsort,
    SortArray,
};

fn parse_i32_array(input: &str) -> Result<Vec<i32>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("输入不能为空".to_string());
    }

    trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().map_err(|_| "输入包含无效数字".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .and_then(|arr| {
            if arr.is_empty() {
                Err("未检测到有效数字".to_string())
            } else {
                Ok(arr)
            }
        })
}

fn parse_u32_array(input: &str) -> Result<Vec<u32>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("输入不能为空".to_string());
    }

    trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().map_err(|_| "输入包含无效数字".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .and_then(|arr| {
            if arr.is_empty() {
                Err("未检测到有效数字".to_string())
            } else {
                Ok(arr)
            }
        })
}

fn parse_i64_array(input: &str) -> Result<Vec<i64>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("输入不能为空".to_string());
    }

    trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().map_err(|_| "输入包含无效数字".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .and_then(|arr| {
            if arr.is_empty() {
                Err("未检测到有效数字".to_string())
            } else {
                Ok(arr)
            }
        })
}

fn parse_u64_array(input: &str) -> Result<Vec<u64>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("输入不能为空".to_string());
    }

    trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().map_err(|_| "输入包含无效数字".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .and_then(|arr| {
            if arr.is_empty() {
                Err("未检测到有效数字".to_string())
            } else {
                Ok(arr)
            }
        })
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\n=== CrumSort 排序算法演示 ===");
        println!("支持的数据类型:");
        println!("1. i32 (32位有符号整数)");
        println!("2. u32 (32位无符号整数)");
        println!("3. i64 (64位有符号整数)");
        println!("4. u64 (64位无符号整数)");
        println!("5. String (字符串)");
        println!("6. 退出");
        
        let choice = read_input("\n请选择数据类型 (1-6):");
        let Ok(choice) = choice.parse::<usize>() else {
            println!("无效输入，请输入数字1-6");
            continue;
        };

        match choice {
            1 => {
                let input = read_input("\n请输入 i32 数组（空格或逗号分隔）:");
                let mut arr = match parse_i32_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                println!("原始数组: {:?}", arr);
                
                let mut arr_copy = arr.clone();
                crumsort_i32(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
            }
            2 => {
                let input = read_input("\n请输入 u32 数组（空格或逗号分隔）:");
                let mut arr = match parse_u32_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                println!("原始数组: {:?}", arr);
                
                let mut arr_copy = arr.clone();
                crumsort_u32(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
            }
            3 => {
                let input = read_input("\n请输入 i64 数组（空格或逗号分隔）:");
                let mut arr = match parse_i64_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                println!("原始数组: {:?}", arr);
                
                let mut arr_copy = arr.clone();
                crumsort_i64(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
            }
            4 => {
                let input = read_input("\n请输入 u64 数组（空格或逗号分隔）:");
                let mut arr = match parse_u64_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                println!("原始数组: {:?}", arr);
                
                let mut arr_copy = arr.clone();
                crumsort_u64(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
            }
            5 => {
                let input = read_input("\n请输入字符串数组（空格或逗号分隔）:");
                let mut arr: Vec<String> = input
                    .split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                if arr.is_empty() {
                    println!("错误: 未检测到有效字符串");
                    continue;
                }
                println!("原始数组: {:?}", arr);
                
                let mut arr_copy = arr.clone();
                crumsort_str(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
            }
            6 => {
                println!("退出程序");
                break;
            }
            _ => println!("无效选择，请输入1-6"),
        }
    }
}

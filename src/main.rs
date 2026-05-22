use std::io;
use sort_algorithms::{
    crumsort_i32,
    fluxsort_i32,
    quadsort_i32,
    crumsort_u32,
    fluxsort_u32,
    quadsort_u32,
    crumsort_i64,
    fluxsort_i64,
    quadsort_i64,
    crumsort_u64,
    fluxsort_u64,
    quadsort_u64,
    crumsort_str,
    fluxsort_str,
    quadsort_str,
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

fn run_sort_test_i32(arr: &[i32]) {
    println!("原始数组: {:?}", arr);
    
    let mut arr1 = arr.to_vec();
    crumsort_i32(&mut arr1);
    println!("Crumsort: {:?}", arr1);
    
    let mut arr2 = arr.to_vec();
    fluxsort_i32(&mut arr2);
    println!("Fluxsort: {:?}", arr2);
    
    let mut arr3 = arr.to_vec();
    quadsort_i32(&mut arr3);
    println!("Quadsort: {:?}", arr3);
}

fn run_sort_test_u32(arr: &[u32]) {
    println!("原始数组: {:?}", arr);
    
    let mut arr1 = arr.to_vec();
    crumsort_u32(&mut arr1);
    println!("Crumsort: {:?}", arr1);
    
    let mut arr2 = arr.to_vec();
    fluxsort_u32(&mut arr2);
    println!("Fluxsort: {:?}", arr2);
    
    let mut arr3 = arr.to_vec();
    quadsort_u32(&mut arr3);
    println!("Quadsort: {:?}", arr3);
}

fn run_sort_test_i64(arr: &[i64]) {
    println!("原始数组: {:?}", arr);
    
    let mut arr1 = arr.to_vec();
    crumsort_i64(&mut arr1);
    println!("Crumsort: {:?}", arr1);
    
    let mut arr2 = arr.to_vec();
    fluxsort_i64(&mut arr2);
    println!("Fluxsort: {:?}", arr2);
    
    let mut arr3 = arr.to_vec();
    quadsort_i64(&mut arr3);
    println!("Quadsort: {:?}", arr3);
}

fn run_sort_test_u64(arr: &[u64]) {
    println!("原始数组: {:?}", arr);
    
    let mut arr1 = arr.to_vec();
    crumsort_u64(&mut arr1);
    println!("Crumsort: {:?}", arr1);
    
    let mut arr2 = arr.to_vec();
    fluxsort_u64(&mut arr2);
    println!("Fluxsort: {:?}", arr2);
    
    let mut arr3 = arr.to_vec();
    quadsort_u64(&mut arr3);
    println!("Quadsort: {:?}", arr3);
}

fn run_sort_test_str(arr: &[String]) {
    println!("原始数组: {:?}", arr);
    
    let mut arr1 = arr.to_vec();
    crumsort_str(&mut arr1);
    println!("Crumsort: {:?}", arr1);
    
    let mut arr2 = arr.to_vec();
    fluxsort_str(&mut arr2);
    println!("Fluxsort: {:?}", arr2);
    
    let mut arr3 = arr.to_vec();
    quadsort_str(&mut arr3);
    println!("Quadsort: {:?}", arr3);
}

fn main() {
    loop {
        println!("\n=== 排序算法演示 ===");
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
                let arr = match parse_i32_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                run_sort_test_i32(&arr);
            }
            2 => {
                let input = read_input("\n请输入 u32 数组（空格或逗号分隔）:");
                let arr = match parse_u32_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                run_sort_test_u32(&arr);
            }
            3 => {
                let input = read_input("\n请输入 i64 数组（空格或逗号分隔）:");
                let arr = match parse_i64_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                run_sort_test_i64(&arr);
            }
            4 => {
                let input = read_input("\n请输入 u64 数组（空格或逗号分隔）:");
                let arr = match parse_u64_array(&input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("错误: {}", e);
                        continue;
                    }
                };
                run_sort_test_u64(&arr);
            }
            5 => {
                let input = read_input("\n请输入字符串数组（空格或逗号分隔）:");
                let arr: Vec<String> = input
                    .split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                if arr.is_empty() {
                    println!("错误: 未检测到有效字符串");
                    continue;
                }
                run_sort_test_str(&arr);
            }
            6 => {
                println!("退出程序");
                break;
            }
            _ => println!("无效选择，请输入1-6"),
        }
    }
}

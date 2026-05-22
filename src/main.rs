use std::io;
use sort_algorithms::{crumsort, fluxsort};

fn parse_array_input(input: &str) -> Result<Vec<i128>, String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Err("输入不能为空".to_string());
    }

    trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i128>().map_err(|_| "输入包含无效数字，请只输入整数".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .and_then(|arr| {
            if arr.is_empty() {
                Err("未检测到有效数字".to_string())
            } else {
                Ok(arr)
            }
        })
}

fn read_array() -> Vec<i128> {
    println!("请输入要排序的数组（数字之间用空格或逗号分隔）:");
    println!("例如: 9, 4, 7, 2, 5 或 9 4 7 2 5\n");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        match parse_array_input(&input) {
            Ok(arr) => {
                println!("成功读取 {} 个数字", arr.len());
                return arr;
            }
            Err(e) => println!("错误: {}\n请重新输入:", e),
        }
    }
}

fn main() {
    let mut arr = read_array();

    loop {
        println!("\n初始数组: {:?}\n", arr);
        println!("请选择操作:");
        println!("1. crumsort");
        println!("2. fluxsort");
        println!("3. 重新输入数组");
        println!("4. 退出");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");

        let Ok(choice) = choice.trim().parse::<usize>() else {
            println!("无效输入，请输入数字1-4");
            continue;
        };

        match choice {
            1 => {
                let mut arr_copy = arr.clone();
                println!("使用 crumsort 排序...");
                crumsort(&mut arr_copy);
                println!("排序结果: {:?}\n", arr_copy);
            }
            2 => {
                let mut arr_copy = arr.clone();
                println!("使用 fluxsort 排序...");
                fluxsort(&mut arr_copy);
                println!("排序结果: {:?}\n", arr_copy);
            }
            3 => arr = read_array(),
            4 => {
                println!("退出程序");
                break;
            }
            _ => println!("无效选择，请输入1-4"),
        }
    }
}

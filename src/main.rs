use std::io;
use sort_algorithms::{crumsort, fluxsort};

fn main() {
    let arr = vec![9, 4, 7, 2, 5, 1, 8, 3, 6, 0, 15, 12, 18, 11, 13, 10, 17, 14, 16, 19];
    
    println!("初始数组: {:?}", arr);
    println!();
    
    loop {
        println!("请选择排序算法:");
        println!("1. crumsort");
        println!("2. fluxsort");
        println!("3. 退出");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        
        let choice: usize = match choice.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("无效输入，请输入数字1-3");
                continue;
            }
        };
        
        match choice {
            1 => {
                let mut arr_copy = arr.clone();
                println!("使用 crumsort 排序...");
                crumsort(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
                println!();
            }
            2 => {
                let mut arr_copy = arr.clone();
                println!("使用 fluxsort 排序...");
                fluxsort(&mut arr_copy);
                println!("排序结果: {:?}", arr_copy);
                println!();
            }
            3 => {
                println!("退出程序");
                break;
            }
            _ => {
                println!("无效选择，请输入1-3");
            }
        }
    }
}

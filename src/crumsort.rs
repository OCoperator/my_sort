fn fulcrum_partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let len = arr.len();
    if len <= 1 {
        return len;
    }
    
    let pivot = arr[0].clone();
    let mut head = 0;
    let mut tail = len - 1;
    
    loop {
        if arr[tail] > pivot {
            tail -= 1;
            continue;
        }
        
        if head >= tail {
            arr[head] = pivot;
            return head;
        }
        
        arr[head] = arr[tail].clone();
        head += 1;
        
        loop {
            if head >= tail {
                arr[head] = pivot;
                return head;
            }
            
            if arr[head] <= pivot {
                head += 1;
                continue;
            }
            
            arr[tail] = arr[head].clone();
            tail -= 1;
            break;
        }
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn crumsort_recursive<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    
    if len <= 24 {
        insertion_sort(arr);
        return;
    }
    
    let pivot_pos = fulcrum_partition(arr);
    
    if pivot_pos > 0 {
        crumsort_recursive(&mut arr[..pivot_pos]);
    }
    
    if pivot_pos + 1 < len {
        crumsort_recursive(&mut arr[pivot_pos + 1..]);
    }
}

pub fn crumsort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    crumsort_recursive(arr);
}

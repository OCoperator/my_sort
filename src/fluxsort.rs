fn partition<T: Ord + Clone>(arr: &mut [T], swap: &mut [T]) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }
    
    let len = arr.len();
    let pivot_idx = len / 2;
    let pivot = arr[pivot_idx].clone();
    
    let mut left = 0;
    let mut right = len - 1;
    let mut swap_pos = 0;
    
    while left <= right {
        if arr[right] > pivot {
            swap[swap_pos] = arr[right].clone();
            swap_pos += 1;
            right -= 1;
        } else {
            arr[left] = arr[right].clone();
            left += 1;
            right -= 1;
        }
    }
    
    arr[left - 1] = pivot;
    
    for i in 0..swap_pos {
        arr[len - swap_pos + i] = swap[i].clone();
    }
    
    len - swap_pos
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

fn fluxsort_recursive<T: Ord + Clone>(arr: &mut [T], swap: &mut [T]) {
    let len = arr.len();
    
    if len <= 96 {
        insertion_sort(arr);
        return;
    }
    
    let pivot_pos = partition(arr, swap);
    
    if pivot_pos > 0 {
        fluxsort_recursive(&mut arr[..pivot_pos], swap);
    }
    
    if pivot_pos < len {
        fluxsort_recursive(&mut arr[pivot_pos..], swap);
    }
}

pub fn fluxsort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let len = arr.len();
    let swap_size = len / 2;
    let mut swap: Vec<T> = Vec::with_capacity(swap_size);
    unsafe { swap.set_len(swap_size); }
    
    fluxsort_recursive(arr, &mut swap);
}

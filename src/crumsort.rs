#![allow(clippy::many_single_char_names)]

use std::mem::{self, MaybeUninit};
use std::ptr;

#[inline(always)]
fn insertion_sort_generic<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[inline(always)]
fn median3_i32(arr: &mut [i32], a: usize, b: usize, c: usize) -> usize {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    if x < y {
        if y < z { b } else if x < z { c } else { a }
    } else {
        if x < z { a } else if y < z { c } else { b }
    }
}

#[inline(always)]
fn median3_u32(arr: &mut [u32], a: usize, b: usize, c: usize) -> usize {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    if x < y {
        if y < z { b } else if x < z { c } else { a }
    } else {
        if x < z { a } else if y < z { c } else { b }
    }
}

#[inline(always)]
fn median3_i64(arr: &mut [i64], a: usize, b: usize, c: usize) -> usize {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    if x < y {
        if y < z { b } else if x < z { c } else { a }
    } else {
        if x < z { a } else if y < z { c } else { b }
    }
}

#[inline(always)]
fn pseudomedian_of_nine_i32(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;
    let a = lo;
    let b = lo + 1;
    let c = lo + 2;
    let d = mid - 1;
    let e = mid;
    let f = mid + 1;
    let g = hi - 2;
    let h = hi - 1;
    let i = hi;

    let m1 = median3_i32(arr, a, b, c);
    let m2 = median3_i32(arr, d, e, f);
    let m3 = median3_i32(arr, g, h, i);
    median3_i32(arr, m1, m2, m3)
}

#[inline(always)]
fn branchless_partition_i32(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let len = hi - lo + 1;
    
    let pivot_idx = if len < 2048 {
        pseudomedian_of_nine_i32(arr, lo, hi)
    } else if len < 65536 {
        let mid = lo + (hi - lo) / 2;
        let q1 = lo + (mid - lo) / 2;
        let q3 = mid + (hi - mid) / 2;
        let m1 = median3_i32(arr, lo, q1, mid);
        let m2 = median3_i32(arr, mid, q3, hi);
        median3_i32(arr, m1, mid, m2)
    } else {
        let sample_size = ((len as f64).cbrt() as usize).max(9);
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push((lo + i * step, arr[lo + i * step]));
        }
        samples.sort_by_key(|s| s.1);
        samples[sample_size / 2].0
    };

    arr.swap(lo, pivot_idx);
    let pivot = arr[lo];
    
    let mut head = lo;
    let mut tail = hi;

    unsafe {
        loop {
            while tail > head && arr[tail] > pivot {
                tail -= 1;
            }

            if head >= tail {
                arr[head] = pivot;
                return head;
            }

            ptr::write(&mut arr[head], arr[tail]);
            head += 1;

            while head <= tail {
                let cmp = arr[head] > pivot;
                if cmp {
                    ptr::write(&mut arr[tail], arr[head]);
                    tail -= 1;
                    break;
                }
                head += 1;
            }
        }
    }
}

#[inline(always)]
fn is_sorted_i32(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[inline(always)]
fn is_reverse_sorted_i32(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

#[inline(always)]
fn reverse_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

#[inline(always)]
fn analyze_segment_i32(arr: &[i32], start: usize, end: usize) -> f64 {
    let len = end - start;
    if len <= 1 {
        return 1.0;
    }

    let mut ordered = 0;
    for i in start..end - 1 {
        ordered += (arr[i] <= arr[i + 1]) as usize;
    }

    ordered as f64 / (len - 1) as f64
}

#[inline(always)]
fn insertion_sort_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let val = arr[i];
        let mut j = i;
        while j > 0 && val < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }
}

fn quadsort_i32(arr: &mut [i32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_i32(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_i32(arr, lo, mid - 1);
    quadsort_i32(arr, mid, hi);

    merge_i32(arr, lo, mid - 1, mid, hi);
}

fn merge_i32(arr: &mut [i32], lo: usize, mid_left: usize, mid_right: usize, hi: usize) {
    let left_len = mid_left - lo + 1;
    let right_len = hi - mid_right + 1;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid_right], right.as_mut_ptr(), right_len);
        left.set_len(left_len);
        right.set_len(right_len);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = lo;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn crumsort_i32_recursive(arr: &mut [i32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort_i32(&mut arr[lo..=hi]);
        return;
    }

    if len <= 128 {
        if is_sorted_i32(&arr[lo..=hi]) {
            return;
        }

        if is_reverse_sorted_i32(&arr[lo..=hi]) {
            reverse_i32(&mut arr[lo..=hi]);
            return;
        }
    }

    if len > 128 && len < 65536 {
        let segment_size = len / 4;
        let mut max_ordered: f64 = 0.0;
        for i in 0..4 {
            let start = lo + i * segment_size;
            let end = if i == 3 { hi + 1 } else { start + segment_size };
            let ordered_ratio = analyze_segment_i32(arr, start, end);
            max_ordered = max_ordered.max(ordered_ratio);
        }

        if max_ordered > 0.6 {
            quadsort_i32(arr, lo, hi);
            return;
        }
    }

    if len >= 65536 {
        let sample_size = 256;
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push((lo + i * step, arr[lo + i * step]));
        }
        samples.sort_by_key(|s| s.1);
        
        let pivot = samples[sample_size / 2].1;
        let mut left = lo;
        let mut right = hi;

        while left <= right {
            while left <= right && arr[left] <= pivot {
                left += 1;
            }
            while left <= right && arr[right] > pivot {
                right -= 1;
            }
            if left <= right {
                arr.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        if left > lo {
            crumsort_i32_recursive(arr, lo, left - 1);
        }
        if left <= hi {
            crumsort_i32_recursive(arr, left, hi);
        }
        return;
    }

    let pivot_pos = branchless_partition_i32(arr, lo, hi);

    let left_size = pivot_pos - lo;
    let right_size = hi - pivot_pos;
    let total_size = len;

    if left_size < total_size / 16 || right_size < total_size / 16 {
        quadsort_i32(arr, lo, hi);
        return;
    }

    if pivot_pos > lo {
        crumsort_i32_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_i32_recursive(arr, pivot_pos + 1, hi);
    }
}

pub fn crumsort_i32(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    crumsort_i32_recursive(arr, 0, len - 1);
}

#[inline(always)]
fn branchless_partition_u32(arr: &mut [u32], lo: usize, hi: usize) -> usize {
    let len = hi - lo + 1;
    
    let pivot_idx = if len < 2048 {
        pseudomedian_of_nine_u32(arr, lo, hi)
    } else if len < 65536 {
        let mid = lo + (hi - lo) / 2;
        let q1 = lo + (mid - lo) / 2;
        let q3 = mid + (hi - mid) / 2;
        let m1 = median3_u32(arr, lo, q1, mid);
        let m2 = median3_u32(arr, mid, q3, hi);
        median3_u32(arr, m1, mid, m2)
    } else {
        let sample_size = ((len as f64).cbrt() as usize).max(9);
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push((lo + i * step, arr[lo + i * step]));
        }
        samples.sort_by_key(|s| s.1);
        samples[sample_size / 2].0
    };

    arr.swap(lo, pivot_idx);
    let pivot = arr[lo];
    
    let mut head = lo;
    let mut tail = hi;

    loop {
        while tail > head && arr[tail] > pivot {
            tail -= 1;
        }

        if head >= tail {
            arr[head] = pivot;
            return head;
        }

        arr[head] = arr[tail];
        head += 1;

        while head <= tail {
            if arr[head] > pivot {
                arr[tail] = arr[head];
                tail -= 1;
                break;
            }
            head += 1;
        }
    }
}

fn pseudomedian_of_nine_u32(arr: &mut [u32], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;
    let a = lo;
    let b = lo + 1;
    let c = lo + 2;
    let d = mid - 1;
    let e = mid;
    let f = mid + 1;
    let g = hi - 2;
    let h = hi - 1;
    let i = hi;

    let m1 = median3_u32(arr, a, b, c);
    let m2 = median3_u32(arr, d, e, f);
    let m3 = median3_u32(arr, g, h, i);
    median3_u32(arr, m1, m2, m3)
}

fn insertion_sort_u32(arr: &mut [u32]) {
    let len = arr.len();
    for i in 1..len {
        let val = arr[i];
        let mut j = i;
        while j > 0 && val < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }
}

fn quadsort_u32(arr: &mut [u32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_u32(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_u32(arr, lo, mid - 1);
    quadsort_u32(arr, mid, hi);

    merge_u32(arr, lo, mid - 1, mid, hi);
}

fn merge_u32(arr: &mut [u32], lo: usize, mid_left: usize, mid_right: usize, hi: usize) {
    let left_len = mid_left - lo + 1;
    let right_len = hi - mid_right + 1;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid_right], right.as_mut_ptr(), right_len);
        left.set_len(left_len);
        right.set_len(right_len);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = lo;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn crumsort_u32_recursive(arr: &mut [u32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort_u32(&mut arr[lo..=hi]);
        return;
    }

    let pivot_pos = branchless_partition_u32(arr, lo, hi);

    let left_size = pivot_pos - lo;
    let right_size = hi - pivot_pos;

    if left_size < len / 16 || right_size < len / 16 {
        quadsort_u32(arr, lo, hi);
        return;
    }

    if pivot_pos > lo {
        crumsort_u32_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_u32_recursive(arr, pivot_pos + 1, hi);
    }
}

pub fn crumsort_u32(arr: &mut [u32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    crumsort_u32_recursive(arr, 0, len - 1);
}

pub fn crumsort_i64(arr: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let mut buf: Vec<u64> = arr.iter().map(|&x| x as u64 ^ (1u64 << 63)).collect();
    crumsort_u64(&mut buf);
    buf.iter_mut().enumerate().for_each(|(i, val)| {
        arr[i] = (*val ^ (1u64 << 63)) as i64;
    });
}

fn crumsort_u64_recursive(arr: &mut [u64], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort_u64(&mut arr[lo..=hi]);
        return;
    }

    let pivot_pos = branchless_partition_u64(arr, lo, hi);

    let left_size = pivot_pos - lo;
    let right_size = hi - pivot_pos;

    if left_size < len / 16 || right_size < len / 16 {
        quadsort_u64(arr, lo, hi);
        return;
    }

    if pivot_pos > lo {
        crumsort_u64_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_u64_recursive(arr, pivot_pos + 1, hi);
    }
}

pub fn crumsort_u64(arr: &mut [u64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    crumsort_u64_recursive(arr, 0, len - 1);
}

fn branchless_partition_u64(arr: &mut [u64], lo: usize, hi: usize) -> usize {
    let len = hi - lo + 1;
    
    let mid = lo + (hi - lo) / 2;
    let pivot_idx = if len < 2048 {
        let a = lo;
        let b = lo + 1;
        let c = lo + 2;
        let d = mid - 1;
        let e = mid;
        let f = mid + 1;
        let g = hi - 2;
        let h = hi - 1;
        let i = hi;

        let m1 = median3_u64(arr, a, b, c);
        let m2 = median3_u64(arr, d, e, f);
        let m3 = median3_u64(arr, g, h, i);
        median3_u64(arr, m1, m2, m3)
    } else {
        let q1 = lo + (mid - lo) / 2;
        let q3 = mid + (hi - mid) / 2;
        let m1 = median3_u64(arr, lo, q1, mid);
        let m2 = median3_u64(arr, mid, q3, hi);
        median3_u64(arr, m1, mid, m2)
    };

    arr.swap(lo, pivot_idx);
    let pivot = arr[lo];
    
    let mut head = lo;
    let mut tail = hi;

    loop {
        while tail > head && arr[tail] > pivot {
            tail -= 1;
        }

        if head >= tail {
            arr[head] = pivot;
            return head;
        }

        arr[head] = arr[tail];
        head += 1;

        while head <= tail {
            if arr[head] > pivot {
                arr[tail] = arr[head];
                tail -= 1;
                break;
            }
            head += 1;
        }
    }
}

fn median3_u64(arr: &mut [u64], a: usize, b: usize, c: usize) -> usize {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    if x < y {
        if y < z { b } else if x < z { c } else { a }
    } else {
        if x < z { a } else if y < z { c } else { b }
    }
}

fn insertion_sort_u64(arr: &mut [u64]) {
    let len = arr.len();
    for i in 1..len {
        let val = arr[i];
        let mut j = i;
        while j > 0 && val < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }
}

fn quadsort_u64(arr: &mut [u64], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_u64(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_u64(arr, lo, mid - 1);
    quadsort_u64(arr, mid, hi);

    merge_u64(arr, lo, mid - 1, mid, hi);
}

fn merge_u64(arr: &mut [u64], lo: usize, mid_left: usize, mid_right: usize, hi: usize) {
    let left_len = mid_left - lo + 1;
    let right_len = hi - mid_right + 1;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid_right], right.as_mut_ptr(), right_len);
        left.set_len(left_len);
        right.set_len(right_len);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = lo;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn crumsort_str(arr: &mut [String]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    crumsort_str_recursive(arr, 0, len - 1);
}

fn crumsort_str_recursive(arr: &mut [String], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort_str(&mut arr[lo..=hi]);
        return;
    }

    let pivot_pos = partition_str(arr, lo, hi);

    if pivot_pos > lo {
        crumsort_str_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_str_recursive(arr, pivot_pos + 1, hi);
    }
}

fn partition_str(arr: &mut [String], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;
    arr.swap(lo, mid);
    let pivot = arr[lo].clone();
    
    let mut head = lo;
    let mut tail = hi;

    loop {
        while tail > head && arr[tail] > pivot {
            tail -= 1;
        }

        if head >= tail {
            arr[head] = pivot;
            return head;
        }

        arr[head] = arr[tail].clone();
        head += 1;

        while head <= tail {
            if arr[head] > pivot {
                arr[tail] = arr[head].clone();
                tail -= 1;
                break;
            }
            head += 1;
        }
    }
}

fn insertion_sort_str(arr: &mut [String]) {
    let len = arr.len();
    for i in 1..len {
        let val = arr[i].clone();
        let mut j = i;
        while j > 0 && val < arr[j - 1] {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = val;
    }
}

pub fn crumsort_generic<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    crumsort_generic_recursive(arr, 0, len - 1);
}

fn crumsort_generic_recursive<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort_generic(&mut arr[lo..=hi]);
        return;
    }

    let pivot_pos = partition_generic(arr, lo, hi);

    if pivot_pos > lo {
        crumsort_generic_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_generic_recursive(arr, pivot_pos + 1, hi);
    }
}

fn partition_generic<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;
    arr.swap(lo, mid);
    let pivot = arr[lo].clone();
    
    let mut head = lo;
    let mut tail = hi;

    loop {
        while tail > head && arr[tail] > pivot {
            tail -= 1;
        }

        if head >= tail {
            arr[head] = pivot;
            return head;
        }

        arr[head] = arr[tail].clone();
        head += 1;

        while head <= tail {
            if arr[head] > pivot {
                arr[tail] = arr[head].clone();
                tail -= 1;
                break;
            }
            head += 1;
        }
    }
}

pub enum SortArray<'a> {
    I32(&'a mut [i32]),
    U32(&'a mut [u32]),
    I64(&'a mut [i64]),
    U64(&'a mut [u64]),
    String(&'a mut [String]),
}

pub fn crumsort(arr: SortArray) {
    match arr {
        SortArray::I32(a) => crumsort_i32(a),
        SortArray::U32(a) => crumsort_u32(a),
        SortArray::I64(a) => crumsort_i64(a),
        SortArray::U64(a) => crumsort_u64(a),
        SortArray::String(a) => crumsort_str(a),
    }
}

use std::ptr;

#[inline(always)]
fn median3_i32(arr: &[i32], a: usize, b: usize, c: usize) -> i32 {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    let x_gt_y = (x > y) as i32;
    let x_gt_z = (x > z) as i32;
    let y_gt_z = (y > z) as i32;
    [x, y, z][((x_gt_y == x_gt_z) as usize) + ((x_gt_z ^ y_gt_z) as usize)]
}

#[inline(always)]
fn quasimedian_of_nine_i32(arr: &[i32], lo: usize, hi: usize) -> i32 {
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
    median3_i32(&[m1, m2, m3], 0, 1, 2)
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

fn is_sorted_i32(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn is_reverse_sorted_i32(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

fn reverse_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

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

fn flux_partition_i32(arr: &mut [i32], swap: &mut [i32], lo: usize, hi: usize) -> (usize, usize) {
    let len = hi - lo + 1;
    
    let pivot = if len < 2048 {
        quasimedian_of_nine_i32(arr, lo, hi)
    } else {
        let sample_size = ((len as f64).cbrt() as usize).max(32).min(1024);
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push(arr[lo + i * step]);
        }
        samples.sort_unstable();
        samples[sample_size / 2]
    };

    let mut left = lo;
    let mut right = lo;
    let mut swap_pos = 0;

    for i in lo..=hi {
        if arr[i] <= pivot {
            arr[right] = arr[i];
            if left != right {
                arr[left] = arr[right];
            }
            left += 1;
            right += 1;
        } else {
            swap[swap_pos] = arr[i];
            swap_pos += 1;
        }
    }

    (left - lo, swap_pos)
}

fn fluxsort_i32_recursive(arr: &mut [i32], swap: &mut [i32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 96 {
        insertion_sort_i32(&mut arr[lo..=hi]);
        return;
    }

    if len <= 512 {
        if is_sorted_i32(&arr[lo..=hi]) {
            return;
        }

        if is_reverse_sorted_i32(&arr[lo..=hi]) {
            reverse_i32(&mut arr[lo..=hi]);
            return;
        }
    }

    if len > 512 {
        let segment_size = len / 4;
        let mut max_ordered: f64 = 0.0;
        for i in 0..4 {
            let start = lo + i * segment_size;
            let end = if i == 3 { hi + 1 } else { start + segment_size };
            let ordered_ratio = analyze_segment_i32(arr, start, end);
            max_ordered = max_ordered.max(ordered_ratio);
        }

        if max_ordered > 0.5 {
            insertion_sort_i32(&mut arr[lo..=hi]);
            return;
        }
    }

    let (left_size, right_size) = flux_partition_i32(arr, swap, lo, hi);

    if left_size < len / 16 || right_size < len / 16 {
        insertion_sort_i32(&mut arr[lo..=hi]);
        return;
    }

    if left_size > 0 {
        fluxsort_i32_recursive(arr, swap, lo, lo + left_size - 1);
    }

    if right_size > 0 {
        let swap_lo = lo + left_size;
        let swap_hi = lo + left_size + right_size - 1;
        
        unsafe {
            ptr::copy_nonoverlapping(swap.as_ptr(), arr.as_mut_ptr().add(swap_lo), right_size);
        }
        
        fluxsort_i32_recursive(arr, swap, swap_lo, swap_hi);
    }
}

pub fn fluxsort_i32(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let swap_size = len / 2;
    let mut swap = vec![0; swap_size];

    fluxsort_i32_recursive(arr, &mut swap, 0, len - 1);
}

#[inline(always)]
fn median3_u32(arr: &[u32], a: usize, b: usize, c: usize) -> u32 {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    let x_gt_y = (x > y) as u32;
    let x_gt_z = (x > z) as u32;
    let y_gt_z = (y > z) as u32;
    [x, y, z][((x_gt_y == x_gt_z) as usize) + ((x_gt_z ^ y_gt_z) as usize)]
}

#[inline(always)]
fn quasimedian_of_nine_u32(arr: &[u32], lo: usize, hi: usize) -> u32 {
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
    median3_u32(&[m1, m2, m3], 0, 1, 2)
}

#[inline(always)]
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

fn flux_partition_u32(arr: &mut [u32], swap: &mut [u32], lo: usize, hi: usize) -> (usize, usize) {
    let len = hi - lo + 1;
    
    let pivot = if len < 2048 {
        quasimedian_of_nine_u32(arr, lo, hi)
    } else {
        let sample_size = ((len as f64).cbrt() as usize).max(32).min(1024);
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push(arr[lo + i * step]);
        }
        samples.sort_unstable();
        samples[sample_size / 2]
    };

    let mut left = lo;
    let mut right = lo;
    let mut swap_pos = 0;

    for i in lo..=hi {
        if arr[i] <= pivot {
            arr[right] = arr[i];
            if left != right {
                arr[left] = arr[right];
            }
            left += 1;
            right += 1;
        } else {
            swap[swap_pos] = arr[i];
            swap_pos += 1;
        }
    }

    (left - lo, swap_pos)
}

fn fluxsort_u32_recursive(arr: &mut [u32], swap: &mut [u32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 96 {
        insertion_sort_u32(&mut arr[lo..=hi]);
        return;
    }

    let (left_size, right_size) = flux_partition_u32(arr, swap, lo, hi);

    if left_size < len / 16 || right_size < len / 16 {
        insertion_sort_u32(&mut arr[lo..=hi]);
        return;
    }

    if left_size > 0 {
        fluxsort_u32_recursive(arr, swap, lo, lo + left_size - 1);
    }

    if right_size > 0 {
        let swap_lo = lo + left_size;
        let swap_hi = lo + left_size + right_size - 1;
        
        unsafe {
            ptr::copy_nonoverlapping(swap.as_ptr(), arr.as_mut_ptr().add(swap_lo), right_size);
        }
        
        fluxsort_u32_recursive(arr, swap, swap_lo, swap_hi);
    }
}

pub fn fluxsort_u32(arr: &mut [u32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let swap_size = len / 2;
    let mut swap = vec![0; swap_size];

    fluxsort_u32_recursive(arr, &mut swap, 0, len - 1);
}

pub fn fluxsort_i64(arr: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut buf: Vec<u64> = arr.iter().map(|&x| x as u64 ^ (1u64 << 63)).collect();
    fluxsort_u64(&mut buf);
    buf.iter_mut().enumerate().for_each(|(i, val)| {
        arr[i] = (*val ^ (1u64 << 63)) as i64;
    });
}

fn fluxsort_u64_recursive(arr: &mut [u64], swap: &mut [u64], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 96 {
        insertion_sort_u64(&mut arr[lo..=hi]);
        return;
    }

    let (left_size, right_size) = flux_partition_u64(arr, swap, lo, hi);

    if left_size < len / 16 || right_size < len / 16 {
        insertion_sort_u64(&mut arr[lo..=hi]);
        return;
    }

    if left_size > 0 {
        fluxsort_u64_recursive(arr, swap, lo, lo + left_size - 1);
    }

    if right_size > 0 {
        let swap_lo = lo + left_size;
        let swap_hi = lo + left_size + right_size - 1;
        
        unsafe {
            ptr::copy_nonoverlapping(swap.as_ptr(), arr.as_mut_ptr().add(swap_lo), right_size);
        }
        
        fluxsort_u64_recursive(arr, swap, swap_lo, swap_hi);
    }
}

pub fn fluxsort_u64(arr: &mut [u64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let swap_size = len / 2;
    let mut swap = vec![0; swap_size];

    fluxsort_u64_recursive(arr, &mut swap, 0, len - 1);
}

fn flux_partition_u64(arr: &mut [u64], swap: &mut [u64], lo: usize, hi: usize) -> (usize, usize) {
    let len = hi - lo + 1;
    
    let mid = lo + (hi - lo) / 2;
    let pivot = if len < 2048 {
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
        median3_u64(&[m1, m2, m3], 0, 1, 2)
    } else {
        let sample_size = ((len as f64).cbrt() as usize).max(32).min(1024);
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push(arr[lo + i * step]);
        }
        samples.sort_unstable();
        samples[sample_size / 2]
    };

    let mut left = lo;
    let mut right = lo;
    let mut swap_pos = 0;

    for i in lo..=hi {
        if arr[i] <= pivot {
            arr[right] = arr[i];
            if left != right {
                arr[left] = arr[right];
            }
            left += 1;
            right += 1;
        } else {
            swap[swap_pos] = arr[i];
            swap_pos += 1;
        }
    }

    (left - lo, swap_pos)
}

fn median3_u64(arr: &[u64], a: usize, b: usize, c: usize) -> u64 {
    let (x, y, z) = (arr[a], arr[b], arr[c]);
    let x_gt_y = (x > y) as u64;
    let x_gt_z = (x > z) as u64;
    let y_gt_z = (y > z) as u64;
    [x, y, z][((x_gt_y == x_gt_z) as usize) + ((x_gt_z ^ y_gt_z) as usize)]
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

pub fn fluxsort_str(arr: &mut [String]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    fluxsort_str_recursive(arr, 0, len - 1);
}

fn fluxsort_str_recursive(arr: &mut [String], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 96 {
        insertion_sort_str(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + (hi - lo) / 2;
    let pivot = arr[mid].clone();
    
    let mut left = lo;
    let mut right = hi;

    while left <= right {
        while left <= right && arr[left] < pivot {
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

    if lo < right {
        fluxsort_str_recursive(arr, lo, right);
    }
    if left < hi {
        fluxsort_str_recursive(arr, left, hi);
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

pub enum FluxSortArray<'a> {
    I32(&'a mut [i32]),
    U32(&'a mut [u32]),
    I64(&'a mut [i64]),
    U64(&'a mut [u64]),
    String(&'a mut [String]),
}

pub fn fluxsort(arr: FluxSortArray) {
    match arr {
        FluxSortArray::I32(a) => fluxsort_i32(a),
        FluxSortArray::U32(a) => fluxsort_u32(a),
        FluxSortArray::I64(a) => fluxsort_i64(a),
        FluxSortArray::U64(a) => fluxsort_u64(a),
        FluxSortArray::String(a) => fluxsort_str(a),
    }
}

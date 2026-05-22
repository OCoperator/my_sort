use std::ptr;

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

#[inline(always)]
fn insertion_sort_i64(arr: &mut [i64]) {
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

#[inline(always)]
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

fn merge_i32(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let left_len = mid - lo + 1;
    let right_len = hi - mid;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid + 1], right.as_mut_ptr(), right_len);
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

fn quadsort_i32_recursive(arr: &mut [i32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_i32(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_i32_recursive(arr, lo, mid - 1);
    quadsort_i32_recursive(arr, mid, hi);

    merge_i32(arr, lo, mid - 1, hi);
}

pub fn quadsort_i32(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quadsort_i32_recursive(arr, 0, len - 1);
}

fn merge_u32(arr: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let left_len = mid - lo + 1;
    let right_len = hi - mid;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid + 1], right.as_mut_ptr(), right_len);
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

fn quadsort_u32_recursive(arr: &mut [u32], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_u32(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_u32_recursive(arr, lo, mid - 1);
    quadsort_u32_recursive(arr, mid, hi);

    merge_u32(arr, lo, mid - 1, hi);
}

pub fn quadsort_u32(arr: &mut [u32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quadsort_u32_recursive(arr, 0, len - 1);
}

fn merge_i64(arr: &mut [i64], lo: usize, mid: usize, hi: usize) {
    let left_len = mid - lo + 1;
    let right_len = hi - mid;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid + 1], right.as_mut_ptr(), right_len);
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

fn quadsort_i64_recursive(arr: &mut [i64], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_i64(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_i64_recursive(arr, lo, mid - 1);
    quadsort_i64_recursive(arr, mid, hi);

    merge_i64(arr, lo, mid - 1, hi);
}

pub fn quadsort_i64(arr: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quadsort_i64_recursive(arr, 0, len - 1);
}

fn merge_u64(arr: &mut [u64], lo: usize, mid: usize, hi: usize) {
    let left_len = mid - lo + 1;
    let right_len = hi - mid;

    let mut left = Vec::with_capacity(left_len);
    let mut right = Vec::with_capacity(right_len);

    unsafe {
        ptr::copy_nonoverlapping(&arr[lo], left.as_mut_ptr(), left_len);
        ptr::copy_nonoverlapping(&arr[mid + 1], right.as_mut_ptr(), right_len);
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

fn quadsort_u64_recursive(arr: &mut [u64], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_u64(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_u64_recursive(arr, lo, mid - 1);
    quadsort_u64_recursive(arr, mid, hi);

    merge_u64(arr, lo, mid - 1, hi);
}

pub fn quadsort_u64(arr: &mut [u64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quadsort_u64_recursive(arr, 0, len - 1);
}

pub fn quadsort_str(arr: &mut [String]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quadsort_str_recursive(arr, 0, len - 1);
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

fn merge_str(arr: &mut [String], lo: usize, mid: usize, hi: usize) {
    let left_len = mid - lo + 1;
    let right_len = hi - mid;

    let mut left: Vec<String> = Vec::with_capacity(left_len);
    let mut right: Vec<String> = Vec::with_capacity(right_len);

    for i in 0..left_len {
        left.push(arr[lo + i].clone());
    }
    for i in 0..right_len {
        right.push(arr[mid + 1 + i].clone());
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = lo;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn quadsort_str_recursive(arr: &mut [String], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 32 {
        insertion_sort_str(&mut arr[lo..=hi]);
        return;
    }

    let mid = lo + len / 2;
    quadsort_str_recursive(arr, lo, mid - 1);
    quadsort_str_recursive(arr, mid, hi);

    merge_str(arr, lo, mid - 1, hi);
}

pub enum QuadSortArray<'a> {
    I32(&'a mut [i32]),
    U32(&'a mut [u32]),
    I64(&'a mut [i64]),
    U64(&'a mut [u64]),
    String(&'a mut [String]),
}

pub fn quadsort(arr: QuadSortArray) {
    match arr {
        QuadSortArray::I32(a) => quadsort_i32(a),
        QuadSortArray::U32(a) => quadsort_u32(a),
        QuadSortArray::I64(a) => quadsort_i64(a),
        QuadSortArray::U64(a) => quadsort_u64(a),
        QuadSortArray::String(a) => quadsort_str(a),
    }
}

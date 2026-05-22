fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn median3<T: Ord>(arr: &mut [T], a: usize, b: usize, c: usize) -> usize {
    if arr[a] < arr[b] {
        if arr[b] < arr[c] { b } else if arr[a] < arr[c] { c } else { a }
    } else {
        if arr[a] < arr[c] { a } else if arr[b] < arr[c] { c } else { b }
    }
}

fn pseudomedian_of_nine<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
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

    let m1 = median3(arr, a, b, c);
    let m2 = median3(arr, d, e, f);
    let m3 = median3(arr, g, h, i);
    median3(arr, m1, m2, m3)
}

fn fulcrum_partition<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let len = hi - lo + 1;
    if len <= 1 {
        return lo;
    }

    let pivot_idx = if len < 2048 {
        pseudomedian_of_nine(arr, lo, hi)
    } else if len < 65536 {
        let mid = lo + (hi - lo) / 2;
        let quarter1 = lo + (mid - lo) / 2;
        let quarter3 = mid + (hi - mid) / 2;
        let m1 = median3(arr, lo, quarter1, mid);
        let m2 = median3(arr, mid, quarter3, hi);
        median3(arr, m1, mid, m2)
    } else {
        let sample_size = (len as f64).cbrt() as usize;
        let step = len / sample_size;
        let mut samples = Vec::with_capacity(sample_size);
        for i in 0..sample_size {
            samples.push((lo + i * step, arr[lo + i * step].clone()));
        }
        samples.sort_by(|a, b| a.1.cmp(&b.1));
        samples[sample_size / 2].0
    };

    arr.swap(lo, pivot_idx);
    let pivot = arr[lo].clone();
    let (mut head, mut tail) = (lo, hi);

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

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn is_reverse_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

fn reverse<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

fn analyze_segment<T: Ord>(arr: &[T], start: usize, end: usize) -> f64 {
    let len = end - start;
    if len <= 1 {
        return 1.0;
    }

    let mut ordered = 0;
    for i in start..end - 1 {
        if arr[i] <= arr[i + 1] {
            ordered += 1;
        }
    }

    ordered as f64 / (len - 1) as f64
}

fn crumsort_recursive<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize) {
    let len = hi - lo + 1;

    if len <= 24 {
        insertion_sort(&mut arr[lo..=hi]);
        return;
    }

    if len <= 128 {
        let sorted = is_sorted(&arr[lo..=hi]);
        if sorted {
            return;
        }

        let reversed = is_reverse_sorted(&arr[lo..=hi]);
        if reversed {
            reverse(&mut arr[lo..=hi]);
            return;
        }
    }

    if len > 128 {
        let segment_size = len / 4;
        let mut max_ordered: f64 = 0.0;
        for i in 0..4 {
            let start = lo + i * segment_size;
            let end = if i == 3 { hi + 1 } else { start + segment_size };
            let ordered_ratio = analyze_segment(arr, start, end);
            max_ordered = max_ordered.max(ordered_ratio);
        }

        if max_ordered > 0.5 {
            insertion_sort(&mut arr[lo..=hi]);
            return;
        }
    }

    let pivot_pos = fulcrum_partition(arr, lo, hi);

    let left_size = pivot_pos - lo;
    let right_size = hi - pivot_pos;
    let total_size = len;

    if left_size < total_size / 16 || right_size < total_size / 16 {
        insertion_sort(&mut arr[lo..=hi]);
        return;
    }

    if pivot_pos > lo {
        crumsort_recursive(arr, lo, pivot_pos - 1);
    }

    if pivot_pos < hi {
        crumsort_recursive(arr, pivot_pos + 1, hi);
    }
}

pub fn crumsort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    crumsort_recursive(arr, 0, len - 1);
}

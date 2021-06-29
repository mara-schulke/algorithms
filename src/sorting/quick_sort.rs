pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    quick_sort_bound(arr, 0, arr.len() as isize - 1);
}

fn quick_sort_bound<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if high <= low {
        return;
    }

    let pivot = partition(arr, low, high);

    quick_sort_bound(arr, low, pivot - 1);
    quick_sort_bound(arr, pivot + 1, high);
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high;

    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;

            arr.swap(i as usize, j as usize);
        }
    }

    i += 1;

    arr.swap(i as usize, pivot as usize);

    i
}

crate::test_sort!(quick_sort);

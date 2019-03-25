pub fn quick_sort(arr: &mut [u8]) {
    qs(arr, 0, (arr.len() - 1) as isize);
}

fn qs(arr: &mut [u8], lower: isize, upper: isize) {
    if upper > lower {
        let pivot = partition(arr, lower, upper);

        qs(arr, lower, pivot - 1);
        qs(arr, pivot + 1, upper);
    }
}

fn partition(arr: &mut [u8], lower: isize, upper: isize) -> isize {
    let pivot = arr[upper as usize];

    let mut i = lower - 1;
    for j in lower..upper {
        if arr[j as usize] <= pivot {
            i += 1;

            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, upper as usize);

    i + 1
}

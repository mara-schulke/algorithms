//! not workin

pub fn heap_sort(arr: &mut [u8]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    // for (i = n; i >= 2; i--)
    // {
    // 	temp = a[i];
    // 	a[i] = a[1];
    // 	a[1] = temp;
    // 	max_heapify(a, 1, i - 1);
    // }

    for i in (arr.len()..2).rev() {
        arr.swap(i, 1);

        heapify(arr, 1, i - 1);
    }
}

/// Convert `arr` into a max heap.
fn heapify(arr: &mut [u8], index: usize, heap_size: usize) {
    let current = arr[index];

    let mut left = 2 * index;
    let mut right = 2 * index + 1;

    while left <= heap_size {
        if left < heap_size && arr[left] < arr[right] {
            right += 1;
        }

        if current > arr[left] {
            break;
        } else {
            arr[left / 2] = arr[left];
            left *= 2;
        }
    }

    arr[left / 2] = current;
}

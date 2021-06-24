pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut swapped: bool = true;

    while swapped {
        swapped = false;

        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                swapped = true;

                arr.swap(i - 1, i);
            }
        }
    }
}

crate::test_sort!(bubble_sort);

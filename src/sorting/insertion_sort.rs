pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    self::optimized::insertion_sort(arr);
}

mod traditional {
    #[allow(dead_code)]
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        for unsorted in 1..arr.len() {
            let mut i = unsorted;

            while i > 0 && arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                i -= 1;
            }
        }
    }

    crate::test_sort!(insertion_sort);
}

mod optimized {
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        (1..arr.len()).for_each(|unsorted| {
            let target_index = arr[..unsorted]
                .binary_search(&arr[unsorted])
                .into_ok_or_err();
            arr[target_index..=unsorted].rotate_right(1);
        });
    }

    crate::test_sort!(insertion_sort);
}

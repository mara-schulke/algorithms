pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for unsorted in 0..arr.len() {
        let min = arr[unsorted..]
            .iter()
            .enumerate()
            .min_by_key(|i| i.1)
            .map(|(i, _)| i + unsorted)
            .unwrap();

        arr.swap(unsorted, min);
    }
}

crate::test_sort!(selection_sort);

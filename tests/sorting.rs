extern crate rand;
extern crate algorithms;

fn is_sorted(arr: &[u8]) -> bool {
    if arr.is_empty() { return true; }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > &item {
            return false;
        }

        prev = &item;
    }

    true
}

mod bubble_sort {
    #[test]
    fn unsorted() {
        let mut arr = vec![3, 124, 2, 53, 5, 4, 2, 6, 23, 124, 1, 43];

        algorithms::sorting::bubble_sort(&mut arr);

        assert!(super::is_sorted(&arr));
    }

    #[test]
    fn reversed() {
        let mut arr = vec![54, 32, 21, 1];

        algorithms::sorting::bubble_sort(&mut arr);

        assert!(super::is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];

        algorithms::sorting::bubble_sort(&mut arr);

        assert!(super::is_sorted(&arr));
    }
}

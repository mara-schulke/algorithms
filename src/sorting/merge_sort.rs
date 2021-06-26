#![allow(dead_code)]

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    self::top_down::merge_sort(arr);
}

mod top_down {
    use super::merge::merge;

    pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
        fn merge_sort_bound<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
            if low == high || low + 1 == high || high < low {
                return;
            }

            let mid = (low + high) / 2;

            merge_sort_bound(arr, low, mid);
            merge_sort_bound(arr, mid, high);

            merge(arr, low, mid, high);
        }

        merge_sort_bound(arr, 0, arr.len());
    }

    crate::test_sort!(merge_sort);
}

mod bottom_up {
    use std::cmp::min;
    use super::merge::merge;

    pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
        let len = arr.len();
        let mut group_size = 2;

        if len < group_size {
            merge(arr, 0, len / 2, len);
            return;
        }

        while group_size <= len {
            let group_count = {
                if len % group_size == 0 {
                    len / group_size
                } else {
                    len / group_size + 1
                }
            };

            for group in 0..group_count {
                let low = group_size * group;
                let mid = min(low + group_size / 2, len);
                let high = min(low + group_size, len);

                merge(arr, low, mid, high);
            }

            if group_count == 2 {
                merge(arr, 0, group_size, len);
            }

            group_size *= group_size;
        }
    }

    crate::test_sort!(merge_sort);
}

mod merge {
    pub fn merge<T: Ord + Copy>(arr: &mut [T], low: usize, mid: usize, high: usize) {
        assert!(arr[low..mid].is_sorted());
        assert!(arr[mid..high].is_sorted());

        let left = &arr[low..mid].iter().copied().collect::<Vec<T>>();
        let right = &arr[mid..high].iter().copied().collect::<Vec<T>>();

        let mut li = 0;
        let mut ri = 0;

        while li < left.len() && ri < right.len() {
            if left[li] <= right[ri] {
                arr[low + li + ri] = left[li];
                li += 1;
            } else {
                arr[low + li + ri] = right[ri];
                ri += 1;
            }
        }

        while li < left.len() {
            arr[low + li + ri] = left[li];
            li += 1;
        }

        while ri < right.len() {
            arr[low + li + ri] = right[ri];
            ri += 1;
        }
    }

    macro_rules! merge_tests {
        ($($name:ident: $value:expr,)*) => {
            #[cfg(test)]
            mod test {
                use super::merge;

                $(
                    #[test]
                    fn $name() {
                        let mut arr = $value;
                        let mut res = arr.clone();

                        res.sort_unstable();

                        merge(&mut arr, 0, $value.len() / 2, $value.len());

                        assert_eq!(arr, res);
                    }
                )*
            }
        }
    }

    merge_tests! {
        empty: [] as [u32; 0],
        single: [1],
        equaly_long: [1, 3, 5, 2, 4, 6],
        large_values: [3, 5, 3001312, 2, 6, 1381242],
        only_large_values: [321312, 11512522, 321421421, 6414, 215912, 414342],
        sorted: [1, 2, 3, 4, 5, 6],
        unequal_amount: [2, 32, 51, 214, 318, 2412, 21431],
        wrapped_by_left: [0, 10, 1, 2, 3],
        wrapped_by_right: [1, 2, 0, 3, 10],
        strings: ["d", "c"],
    }
}

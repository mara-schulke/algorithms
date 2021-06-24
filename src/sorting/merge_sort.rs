pub fn merge_sort<T: Ord>(arr: &mut Vec<T>) {
    unimplemented!();
}

mod top_down {
    pub fn merge_sort<T: Ord>(arr: &mut Vec<T>) {
        unimplemented!();
    }

    crate::test_sort!(merge_sort);
}

mod bottom_up {
    pub fn merge_sort<T: Ord>(arr: &mut Vec<T>) {
        unimplemented!();
    }

    crate::test_sort!(merge_sort);
}

mod merge {
    pub fn merge<'a, T: Ord>(left: &'a Vec<T>, right: &'a Vec<T>) -> Vec<&'a T> {
        assert!(left.is_sorted());
        assert!(right.is_sorted());

        let mut res = vec![];

        let mut li = 0;
        let mut ri = 0;

        while li < left.len() && ri < right.len() {
            res.push({
                if left[li] <= right[ri] {
                    li += 1;
                    &left[li - 1]
                } else {
                    ri += 1;
                    &right[ri - 1]
                }
            });
        }

        while li < left.len() {
            res.push(&left[li]);
            li += 1;
        }

        while ri < right.len() {
            res.push(&right[ri]);
            ri += 1;
        }

        res
    }

    macro_rules! merge_tests {
        ($($name:ident: $value:expr,)*) => {
            #[cfg(test)]
            mod test {
                use super::merge;

                $(
                    #[test]
                    fn $name() {
                        let (left, right) = $value;
                        let mut res = left
                                .iter()
                                .chain(right.iter())
                                .collect::<Vec<&u32>>();

                        res.sort_unstable();

                        assert_eq!(merge(&left, &right), res);
                    }
                )*
            }
        }
    }

    merge_tests! {
        empty: (&vec![], &vec![]),
        equaly_long: (&vec![1, 3, 5], &vec![2, 4, 6]),
        large_values: (&vec![3, 5, 3001312], &vec![2, 6, 1381242]),
        only_large_values: (&vec![321312, 11512522, 321421421], &vec![6414, 215912, 414342]),
        sorted: (&vec![1, 2, 3], &vec![4, 5, 6]),
        only_right: (&vec![], &vec![1, 2, 3, 4]),
        only_left: (&vec![32, 51, 315], &vec![]),
        unequal_amount: (&vec![2, 32, 51, 214, 318, 2412], &vec![0, 21431]),
        wrapped_by_left: (&vec![0, 10], &vec![1, 2, 3]),
        wrapped_by_right: (&vec![1, 2, 3], &vec![0, 10]),
    }
}

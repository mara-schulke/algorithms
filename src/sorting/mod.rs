mod bubble_sort;
mod bucket_sort;
mod counting_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod slow_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::bucket_sort::bucket_sort;
pub use self::counting_sort::counting_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::quick_sort::quick_sort;
pub use self::radix_sort::radix_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;
pub use self::slow_sort::slow_sort;

#[macro_export]
macro_rules! gen_sort_tests {
    ($alg:ident, $($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (mut input, expected) = $value;
                $alg(&mut input);
                assert_eq!(input, expected);
            }
        )*
    }
}

#[macro_export]
macro_rules! test_sort (
    ($alg:ident) => {
        #[cfg(test)]
        mod test {
            use super::$alg;
            use crate::gen_sort_tests;

            gen_sort_tests! {
                $alg,
                empty: (
                    vec![] as Vec<u8>,
                    vec![] as Vec<u8>
                ),
                single: (
                    vec![1], vec![1]
                ),
                sorted: (
                    vec![1, 2, 3, 4],
                    vec![1, 2, 3, 4]
                ),
                slightly_unsorted: (
                    vec![1, 3, 2, 4],
                    vec![1, 2, 3, 4]
                ),
                even_unsorted: (
                    vec![6, 21, 1, 2],
                    vec![1, 2, 6, 21]
                ),
                even_sorted: (
                    vec![1, 2, 6, 21],
                    vec![1, 2, 6, 21]
                ),
                odd_unsorted: (
                    vec![1, 2, 0],
                    vec![0, 1, 2]
                ),
                odd_sorted: (
                    vec![0, 1, 2],
                    vec![0, 1, 2]
                ),
                large_numbers_sorted: (
                    vec![124122, 401291, 4192421, 22131293, 124214124],
                    vec![124122, 401291, 4192421, 22131293, 124214124]
                ),
                large_numbers_unsorted: (
                    vec![124122, 24192421, 2131293, 124214124, 401291],
                    vec![124122, 401291, 2131293, 24192421, 124214124]
                ),
                strings: (
                    vec!["a", "b", "d", "c"],
                    vec!["a", "b", "c", "d"],
                ),
                complex_strings: (
                    vec!["a2", "a1", ".", "1c"],
                    vec![".", "1c", "a1", "a2"],
                ),
            }
        }
    }
);


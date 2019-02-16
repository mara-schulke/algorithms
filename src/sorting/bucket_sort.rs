pub use super::insertion_sort::insertion_sort;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn bucket_sort(arr: &mut [u8]) {
    insertion_sort(arr);
}

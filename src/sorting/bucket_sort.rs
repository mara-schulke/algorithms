pub use super::insertion_sort::insertion_sort;

pub fn bucket_sort(arr: &mut [u8]) {
    insertion_sort(arr);
}

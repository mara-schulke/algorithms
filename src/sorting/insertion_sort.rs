#[allow(dead_code)]
#[allow(unused_variables)]
pub fn insertion_sort(arr: &mut [u8]) {
    for i in 1..arr.len() {
        let mut j: usize = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);

            j -= 1;
        }
    }
}

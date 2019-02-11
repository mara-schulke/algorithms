#[allow(dead_code)]
#[allow(unused_variables)]
pub fn gnome_sort(arr: &mut [u8]) {
    let mut i: usize = 1;

    while i < arr.len() {
        if arr[i - 1] <= arr[i] {
            i += 1;
        } else {
            let temp = arr[i - 1];
            arr[i - 1] = arr[i];
            arr[i] = temp;

            i -= 1;

            if i == 0 {
                i = 1;
            }
        }
    }
}

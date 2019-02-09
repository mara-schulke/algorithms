#[allow(dead_code)]
#[allow(unused_variables)]
pub fn bubble_sort(arr: &mut [u8]) {
    loop {
        let mut swapped: bool = false;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                swapped = true;

                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn bubble_sort(arr: &mut [u8]) {
    loop {
        let mut swapped: bool = false;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                swapped = true;

                arr.swap(i, i + 1);
            }
        }

        if !swapped {
            break;
        }
    }
}

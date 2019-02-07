use std::mem;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn bubble_sort(arr: &mut [u8]) {
    loop {
        let mut swapped: bool = false;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                swapped = true;

                // arr[i] = arr[i + 1];
                // arr[i + 1] = temp;
                let mut cur: &u8 = &mut arr[i];
                let mut next: &u8 = &mut arr[i + 1];

                mem::swap(&mut cur, &mut next);
            }
        }

        if !swapped {
            break;
        }
    }
}

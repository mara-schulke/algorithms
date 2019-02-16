#[allow(dead_code)]
#[allow(unused_variables)]
pub fn quick_sort(arr: &mut [u8]) {
    // while 0 < arr.

    partition(arr);

    // partionieren
    // swap
    // again
}

fn partition(arr: &mut [u8]) {
    let pivot: usize = arr.len() / 2;

    println!("OLA {}", pivot)
}

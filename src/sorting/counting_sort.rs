#[allow(dead_code)]
#[allow(unused_variables)]
pub fn counting_sort(arr: &mut [u8], maxval: usize) {
    let mut occurences: Vec<u8> = vec![0; maxval + 1];
    let mut index: Vec<u8> = vec![0; maxval + 1];

    for &i in arr.iter() {
        occurences[i as usize] += 1;
    }

    let mut calculated_index: u8 = 0;

    for j in 0..maxval + 1 {
        calculated_index += occurences[j];

        index[j] += calculated_index;
    }

    let mut sorted: Vec<u8> = vec![0; arr.len()];

    for k in arr.iter() {
        sorted[(index[*k as usize] - 1) as usize] = *k;

        index[*k as usize] -= 1;
    }

    for x in 0..arr.len() {
        arr[x] = sorted[x];
    }
}

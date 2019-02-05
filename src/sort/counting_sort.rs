#[allow(dead_code)]
#[allow(unused_variables)]
pub fn counting_sort(arr: &mut [u8], maxval: usize) {
    let mut occurances = vec![0; maxval + 1];
    let mut index = vec![0; maxval + 1];

    for i in arr {
        occurances[*i as usize] += 1;
        println!("{}", occurances[*i as usize]);
        // println!("{}", count[*i as usize - 1])
    }

    for occured in occurances {
        index[*occured as usize] = 0x04;
    }
}

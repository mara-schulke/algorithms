#[allow(dead_code)]
#[allow(unused_variables)]
pub fn quick_sort(arr: &mut [u8]) {
    let len: usize = arr.len();

    if len <= 1 {
        return;
    }

    let pivot: usize = len / 2;

    let mut greater: Vec<u8> = vec![];
    let mut smaller: Vec<u8> = vec![];

    for (index, element) in arr.iter().enumerate() {
        if index == (len / 2) {
            continue;
        }

        if element > &arr[pivot] {
            greater.push(*element);
        } else {
            smaller.push(*element);
        }
    }

    // quick_sort(&mut smaller);
    // quick_sort(&mut greater);

    {
        println!("SMALLER {:?}", smaller);
        println!("PIVOT {:?}", arr[pivot]);
        println!("GREATER {:?}", greater);
        println!("");
    }

    // let mut i: usize = 0;

    // while i < smaller.len() {
    //     arr[i] = smaller[i];
    //     i += 1;
    // }

    // skip the pivot
    // i += 1;

    // while i < greater.len() {
    //     arr[i] = smaller[i];
    //     i += 1;
    // }

    // println!("{:?}", greater);
    // println!("{:?}", smaller);

    // [ element for element in ARRAY[1:] if element > PIVOT ]

    // LESSER = [ element for element in ARRAY[1:] if element <= PIVOT ]
    // return quick_sort(LESSER) + [PIVOT] + quick_sort(GREATER)
}

// fn partition(arr: &mut [u8], lower: usize, upper: usize) {}

// fn qs(arr: &mut [u8], lower: usize, upper: usize) {
//     if lower < upper {}
// }

pub fn radix_sort(arr: &mut [u8]) {
    let max: usize = arr.iter().cloned().fold(0, u8::max).to_string().len();
    let mut index: usize;
    let mut buckets: Vec<Vec<u8>>;

    for i in 0..max + 1 {
        buckets = vec![vec![]; arr.len()];
        index = 0;

        for j in 0..arr.len() {
            let digit: usize = get_digit(arr[j], i + 1);
            buckets[digit].push(arr[j]);
        }

        for t in 0..buckets.len() {
            if buckets[t].len() > 0 {
                for m in 0..buckets[t].len() {
                    arr[index] = buckets[t][m];
                    index += 1;
                }
            }
        }
    }
}

fn get_digit(mut num: u8, mut nth: usize) -> usize {
    let mut value: u8 = 0;

    while nth > 0 {
        nth -= 1;

        value = num % 10;
        num = (num - value) / 10;
    }

    value as usize
}

pub fn selection_sort(arr: &mut [u8]) {
    for i in 0..arr.len() - 1 {
        let mut index_of_minimum: usize = 0;

        for j in i..arr.len() {
            if arr[j] < arr[index_of_minimum] {
                index_of_minimum = j;
            }
        }

        if index_of_minimum != i {
            let temp = arr[index_of_minimum];
            arr[index_of_minimum] = arr[i];
            arr[i] = temp;
        }
    }
}

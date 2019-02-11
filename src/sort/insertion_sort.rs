#[allow(dead_code)]
#[allow(unused_variables)]
pub fn insertion_sort(arr: &mut [u8]) {
	for i in 1..arr.len() {
		let mut j: u16 = i as u16;

		while j > 0 && arr[j as usize] < arr[(j - 1) as usize] {
			let temp = arr[j as usize];
			arr[j as usize] = arr[(j - 1) as usize];
			arr[(j - 1) as usize] = temp;

			j -= 1;
		}
	}
}

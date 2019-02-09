#[allow(dead_code)]
#[allow(unused_variables)]
pub fn insertion_sort(arr: &mut [u8]) {
	for i in 1..arr.len() {
		println!("{}", arr[i as usize]);

		let mut j: u8 = i as u8;

		while j > 1 && arr[(j - 1) as usize] > arr[i] {
			arr[j as usize] = arr[(j - 1) as usize];

			j -= 1;
		}
		arr[j as usize] = arr[i];
	}
}

// extern crate rand;

// use rand::Rng;

mod sort;

fn main() {
	/* gen an array containing random numbers */
	let mut random_array: [u8; 20] = [
		214, 12, 145, 135, 32, 0, 255, 233, 231, 123, 224, 123, 55, 32, 43, 32, 52, 43, 21, 85,
	]; //rand::thread_rng().gen();

	println!("unsorted {:?}", random_array);

	sort::counting_sort(&mut random_array, 255 as usize);

	println!("sorted {:?}", random_array);
}

extern crate rand;

use rand::Rng;
use std::u8::MAX;

mod sort;

fn main() {
	/* gen an array containing random numbers */
	let mut random_array: [u8; 20] = rand::thread_rng().gen();

	println!("unsorted {:?}", random_array);

	sort::counting_sort(&mut random_array, MAX as usize);

	println!("sorted {:?}", random_array);
}

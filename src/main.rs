extern crate rand;

use rand::Rng;
// use std::u8::MAX;

mod sorting;

fn main() {
    /* gen an array containing random numbers */
    let mut random_array: [u8; 20] = rand::thread_rng().gen();

    println!("unsorted {:?}", random_array);

    sorting::heap_sort(&mut random_array);

    println!("sorted {:?}", random_array);
}

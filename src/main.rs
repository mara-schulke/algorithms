extern crate rand;

use rand::Rng;

mod sort;

fn main() {
    /* gen an array containing random numbers */
    let mut random_array: [u8; 20] = rand::thread_rng().gen();

    println!("unsorted {:?}", random_array);

    sort::insertion_sort(&mut random_array);

    println!("sorted {:?}", random_array);
}

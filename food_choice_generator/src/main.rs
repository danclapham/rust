extern crate rand;

use rand::Rng;

fn main() {
	let foods = ["Pizza", "Fried Chicken", "Curry", "Chinese", "Mexican", "Italian"];
	let mut rng = rand::thread_rng();
	let x = rng.gen_range(1, foods.len());
	println!("Your food choice is {}", foods[x]);
}

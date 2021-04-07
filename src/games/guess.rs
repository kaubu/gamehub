// Guessing Game
use rand::Rng;
use read_input::prelude::*;

use crate::version::Version;

// Variables
const VERSION: Version = Version::new(0, 1, 0);

pub fn run() {
	println!("Guessing Game v{}", VERSION.display());
	
	let min = input::<i32>()
		.repeat_msg("Minimum: ")
		.err("That doesn't seem to be a number. Please try again.")
		.get();
	
	let max = input::<i32>()
		.repeat_msg("Maximum: ")
		.err("That doesn't seem to be a number. Please try again.")
		.get();

	let mut rng = rand::thread_rng();
	println!("rng: {}", rng.gen_range(min..(max+1)));
}

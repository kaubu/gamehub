// Guessing Game
use rand::Rng;
use read_input::prelude::*;

use crate::version::Version;

// Variables
const VERSION: Version = Version::new(1, 0, 0);

pub fn run() {
	println!("Guessing Game v{}\nType 'exit' to exit this game.", VERSION.format());
	
	let min = input::<i32>()
		.repeat_msg("Minimum: ")
		.err("That doesn't seem to be a number. Please try again.")
		.get();
	
	let max = input::<i32>()
		.repeat_msg("Maximum: ")
		.err("That doesn't seem to be a number. Please try again.")
		.get();

	let mut rng = rand::thread_rng();
	let answer = rng.gen_range(min..(max+1));
	// println!("rng: {}", answer); // For debug only

	loop {
		let attempt = input::<String>()
			.repeat_msg("Guess>> ")
			.err("That doesn't seem to be a number. Please try again.")
			.get();
		
		if (attempt == "exit") || (attempt == "quit") {
			println!("Thanks for playing!\n");
			break;
		}

		let attempt = attempt.parse::<i32>().unwrap();

		if attempt == answer {
			println!("Correct!\nThanks for playing!\n");
			break;
		} else if attempt > answer {
			println!("Too high!");
		} else if attempt < answer {
			println!("Too low!");
		}
	}
}

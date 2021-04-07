// Core
mod version;
mod games;

// Use
use version::Version;
use read_input::prelude::*;

// Constants
const VERSION: Version = Version::new(0, 1, 0);

// Structs
struct Game {
	name: &'static str, // Guessing Game
	command: &'static str, // gg
	run: fn() // guess::run()
}

impl Game {
	fn new(n: &'static str, c: &'static str, r: fn()) -> Game {
		Game {
			name: n,
			command: c,
			run: r
		}
	}

	fn run(&self) {
		(self.run)();
	}
}

// Type aliases
type GameList = Vec<Game>;

fn main() {
	println!("GameHub v{}\nType 'help' for a list of games.\nType 'exit' to exit GameHub.\n", VERSION.format());
	
	let games = vec![
		Game::new("Guessing Game", "gg", games::guess::run),
	];

	help(&games);

	loop {
		let input = input::<String>()
			.repeat_msg(">> ")
			.err("There was an error getting your input.")
			.get();
		
		if input == "help" {
			help(&games);
		}

		if input == "exit" || input == "quit" {
			break;
		}

		for game in games.iter() {
			if input == game.command {
				game.run();
			}
		}
	}
}

fn help(games: &GameList) {
	println!("Games:\nCommand - Game name");
	for game in games.iter() {
		println!("{}\t{}", game.command, game.name);
	}
	println!("");
}
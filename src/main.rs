// Core
mod game;
mod version;
mod games;

// Use
use game::Game;
use read_input::prelude::*;

type GameList = Vec<Game>;

fn main() {
	let games = vec![
		Game::new("Guessing Game", "gg", games::guess::run),
	];

	help(&games);

	loop {
		let input = input::<String>()
			.repeat_msg("Game: ")
			.err("There was an error getting your input.")
			.get();
		
		if input == "help" {
			help(&games);
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
		println!("{}\t\t{}", game.command, game.name);
	}
	println!("");
}
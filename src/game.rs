// pub struct Game<F>
// where
// 	F: Fn() -> ()
// {
// 	pub name: &'static str,
// 	pub launcher: &'static str, // What do you type to execute the game
// 	pub function: F,
// }

// impl<F> Game<F>
// where
// 	F: Fn()
// {
// 	pub fn run(&self) {
// 		(self.function)();
// 	}
// }

pub struct Game {
	pub name: &'static str, // Guessing Game
	pub command: &'static str, // gg
	pub run: fn() // guess::run()
}

impl Game {
	pub fn new(n: &'static str, c: &'static str, r: fn()) -> Game {
		Game {
			name: n,
			command: c,
			run: r
		}
	}

	pub fn run(&self) {
		(self.run)();
	}
}
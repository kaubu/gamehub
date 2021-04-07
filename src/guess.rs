// Guessing Game
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn seed() {
	let rng = ChaCha20Rng::from_entropy();
}

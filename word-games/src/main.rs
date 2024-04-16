use crate::common::choose_target_word;

mod common;
mod word_list;
mod wordle;

fn main() -> Result<(), word_list::Error> {
	let word_lists = word_list::load("wordle/dictionary.txt", "wordle/targets.txt")?;
	let mut wordle = wordle::Game::from_word_lists(&word_lists, 6);

	for _ in 0..8 {
		println!(
			"{:?}",
			wordle.guess(choose_target_word(&word_lists.targets))
		);
	}

	Ok(())
}

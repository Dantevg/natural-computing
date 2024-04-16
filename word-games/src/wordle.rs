use crate::{
	common::choose_target_word,
	word_list::{WordList, WordLists},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
	/// Letter is in the correct position.
	Correct,

	/// Letter appears in the word, but in a different position.
	Misplaced,

	/// Letter does not appear in the word at all.
	Incorrect,
}

#[derive(Debug)]
pub enum Response {
	/// List of hints, one for each letter in the guess.
	Hint(Vec<Status>),

	/// 🎉 You guessed the target word.
	Correct,

	/// You do not have any guesses left.
	TooManyGuesses,

	/// The guess is not a word in the dictionary (it may not even be of the
	/// right length).
	InvalidGuess,
}

#[derive(Debug)]
pub struct Game<'dict> {
	/// The list of all words that are considered valid. Contains potential
	/// target words and also words that will never be chosen as a target word.
	dictionary: &'dict WordList,

	/// The target word to be guessed.
	target: &'dict str,

	/// The number of guesses left. Before the first guess, this will be the
	/// maximum number of guesses. Calling [`guess`](Game::guess) is only
	/// accepted if [`guesses`](Game::guesses) is > 0.
	guesses: u32,
}

impl<'dict> Game<'dict> {
	#[must_use]
	pub fn new(dictionary: &'dict WordList, target: &'dict str, guesses: u32) -> Self {
		Self {
			dictionary,
			target,
			guesses,
		}
	}

	#[must_use]
	pub fn from_word_lists(word_lists: &'dict WordLists, guesses: u32) -> Self {
		Self {
			dictionary: &word_lists.dictionary,
			target: choose_target_word(&word_lists.targets),
			guesses,
		}
	}

	/// Guesses this word, which should be a lowercase ASCII word, and returns
	/// hints if it was a correct guess.
	pub fn guess(&mut self, guess: &str) -> Response {
		if self.guesses == 0 {
			return Response::TooManyGuesses;
		}

		if !self.dictionary.contains(guess) {
			return Response::InvalidGuess;
		}

		self.guesses -= 1;

		if guess == self.target {
			return Response::Correct;
		}

		Response::Hint(
			guess
				.as_bytes()
				.iter()
				.enumerate()
				.map(|(i, &c)| self.check_char(c, i))
				.collect(),
		)
	}

	/// Returns a single [Status] of this letter `c` in this position.
	///
	/// TODO: check that a repeated letter should actually be marked as
	/// misplaced multiple times, or only the amount of times it appears in the
	/// target word.
	#[must_use]
	fn check_char(&self, c: u8, index: usize) -> Status {
		let target = self.target.as_bytes();
		if target[index] == c {
			Status::Correct
		} else if target.contains(&c) {
			Status::Misplaced
		} else {
			Status::Incorrect
		}
	}
}

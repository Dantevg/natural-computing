use rand::{thread_rng, Rng};

use crate::word_list::WordList;

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

/// Returns a random word from the target list.
pub fn choose_target_word(targets_list: &WordList) -> &str {
	&targets_list[thread_rng().gen_range(0..targets_list.len())]
}

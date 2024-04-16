use core::{cmp::Ordering, fmt::Debug, ops::Index};
use std::{
	fs::File,
	io::{self, BufRead, BufReader},
	path::Path,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("words have inequal word lengths ({0} != {1})")]
	InequalLengths(u32, u32),

	#[error("word is too long: {0}")]
	TooLong(String),

	#[error(transparent)]
	Io(#[from] io::Error),
}

/// A list of words that are all of the same length.
///
/// Implementation inspired by https://lib.rs/crates/packed_str.
#[derive(Default)]
pub struct WordList {
	/// The length of every word in this [WordList].
	word_len: usize,

	/// The actual list of words. The length of this list is the number of words
	/// times [`word_len`].
	words: Vec<u8>,
}

impl Debug for WordList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WordList")
			.field("word_len", &self.word_len)
			.field(
				"words",
				&self
					.words
					.iter()
					.map(|&c| char::from_u32(u32::from(c)).unwrap())
					.collect::<String>(),
			)
			.finish()
	}
}

impl Index<usize> for WordList {
	type Output = str;

	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).unwrap_or_else(|| {
			panic!(
				"index {} out of bounds for size {} (word length {})",
				index,
				self.len(),
				self.word_len()
			)
		})
	}
}

impl WordList {
	/// Creates a new [WordList] from this slice. **Note that `slice` needs to
	/// be sorted!**
	#[must_use]
	pub fn from_slice<S: AsRef<str>>(slice: &[S]) -> Self {
		let n_words = slice.len();
		if n_words == 0 {
			return Self {
				word_len: 0,
				words: Vec::new(),
			};
		}
		let word_len = slice[0].as_ref().len();
		let mut words: Vec<u8> = Vec::with_capacity(n_words * word_len);

		for word in slice {
			words.extend_from_slice(word.as_ref().as_bytes());
		}

		Self { word_len, words }
	}

	#[must_use]
	pub fn get(&self, index: usize) -> Option<&str> {
		let start_idx = index * self.word_len();
		let end_idx = (index + 1) * self.word_len();

		let slice = self.words.get(start_idx..end_idx)?;
		let result = unsafe { std::str::from_utf8_unchecked(slice) };

		Some(result)
	}

	/// Returns whether `word` appears in this list.
	///
	/// Does a binary search through this sorted list. Implementation inspired
	/// by `[T]::binary_search_by`.
	#[must_use]
	pub fn contains(&self, word: &str) -> bool {
		if word.len() != self.word_len() || !word.is_ascii() {
			return false;
		}

		let mut left = 0;
		let mut right = self.len();

		while left < right {
			let mid = left + (right - left) / 2;
			let cmp = self[mid].cmp(word);
			left = if cmp == Ordering::Less { mid + 1 } else { left };
			right = if cmp == Ordering::Greater { mid } else { right };
			if cmp == Ordering::Equal {
				return true;
			}
		}

		false
	}

	#[must_use]
	#[inline]
	pub fn word_len(&self) -> usize {
		self.word_len
	}

	#[must_use]
	#[inline]
	pub fn len(&self) -> usize {
		self.words.len() / self.word_len()
	}

	#[must_use]
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.words.is_empty()
	}
}

#[derive(Debug)]
pub struct WordLists {
	pub dictionary: WordList,
	pub targets: WordList,
	pub word_length: u32,
}

pub fn load<P: AsRef<Path>>(dictionary_path: P, targets_path: P) -> Result<WordLists, Error> {
	let dictionary = load_list(dictionary_path)?;
	let targets = load_list(targets_path)?;

	let dict_word_length = dictionary.word_len();
	let target_word_length = targets.word_len();
	if dict_word_length != target_word_length {
		return Err(Error::InequalLengths(
			dict_word_length as u32,
			target_word_length as u32,
		));
	}

	Ok(WordLists {
		dictionary,
		targets,
		word_length: dict_word_length as u32,
	})
}

fn load_list<P: AsRef<Path>>(path: P) -> Result<WordList, Error> {
	let file = File::open(path)?;
	let file_size = file.metadata()?.len() as usize;

	let mut reader = BufReader::new(file);

	let mut first_word = String::new();
	let line_length = reader.read_line(&mut first_word)?;
	if line_length > u32::MAX as usize {
		return Err(Error::TooLong(first_word));
	}
	first_word.pop(); // remove newline character that was included by read_line

	let mut list = Vec::with_capacity(file_size / line_length);
	list.push(first_word);

	for word in reader.lines() {
		match word {
			Ok(w) if w.len() == line_length - 1 => list.push(w),
			Ok(w) => {
				return Err(Error::InequalLengths(
					(line_length - 1) as u32,
					w.len() as u32,
				))
			}
			Err(e) => return Err(e.into()),
		}
	}

	Ok(WordList::from_slice(&list))
}

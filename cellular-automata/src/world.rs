use imgref::Img;
use loop9::loop9_img;
use rand::prelude::*;

use crate::Cell;

pub struct World<const W: usize, const H: usize, C: Cell> {
	pub img: Img<Vec<C>>,
	pub wrap: bool,
}

impl<const W: usize, const H: usize, C: Cell + Default> Default for World<W, H, C> {
	fn default() -> Self {
		Self {
			img: Img::new(vec![C::default(); W * H], W, H),
			wrap: true,
		}
	}
}

impl<const W: usize, const H: usize, C: Cell> World<W, H, C> {
	pub fn from_fn<F>(function: F, wrap: bool) -> Self
	where
		F: FnMut(usize) -> C,
	{
		let buf = (0..(W * H)).map(function).collect();
		Self {
			img: Img::new(buf, W, H),
			wrap,
		}
	}

	pub fn draw(&self, frame: &mut [u8], frame_width: usize, scale: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width / scale;
			let y = i / frame_width / scale;

			if x < W && y < H {
				pixel.copy_from_slice(&self.img[(x, y)].colour());
			}
		}
	}

	pub fn convolve<F>(&mut self, mut rule: F)
	where
		F: FnMut([C; 9]) -> C,
	{
		if self.wrap {
			self.wrap_edges();
		}

		let mut new_img = self.img.clone();
		loop9_img(self.img.as_ref(), |x, y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.curr, mid.next, bot.prev, bot.curr,
				bot.next,
			];
			new_img[(x, y)] = rule(neighbourhood);
		});
		self.img = new_img;
	}

	pub fn metropolis<F>(&mut self, mut update: F)
	where
		F: FnMut(&Self, C, C, usize, usize) -> C,
	{
		for _ in 0..W * H {
			let mut rng = rand::thread_rng();
			let src_idx = rng.gen_range(0..(W * H));
			let dest_idx = *self.get_neighbours_idx(src_idx).choose(&mut rng).unwrap();
			let src = self.get_cell(src_idx);
			let dest = self.get_cell(dest_idx);

			if src != dest {
				// TODO: check that this update really needs to be sequential
				self.img[(dest_idx % W, dest_idx / W)] = update(self, src, dest, src_idx, dest_idx);
			}
		}
	}

	#[inline(always)]
	pub fn get_cell(&self, idx: usize) -> C {
		self.img[(idx % W, idx / W)]
	}

	#[inline(always)]
	pub fn get_cell_mut(&mut self, idx: usize) -> &mut C {
		&mut self.img[(idx % W, idx / W)]
	}

	pub fn get_neighbour(&self, cell_idx: usize, offset: isize) -> Option<C> {
		self.get_neighbour_idx(cell_idx, offset)
			.map(|idx| self.get_cell(idx))
	}

	pub fn get_neighbours_idx(&self, cell_idx: usize) -> Box<[usize]> {
		[
			self.get_neighbour_idx(cell_idx, -(W as isize) - 1),
			self.get_neighbour_idx(cell_idx, -(W as isize)),
			self.get_neighbour_idx(cell_idx, -(W as isize) + 1),
			self.get_neighbour_idx(cell_idx, -1),
			/* ignore self */
			self.get_neighbour_idx(cell_idx, 1),
			self.get_neighbour_idx(cell_idx, W as isize - 1),
			self.get_neighbour_idx(cell_idx, W as isize),
			self.get_neighbour_idx(cell_idx, W as isize + 1),
		]
		.into_iter()
		.filter_map(|idx| idx)
		.collect()
	}

	fn get_neighbour_idx(&self, cell_idx: usize, offset: isize) -> Option<usize> {
		if self.wrap {
			Some((cell_idx as isize + offset).rem_euclid((W * H) as isize) as usize)
		} else {
			cell_idx
				.checked_add_signed(offset)
				.filter(|idx| (0..(W * H)).contains(&idx))
		}
	}

	/// Wraps the edges of this [`World`] in such a way that this:
	///
	///     . . . . .
	///     . 1 2 3 .
	///     . 4 5 6 .
	///     . 7 8 9 .
	///     . . . . .
	///
	/// turns into this:
	///
	///     9 7 8 9 7   (top_row)
	///     3 1 2 3 1   (first_row)
	///     6 4 5 6 4
	///     9 7 8 9 7   (last_row)
	///     3 1 2 3 1   (bot_row)
	fn wrap_edges(&mut self) {
		let width = self.img.width();
		let height = self.img.height();
		let stride = self.img.stride();

		// Wrap vertical edges (left <-> right)
		for row in self.img.rows_mut() {
			row[0] = row[width - 2];
			row[width - 1] = row[1];
		}

		// Wrap horizontal edges (top <-> bottom)
		let top_row = 0; // start index of top padding row
		let first_row = stride; // start index of first actual row
		let last_row = (height - 2) * stride; // start index of last actual row
		let bot_row = (height - 1) * stride; // start index of bottom padding row

		let buf = self.img.buf_mut();
		buf.copy_within(first_row..(first_row + width), bot_row);
		buf.copy_within(last_row..(last_row + width), top_row);
	}
}

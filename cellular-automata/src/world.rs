use imgref::Img;
use loop9::loop9_img;
use rand::prelude::*;

use crate::Cell;

pub type Coord = (u32, u32);

pub struct World<const W: usize, const H: usize, C: Cell> {
	pub img: Img<Vec<C>>,
}

impl<const W: usize, const H: usize, C: Cell + Default> Default for World<W, H, C> {
	fn default() -> Self {
		Self {
			img: Img::new(vec![C::default(); W * H], W, H),
		}
	}
}

impl<const W: usize, const H: usize, C: Cell> World<W, H, C> {
	#[must_use]
	pub fn from_fn<F>(function: F) -> Self
	where
		F: FnMut(usize) -> C,
	{
		let buf = (0..(W * H)).map(function).collect();
		Self {
			img: Img::new(buf, W, H),
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
		self.wrap_edges();

		let mut new_img = self.img.clone();
		loop9_img(self.img.as_ref(), |x, y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, // comments to prevent auto-formatting
				mid.prev, mid.curr, mid.next, //
				bot.prev, bot.curr, bot.next, //
			];
			new_img[(x, y)] = rule(neighbourhood);
		});
		self.img = new_img;
	}

	pub fn metropolis<F>(&mut self, mut update: F)
	where
		F: FnMut(&Self, C, C, Coord, Coord) -> C,
	{
		let mut rng = rand::thread_rng();
		for _ in 0..W * H {
			let src_idx = (rng.gen_range(0..W as u32), rng.gen_range(0..H as u32));
			let dest_idx = *Self::get_neighbours_idx(src_idx).choose(&mut rng).unwrap();
			let src = self.img[src_idx];
			let dest = self.img[dest_idx];

			if src != dest {
				self.img[dest_idx] = update(self, src, dest, src_idx, dest_idx);
			}
		}
	}

	#[inline(always)]
	#[must_use]
	pub fn get_cell(&self, idx: Coord) -> C {
		self.img[idx]
	}

	#[must_use]
	pub fn get_neighbours(&self, cell_idx: Coord) -> [C; 8] {
		[
			self.img[Self::get_neighbour_idx(cell_idx, (-1, -1))],
			self.img[Self::get_neighbour_idx(cell_idx, (0, -1))],
			self.img[Self::get_neighbour_idx(cell_idx, (1, -1))],
			self.img[Self::get_neighbour_idx(cell_idx, (-1, 0))],
			/* ignore self */
			self.img[Self::get_neighbour_idx(cell_idx, (1, 0))],
			self.img[Self::get_neighbour_idx(cell_idx, (-1, 1))],
			self.img[Self::get_neighbour_idx(cell_idx, (0, 1))],
			self.img[Self::get_neighbour_idx(cell_idx, (1, 1))],
		]
	}

	#[must_use]
	fn get_neighbours_idx(cell_idx: Coord) -> [Coord; 8] {
		[
			Self::get_neighbour_idx(cell_idx, (-1, -1)),
			Self::get_neighbour_idx(cell_idx, (0, -1)),
			Self::get_neighbour_idx(cell_idx, (1, -1)),
			Self::get_neighbour_idx(cell_idx, (-1, 0)),
			/* ignore self */
			Self::get_neighbour_idx(cell_idx, (1, 0)),
			Self::get_neighbour_idx(cell_idx, (-1, 1)),
			Self::get_neighbour_idx(cell_idx, (0, 1)),
			Self::get_neighbour_idx(cell_idx, (1, 1)),
		]
	}

	#[inline(always)]
	#[must_use]
	fn get_neighbour_idx(cell_idx: Coord, offset: (i32, i32)) -> Coord {
		(
			(cell_idx.0 as i32 + offset.0).rem_euclid(W as i32) as u32,
			(cell_idx.1 as i32 + offset.1).rem_euclid(H as i32) as u32,
		)
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

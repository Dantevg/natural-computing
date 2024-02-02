pub mod game_of_life;
// pub mod grow;
// pub mod sir;

use core::borrow::BorrowMut;

use imgref::Img;
use loop9::loop9_img;

pub trait Cell
where
	Self: Clone + Copy,
{
	fn colour(&self) -> [u8; 4];
}

pub trait Automaton<S: Cell, const W: usize, const H: usize> {
	fn get_world(&self) -> &World<S, W, H>;
	fn get_world_mut(&mut self) -> &mut World<S, W, H>;
	fn rule(&self, neighbourhood: [S; 9]) -> S;

	fn step(&mut self) {
		let world = self
			.get_world()
			.convolve(|neighbourhood| self.rule(neighbourhood));
		*self.get_world_mut() = world;
	}
}

pub struct World<S: Cell, const W: usize, const H: usize> {
	img: Img<Vec<S>>,
	wrap: bool,
}

impl<S: Cell + Default, const W: usize, const H: usize> Default for World<S, W, H> {
	fn default() -> Self {
		Self {
			img: Img::new(vec![S::default(); W * H], W, H),
			wrap: true,
		}
	}
}

impl<S: Cell, const W: usize, const H: usize> World<S, W, H> {
	pub fn from_fn<F>(function: F, wrap: bool) -> Self
	where
		F: FnMut(usize) -> S,
	{
		let buf = (0..(W * H)).map(function).collect();
		Self {
			img: Img::new(buf, W, H),
			wrap,
		}
	}

	pub fn convolve<F>(&self, mut function: F) -> Self
	where
		F: FnMut([S; 9]) -> S,
	{
		// TODO: check order of wrapping and convolution
		let mut new_world = Self {
			img: self.img.clone(),
			wrap: self.wrap,
		};
		if self.wrap {
			new_world.wrap_edges();
		}
		let mut new_grid = new_world.img.clone();
		loop9_img(new_world.img.as_ref(), |x, y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.curr, mid.next, bot.prev, bot.curr,
				bot.next,
			];
			new_grid[(x, y)] = function(neighbourhood);
		});

		Self {
			img: new_grid,
			wrap: self.wrap,
		}
	}

	pub fn draw(&mut self, frame: &mut [u8], frame_width: usize, scale: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width / scale;
			let y = i / frame_width / scale;

			if x < W && y < H {
				pixel.copy_from_slice(&self.img[(x, y)].colour());
			}
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

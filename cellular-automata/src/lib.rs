pub mod game_of_life;

use imgref::Img;
use loop9::loop9_img;

pub trait Automaton<S: Clone + Copy, const W: usize, const H: usize> {
	fn get_world(&mut self) -> &mut World<S, W, H>;
	fn transition(neighbourhood: [S; 9]) -> S;
	fn colour(cell: S) -> [u8; 4];

	fn step(&mut self) {
		let world = self.get_world();
		let mut new_grid = world.0.clone();
		loop9_img(world.0.as_ref(), |x, y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.curr, mid.next, bot.prev, bot.curr,
				bot.next,
			];
			new_grid[(x, y)] = Self::transition(neighbourhood);
		});
		world.0 = new_grid;
	}

	fn draw(&mut self, frame: &mut [u8], frame_width: usize, scale: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width / scale;
			let y = i / frame_width / scale;

			if x < W && y < H {
				pixel.copy_from_slice(&Self::colour(self.get_world().0[(x, y)]));
			}
		}
	}
}

pub struct World<S: Clone + Copy, const W: usize, const H: usize>(Img<Vec<S>>);

impl<S: Default + Clone + Copy, const W: usize, const H: usize> World<S, W, H> {
	pub fn from_fn<F>(function: F) -> Self
	where
		F: FnMut(usize) -> S,
	{
		let buf = (0..(W * H)).map(function).collect();
		Self(Img::new(buf, W, H))
	}
}

impl<S: Default + Clone + Copy, const W: usize, const H: usize> Default for World<S, W, H> {
	fn default() -> Self {
		Self(Img::new(vec![S::default(); W * H], W, H))
	}
}

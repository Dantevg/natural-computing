use imgref::Img;
use loop9::{loop9_img, Triple};

pub struct GameOfLife<const W: usize, const H: usize> {
	pub grid: Img<Vec<u8>>,
}

impl<const W: usize, const H: usize> GameOfLife<W, H> {
	pub fn new() -> Self {
		GameOfLife {
			grid: Img::new(vec![0; W * H], W, H),
		}
	}

	pub fn step(&mut self) {
		let mut new_grid = self.grid.clone();
		loop9_img(self.grid.as_ref(), |x, y, top, mid, bot| {
			let n_neighbours = get_neighbours(top, mid, bot);
			new_grid[(x, y)] = transition(mid.curr == 1, n_neighbours) as u8;
		});
		self.grid = new_grid;
	}

	pub fn draw(&mut self, frame: &mut [u8], frame_width: usize, scale: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width / scale;
			let y = i / frame_width / scale;

			if x < W && y < H {
				pixel.copy_from_slice(&colour(self.grid[(x, y)]));
			}
		}
	}
}

fn get_neighbours(top: Triple<u8>, mid: Triple<u8>, bot: Triple<u8>) -> u8 {
	(top.prev == 1) as u8
	+ (top.curr == 1) as u8
	+ (top.next == 1) as u8
	+ (mid.prev == 1) as u8
	// ignore self
	+ (mid.next == 1) as u8
	+ (bot.prev == 1) as u8
	+ (bot.curr == 1) as u8
	+ (bot.next == 1) as u8
}

fn transition(cell: bool, n_neighbours: u8) -> bool {
	(cell && (n_neighbours == 2 || n_neighbours == 3)) || (!cell && n_neighbours == 3)
}

fn colour(cell: u8) -> [u8; 4] {
	if cell == 1 {
		[0xff, 0xff, 0xff, 0xff]
	} else {
		[0x00, 0x00, 0x00, 0xff]
	}
}

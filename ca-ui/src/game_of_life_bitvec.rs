use bitvec::prelude::*;

pub struct GameOfLife<const W: usize, const H: usize> {
	pub grid: BitBox,
}

impl<const W: usize, const H: usize> GameOfLife<W, H> {
	pub fn new() -> Self {
		GameOfLife {
			grid: bitbox![0; W * H],
		}
	}

	pub fn step(&mut self) {
		self.grid = self
			.grid
			.iter()
			.enumerate()
			.map(|(i, cell)| transition(cell, self.get_neighbours(i as i32)))
			.collect();
	}

	fn get_neighbours(&self, i: i32) -> u8 {
		self.get(i - W as i32 - 1)
			+ self.get(i - W as i32)
			+ self.get(i - W as i32 + 1)
			+ self.get(i - 1)
			// ignore self cell
			+ self.get(i + 1)
			+ self.get(i + W as i32 - 1)
			+ self.get(i + W as i32)
			+ self.get(i + W as i32 + 1)
	}

	fn get(&self, i: i32) -> u8 {
		if let Ok(i) = usize::try_from(i) {
			self.grid.get(i).map_or(0, |x| *x as u8)
		} else {
			0
		}
	}

	pub fn draw(&self, frame: &mut [u8], frame_width: usize, scale: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width / scale;
			let y = i / frame_width / scale;

			if x < W && y < H {
				let index = x + y * W;
				pixel.copy_from_slice(&colour(self.grid[index]));
			}
		}
	}
}

fn transition(cell: BitRef, n_neighbours: u8) -> bool {
	(*cell && (n_neighbours == 2 || n_neighbours == 3)) || (!*cell && n_neighbours == 3)
}

fn colour(cell: bool) -> [u8; 4] {
	if cell {
		[0xff, 0xff, 0xff, 0xff]
	} else {
		[0x00, 0x00, 0x00, 0xff]
	}
}

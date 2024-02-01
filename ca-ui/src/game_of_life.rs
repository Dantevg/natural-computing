use core::fmt::{Display, Write};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum LifeState {
	#[default]
	Dead,
	Alive,
}

impl LifeState {
	pub fn is_alive(&self) -> bool {
		match self {
			LifeState::Dead => false,
			LifeState::Alive => true,
		}
	}

	pub fn colour(&self) -> [u8; 4] {
		match self {
			LifeState::Dead => [0x00, 0x00, 0x00, 0xff],
			LifeState::Alive => [0xff, 0xff, 0xff, 0xff],
		}
	}
}

impl Display for LifeState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LifeState::Dead => f.write_char('.'),
			LifeState::Alive => f.write_char('#'),
		}
	}
}

fn count_alive_neighbours(neighbours: [[Option<&LifeState>; 3]; 3]) -> u8 {
	let count = neighbours
		.iter()
		.flatten()
		.filter(|cell| cell.is_some_and(LifeState::is_alive))
		.count();
	count.try_into().unwrap()
}

pub fn game_of_life(state: &LifeState, neighbours: [[Option<&LifeState>; 3]; 3]) -> LifeState {
	let n_neighbours = count_alive_neighbours(neighbours);
	match state {
		LifeState::Dead => {
			if n_neighbours == 3 {
				LifeState::Alive
			} else {
				LifeState::Dead
			}
		}
		LifeState::Alive => {
			if n_neighbours >= 2 && n_neighbours <= 3 {
				LifeState::Alive
			} else {
				LifeState::Dead
			}
		}
	}
}

pub fn grid_to_string<const W: usize, const H: usize>(grid: &[[LifeState; H]; W]) -> String {
	grid.iter()
		.map(|row| row.iter().map(|cell| cell.to_string()).collect::<String>() + "\n")
		.collect()
}

pub fn draw_grid<const W: usize, const H: usize>(
	grid: &[[LifeState; W]; H],
	frame: &mut [u8],
	frame_width: usize,
	scale: usize,
) {
	for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
		let x = i % frame_width / scale;
		let y = i / frame_width / scale;

		if x < W && y < H {
			pixel.copy_from_slice(&grid[y][x].colour());
		}
	}
}

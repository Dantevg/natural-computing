use crate::{Automaton, World};

#[derive(Default)]
pub struct GameOfLife<const W: usize, const H: usize> {
	world: World<bool, W, H>,
}

impl<const W: usize, const H: usize> GameOfLife<W, H> {
	pub fn new(world: World<bool, W, H>) -> Self {
		Self { world }
	}
}

impl<const W: usize, const H: usize> Automaton<bool, W, H> for GameOfLife<W, H> {
	fn get_world(&mut self) -> &mut World<bool, W, H> {
		&mut self.world
	}

	fn transition(neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - cell as u8;
		(cell && (n_neighbours == 2 || n_neighbours == 3)) || (!cell && n_neighbours == 3)
	}

	fn colour(cell: bool) -> [u8; 4] {
		if cell {
			[0xff, 0xff, 0xff, 0xff]
		} else {
			[0x00, 0x00, 0x00, 0xff]
		}
	}
}

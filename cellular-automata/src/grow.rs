use crate::{Automaton, GridWorld};

#[derive(Default)]
pub struct Grow<const W: usize, const H: usize> {
	world: GridWorld<bool, W, H>,
}

impl<const W: usize, const H: usize> Grow<W, H> {
	pub fn new(world: GridWorld<bool, W, H>) -> Self {
		Self { world }
	}
}

impl<const W: usize, const H: usize> Automaton<bool> for Grow<W, H> {
	fn get_world(&self) -> &GridWorld<bool, W, H> {
		&self.world
	}
	fn get_world_mut(&mut self) -> &mut GridWorld<bool, W, H> {
		&mut self.world
	}

	fn transition(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - cell as u8;
		cell || rand::random::<f32>() < n_neighbours as f32 * 0.1
	}

	fn colour(cell: bool) -> [u8; 4] {
		if cell {
			[0xff, 0xff, 0xff, 0xff]
		} else {
			[0x00, 0x00, 0x00, 0xff]
		}
	}
}

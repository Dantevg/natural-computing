use crate::{Automaton, World};

#[derive(Default)]
pub struct Grow<const W: usize, const H: usize> {
	world: World<bool, W, H>,
}

impl<const W: usize, const H: usize> Grow<W, H> {
	pub fn new(world: World<bool, W, H>) -> Self {
		Self { world }
	}
}

impl<const W: usize, const H: usize> Automaton<bool, W, H> for Grow<W, H> {
	fn get_world(&self) -> &World<bool, W, H> {
		&self.world
	}
	fn get_world_mut(&mut self) -> &mut World<bool, W, H> {
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

use crate::{Automaton, GridWorld};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SirState {
	#[default]
	Susceptible,
	Infected,
	Resistant,
}

#[derive(Default)]
pub struct Sir<const W: usize, const H: usize> {
	world: GridWorld<SirState, W, H>,
	p_cure: f32,
}

impl<const W: usize, const H: usize> Sir<W, H> {
	pub fn new(world: GridWorld<SirState, W, H>, p_cure: f32) -> Self {
		Self { world, p_cure }
	}
}

impl<const W: usize, const H: usize> Automaton<SirState> for Sir<W, H> {
	fn get_world(&self) -> &GridWorld<SirState, W, H> {
		&self.world
	}
	fn get_world_mut(&mut self) -> &mut GridWorld<SirState, W, H> {
		&mut self.world
	}

	fn transition(&self, neighbourhood: [SirState; 9]) -> SirState {
		let cell = neighbourhood[4];
		match cell {
			SirState::Susceptible => {
				let n_inf_neighbours = neighbourhood
					.into_iter()
					.filter(|&cell| cell == SirState::Infected)
					.count() as u8 - cell as u8;
				if rand::random::<f32>() < n_inf_neighbours as f32 * 0.1 {
					SirState::Infected
				} else {
					SirState::Susceptible
				}
			}
			SirState::Infected => {
				if rand::random::<f32>() < self.p_cure {
					SirState::Resistant
				} else {
					SirState::Infected
				}
			}
			SirState::Resistant => SirState::Resistant,
		}
	}

	fn colour(cell: SirState) -> [u8; 4] {
		match cell {
			SirState::Susceptible => [0xff, 0xff, 0xff, 0xff],
			SirState::Infected => [0x00, 0x00, 0x00, 0xff],
			SirState::Resistant => [0x88, 0x88, 0x88, 0xff],
		}
	}
}

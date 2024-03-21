use crate::{ca::Automaton, Cell};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SirState {
	#[default]
	Susceptible,
	Infected,
	Resistant,
}

impl Cell for SirState {
	fn colour(&self) -> [u8; 4] {
		match self {
			SirState::Susceptible => [0xff, 0xff, 0xff, 0xff],
			SirState::Infected => [0x00, 0x00, 0x00, 0xff],
			SirState::Resistant => [0x88, 0x88, 0x88, 0xff],
		}
	}
}

#[derive(Default)]
pub struct Sir {
	p_cure: f32,
}

impl Sir {
	pub fn new(p_cure: f32) -> Self {
		Self { p_cure }
	}
}

impl<const W: usize, const H: usize> Automaton<W, H> for Sir {
	type C = SirState;

	fn rule(&self, neighbourhood: [SirState; 9]) -> SirState {
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
}

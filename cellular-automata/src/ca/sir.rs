use crate::{ca::Automaton, count_neighbours, Cell};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum State {
	#[default]
	Susceptible,
	Infected,
	Resistant,
}

impl Cell for State {
	fn colour(&self) -> [u8; 4] {
		match self {
			State::Susceptible => [0xff, 0xff, 0xff, 0xff],
			State::Infected => [0x00, 0x00, 0x00, 0xff],
			State::Resistant => [0x88, 0x88, 0x88, 0xff],
		}
	}
}

#[derive(Default)]
pub struct Sir {
	p_cure: f32,
}

impl Sir {
	#[must_use]
	pub fn new(p_cure: f32) -> Self {
		Self { p_cure }
	}
}

impl<const W: usize, const H: usize> Automaton<W, H> for Sir {
	type C = State;

	fn rule(&self, neighbourhood: [State; 9]) -> State {
		let cell = neighbourhood[4];
		match cell {
			State::Susceptible => {
				let n_inf_neighbours =
					count_neighbours(neighbourhood, |cell| cell == State::Infected);
				if rand::random::<f32>() < f32::from(n_inf_neighbours) * 0.1 {
					State::Infected
				} else {
					State::Susceptible
				}
			}
			State::Infected => {
				if rand::random::<f32>() < self.p_cure {
					State::Resistant
				} else {
					State::Infected
				}
			}
			State::Resistant => State::Resistant,
		}
	}
}

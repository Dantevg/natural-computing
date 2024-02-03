use crate::{Automaton, Cell};

impl Cell for bool {
	fn colour(&self) -> [u8; 4] {
		if *self {
			[0xff, 0xff, 0xff, 0xff]
		} else {
			[0x00, 0x00, 0x00, 0xff]
		}
	}
}

#[derive(Default)]
pub struct GameOfLife;

impl<const W: usize, const H: usize> Automaton<W, H> for GameOfLife {
	type C = bool;

	fn rule(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - cell as u8;
		(cell && (n_neighbours == 2 || n_neighbours == 3)) || (!cell && n_neighbours == 3)
	}
}

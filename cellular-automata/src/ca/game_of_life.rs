use crate::{ca::Automaton, count_neighbours};

#[derive(Default)]
pub struct GameOfLife;

impl<const W: usize, const H: usize> Automaton<W, H> for GameOfLife {
	type C = bool;

	#[allow(clippy::nonminimal_bool)]
	fn rule(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours = count_neighbours(neighbourhood, |cell| cell);
		(cell && (n_neighbours == 2 || n_neighbours == 3)) || (!cell && n_neighbours == 3)
	}
}

use crate::ca::Automaton;

#[derive(Default)]
pub struct GameOfLife;

impl<const W: usize, const H: usize> Automaton<W, H> for GameOfLife {
	type C = bool;

	#[allow(clippy::nonminimal_bool)]
	fn rule(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - u8::from(cell);
		(cell && (n_neighbours == 2 || n_neighbours == 3)) || (!cell && n_neighbours == 3)
	}
}

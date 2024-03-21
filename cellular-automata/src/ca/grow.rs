use crate::ca::Automaton;

#[derive(Default)]
pub struct Grow;

impl<const W: usize, const H: usize> Automaton<W, H> for Grow {
	type C = bool;

	fn rule(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - cell as u8;
		cell || rand::random::<f32>() < n_neighbours as f32 * 0.1
	}
}

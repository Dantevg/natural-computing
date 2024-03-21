use crate::ca::Automaton;

#[derive(Default)]
pub struct Grow;

impl<const W: usize, const H: usize> Automaton<W, H> for Grow {
	type C = bool;

	fn rule(&self, neighbourhood: [bool; 9]) -> bool {
		let cell = neighbourhood[4];
		let n_neighbours =
			neighbourhood.into_iter().filter(|&cell| cell).count() as u8 - u8::from(cell);
		cell || rand::random::<f32>() < f32::from(n_neighbours) * 0.1
	}
}

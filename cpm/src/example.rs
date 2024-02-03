use cellular_automata::Cell;

use crate::CPM;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CPMCell(u8);

impl Cell for CPMCell {
	fn colour(&self) -> [u8; 4] {
		[
			(self.0 % 8) * 32,
			(self.0 % 8) * 32,
			(self.0 % 8) * 32,
			0xff,
		]
	}
}

pub struct ExampleCPM {
	temperature: f32,
}

impl<const W: usize, const H: usize> CPM<W, H> for ExampleCPM {
	type C = CPMCell;

	fn rule(&self, src: CPMCell, dest: CPMCell) -> CPMCell {
		let p_copy = 1.0; // TODO: calculate p_copy
		if rand::random::<f32>() < p_copy {
			todo!()
		} else {
			dest
		}
	}
}

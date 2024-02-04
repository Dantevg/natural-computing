use cellular_automata::{world::World, Cell};

use crate::{Adhesion, CPM};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct CPMCell(pub u8);

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
	adhesion_penalty: f32,
}

impl ExampleCPM {
	pub fn new(temperature: f32, adhesion_penalty: f32) -> Self {
		Self {
			temperature,
			adhesion_penalty,
		}
	}
}

impl<const W: usize, const H: usize> CPM<W, H> for ExampleCPM {
	type C = CPMCell;

	#[inline(always)]
	fn get_temperature(&self) -> f32 {
		self.temperature
	}

	fn hamiltonian(
		&self,
		world: &World<W, H, CPMCell>,
		src: CPMCell,
		dest: CPMCell,
		src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		self.adhesion_delta(world, src, dest, src_idx, dest_idx)
	}
}

impl<const W: usize, const H: usize> Adhesion<W, H> for ExampleCPM {
	#[inline(always)]
	fn get_adhesion_penalty(&self) -> f32 {
		self.adhesion_penalty
	}
}

use cellular_automata::{world::World, Cell};

use crate::{Adhesion, Volume, CPM};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct CPMCell(pub u8);

impl CPMCell {
	#[inline(always)]
	pub fn is_bg(&self) -> bool {
		self.0 == 0
	}
}

impl Cell for CPMCell {
	#[inline(always)]
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
	target_volume: u32,
	lambda_volume: f32,
	cell_volumes: Box<[u32]>,
}

impl ExampleCPM {
	pub fn new<const W: usize, const H: usize>(
		temperature: f32,
		adhesion_penalty: f32,
		target_volume: u32,
		lambda_volume: f32,
		world: &World<W, H, CPMCell>,
	) -> Self {
		Self {
			temperature,
			adhesion_penalty,
			target_volume,
			lambda_volume,
			cell_volumes: ExampleCPM::get_cell_volumes(world),
		}
	}

	fn get_cell_volumes<const W: usize, const H: usize>(
		world: &World<W, H, CPMCell>,
	) -> Box<[u32]> {
		let mut volumes = (0..=u8::MAX)
			.into_iter()
			.map(|_| 0 as u32)
			.collect::<Vec<u32>>()
			.into_boxed_slice();

		for cell in world.img.pixels().filter(|cell| !cell.is_bg()) {
			volumes[cell.0 as usize] += 1;
		}

		volumes
	}
}

impl<const W: usize, const H: usize> CPM<W, H> for ExampleCPM {
	type C = CPMCell;

	#[inline(always)]
	fn get_temperature(&self) -> f32 {
		self.temperature
	}

	fn update(&mut self, src: Self::C, dest: Self::C) {
		if !src.is_bg() {
			self.cell_volumes[src.0 as usize] += 1
		}
		if !dest.is_bg() {
			self.cell_volumes[dest.0 as usize] -= 1
		}
	}

	fn hamiltonian(
		&self,
		world: &World<W, H, CPMCell>,
		src: CPMCell,
		dest: CPMCell,
		src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		let adhesion = self.adhesion_delta(world, src, dest, src_idx, dest_idx);
		let volume = self.volume_delta(world, src, dest, src_idx, dest_idx);
		adhesion + volume
	}
}

impl<const W: usize, const H: usize> Adhesion<W, H> for ExampleCPM {
	#[inline(always)]
	fn get_adhesion_penalty(&self, _a: CPMCell, _b: CPMCell) -> f32 {
		self.adhesion_penalty
	}
}

impl<const W: usize, const H: usize> Volume<W, H> for ExampleCPM {
	fn get_volume_penalty(&self, cell: CPMCell, volume: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else {
			self.lambda_volume * (volume - self.target_volume).pow(2) as f32
		}
	}

	fn volume(&self, _world: &World<W, H, Self::C>, _idx: usize, state: Self::C) -> u32 {
		self.cell_volumes[state.0 as usize]
	}
}

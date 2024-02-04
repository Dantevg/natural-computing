use cellular_automata::{world::World, Cell};

use crate::{
	adhesion::Adhesion, cell_perimeters::CellPerimeters, cell_volumes::CellVolumes,
	perimeter::Perimeter, volume::Volume, CPMCell, CPM,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ExampleCell(pub u8);

impl CPMCell for ExampleCell {
	#[inline(always)]
	fn is_bg(&self) -> bool {
		self.0 == 0
	}

	#[inline(always)]
	fn id(&self) -> usize {
		self.0 as usize
	}
}

impl Cell for ExampleCell {
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
	target_perimeter: u32,
	lambda_perimeter: f32,
	cell_volumes: CellVolumes,
	cell_perimeters: CellPerimeters,
}

impl ExampleCPM {
	pub fn new<const W: usize, const H: usize>(
		temperature: f32,
		adhesion_penalty: f32,
		target_volume: u32,
		lambda_volume: f32,
		target_perimeter: u32,
		lambda_perimeter: f32,
		world: &World<W, H, ExampleCell>,
	) -> Self {
		Self {
			temperature,
			adhesion_penalty,
			target_volume,
			lambda_volume,
			target_perimeter,
			lambda_perimeter,
			cell_volumes: CellVolumes::from_world(world),
			cell_perimeters: CellPerimeters::from_world(world),
		}
	}
}

impl<const W: usize, const H: usize> CPM<W, H> for ExampleCPM {
	type C = ExampleCell;

	#[inline(always)]
	fn get_temperature(&self) -> f32 {
		self.temperature
	}

	fn update(
		&mut self,
		world: &World<W, H, ExampleCell>,
		src: ExampleCell,
		dest: ExampleCell,
		src_idx: usize,
		dest_idx: usize,
	) {
		self.cell_volumes
			.update(world, src, dest, src_idx, dest_idx);
		self.cell_perimeters
			.update(world, src, dest, src_idx, dest_idx);
	}

	fn hamiltonian(
		&self,
		world: &World<W, H, ExampleCell>,
		src: ExampleCell,
		dest: ExampleCell,
		src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		let adhesion = self.adhesion_delta(world, src, dest, src_idx, dest_idx);
		let volume = self.volume_delta(world, src, dest, src_idx, dest_idx);
		let perimeter = self.perimeter_delta(world, src, dest, src_idx, dest_idx);
		adhesion + volume + perimeter
	}
}

impl<const W: usize, const H: usize> Adhesion<W, H> for ExampleCPM {
	#[inline(always)]
	fn get_adhesion_penalty(&self, _a: ExampleCell, _b: ExampleCell) -> f32 {
		self.adhesion_penalty
	}
}

impl<const W: usize, const H: usize> Volume<W, H> for ExampleCPM {
	fn get_volume_penalty(&self, cell: ExampleCell, volume: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else {
			self.lambda_volume * (volume - self.target_volume).pow(2) as f32
		}
	}

	fn volume(&self, _world: &World<W, H, ExampleCell>, _idx: usize, state: ExampleCell) -> u32 {
		self.cell_volumes.get(state)
	}
}

impl<const W: usize, const H: usize> Perimeter<W, H> for ExampleCPM {
	fn get_perimeter_penalty(&self, cell: ExampleCell, perimeter: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else {
			self.lambda_perimeter * (perimeter - self.target_perimeter).pow(2) as f32
		}
	}

	fn perimeter(&self, _world: &World<W, H, ExampleCell>, _idx: usize, state: ExampleCell) -> u32 {
		self.cell_perimeters.get(state)
	}
}

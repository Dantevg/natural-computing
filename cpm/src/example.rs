use cellular_automata::{world::World, Cell};
use loop9::loop9_img;

use crate::{adhesion::Adhesion, perimeter::Perimeter, volume::Volume, CPM};

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
	target_perimeter: u32,
	lambda_perimeter: f32,
	cell_volumes: Box<[u32]>,
	cell_perimeters: Box<[u32]>,
}

impl ExampleCPM {
	pub fn new<const W: usize, const H: usize>(
		temperature: f32,
		adhesion_penalty: f32,
		target_volume: u32,
		lambda_volume: f32,
		target_perimeter: u32,
		lambda_perimeter: f32,
		world: &World<W, H, CPMCell>,
	) -> Self {
		Self {
			temperature,
			adhesion_penalty,
			target_volume,
			lambda_volume,
			target_perimeter,
			lambda_perimeter,
			cell_volumes: ExampleCPM::get_cell_volumes(world),
			cell_perimeters: ExampleCPM::get_cell_perimeters(world),
		}
	}

	fn get_cell_volumes<const W: usize, const H: usize>(
		world: &World<W, H, CPMCell>,
	) -> Box<[u32]> {
		let mut volumes = vec![0; u8::MAX as usize + 1].into_boxed_slice();

		for cell in world.img.pixels().filter(|cell| !cell.is_bg()) {
			volumes[cell.0 as usize] += 1;
		}

		volumes
	}

	fn update_volumes<const W: usize, const H: usize>(
		&mut self,
		_world: &World<W, H, CPMCell>,
		src: CPMCell,
		dest: CPMCell,
		_src_idx: usize,
		_dest_idx: usize,
	) {
		if !src.is_bg() {
			self.cell_volumes[src.0 as usize] += 1
		}
		if !dest.is_bg() {
			self.cell_volumes[dest.0 as usize] -= 1
		}
	}

	fn get_cell_perimeters<const W: usize, const H: usize>(
		world: &World<W, H, CPMCell>,
	) -> Box<[u32]> {
		let mut perimeters = vec![0; u8::MAX as usize + 1].into_boxed_slice();

		// TODO: wrap borders

		loop9_img(world.img.as_ref(), |_x, _y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.next, bot.prev, bot.curr, bot.next,
			];
			perimeters[mid.curr.0 as usize] +=
				neighbourhood.into_iter().filter(|&n| n != mid.curr).count() as u32;
		});

		perimeters
	}

	fn update_perimeters<const W: usize, const H: usize>(
		&mut self,
		world: &World<W, H, CPMCell>,
		src: CPMCell,
		dest: CPMCell,
		_src_idx: usize,
		dest_idx: usize,
	) {
		let neighbourhood = world.get_neighbours_idx(dest_idx);
		let mut n_new = 0;
		let mut n_old = 0;
		for neighbour in neighbourhood.into_iter().map(|&i| world.get_cell(i)) {
			n_new += (neighbour != src) as u32;
			n_old += (neighbour != dest) as u32;
			if !neighbour.is_bg() {
				self.cell_perimeters[neighbour.0 as usize] += (neighbour == dest) as u32;
				self.cell_perimeters[neighbour.0 as usize] -= (neighbour == src) as u32;
			}
		}
		if !dest.is_bg() {
			self.cell_perimeters[dest.0 as usize] -= n_old;
		}
		if !src.is_bg() {
			self.cell_perimeters[src.0 as usize] += n_new;
		}
	}
}

impl<const W: usize, const H: usize> CPM<W, H> for ExampleCPM {
	type C = CPMCell;

	#[inline(always)]
	fn get_temperature(&self) -> f32 {
		self.temperature
	}

	fn update(
		&mut self,
		world: &World<W, H, CPMCell>,
		src: CPMCell,
		dest: CPMCell,
		src_idx: usize,
		dest_idx: usize,
	) {
		self.update_volumes(world, src, dest, src_idx, dest_idx);
		self.update_perimeters(world, src, dest, src_idx, dest_idx);
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
		let perimeter = self.perimeter_delta(world, src, dest, src_idx, dest_idx);
		adhesion + volume + perimeter
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

impl<const W: usize, const H: usize> Perimeter<W, H> for ExampleCPM {
	fn get_perimeter_penalty(&self, cell: Self::C, perimeter: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else {
			self.lambda_perimeter * (perimeter - self.target_perimeter).pow(2) as f32
		}
	}

	fn perimeter(&self, _world: &World<W, H, Self::C>, _idx: usize, state: Self::C) -> u32 {
		self.cell_perimeters[state.0 as usize]
	}
}

use cellular_automata::world::{Coord, World};

use crate::CPMCell;

pub struct CellVolumes(Box<[u32]>);

impl CellVolumes {
	pub fn from_world<const W: usize, const H: usize, C: CPMCell>(world: &World<W, H, C>) -> Self {
		let mut volumes = vec![0; C::MAX_ID + 1].into_boxed_slice();

		for cell in world.img.pixels().filter(|c| !c.is_bg()) {
			volumes[cell.id()] += 1;
		}

		Self(volumes)
	}

	pub fn update<const W: usize, const H: usize, C: CPMCell>(
		&mut self,
		_world: &World<W, H, C>,
		src: C,
		dest: C,
		_src_idx: Coord,
		_dest_idx: Coord,
	) {
		if !src.is_bg() {
			self.0[src.id()] += 1
		}
		if !dest.is_bg() {
			self.0[dest.id()] -= 1
		}
	}

	pub fn recalculate<const W: usize, const H: usize, C: CPMCell>(
		&mut self,
		world: &World<W, H, C>,
	) {
		self.0 = vec![0; C::MAX_ID + 1].into_boxed_slice();
		for cell in world.img.pixels().filter(|c| !c.is_bg()) {
			self.0[cell.id()] += 1;
		}
	}

	#[inline(always)]
	pub fn get<C: CPMCell>(&self, cell: C) -> u32 {
		self.0[cell.id()]
	}
}

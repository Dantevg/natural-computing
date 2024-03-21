use crate::{
	cpm::CPMCell,
	world::{Coord, World},
};

use loop9::loop9_img;

pub struct CellPerimeters(Box<[u32]>);

impl CellPerimeters {
	pub fn from_world<const W: usize, const H: usize, C: CPMCell>(world: &World<W, H, C>) -> Self {
		let mut perimeters = vec![0; C::MAX_ID + 1].into_boxed_slice();

		// TODO: wrap borders

		loop9_img(world.img.as_ref(), |_x, _y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.next, bot.prev, bot.curr, bot.next,
			];
			perimeters[mid.curr.id()] +=
				neighbourhood.into_iter().filter(|&n| n != mid.curr).count() as u32;
		});

		Self(perimeters)
	}

	// Ported from https://github.com/ingewortel/artistoo/blob/master/src/hamiltonian/PerimeterConstraint.js
	pub fn update<const W: usize, const H: usize, C: CPMCell>(
		&mut self,
		world: &World<W, H, C>,
		src: C,
		dest: C,
		_src_idx: Coord,
		dest_idx: Coord,
	) {
		let mut n_new = 0;
		let mut n_old = 0;
		for neighbour in world.get_neighbours(dest_idx) {
			n_new += (neighbour != src) as u32;
			n_old += (neighbour != dest) as u32;
			if !neighbour.is_bg() {
				self.0[neighbour.id()] += (neighbour == dest) as u32;
				self.0[neighbour.id()] -= (neighbour == src) as u32;
			}
		}
		if !dest.is_bg() {
			self.0[dest.id()] -= n_old;
		}
		if !src.is_bg() {
			self.0[src.id()] += n_new;
		}
	}

	pub fn recalculate<const W: usize, const H: usize, C: CPMCell>(
		&mut self,
		world: &World<W, H, C>,
	) {
		self.0 = vec![0; C::MAX_ID + 1].into_boxed_slice();
		loop9_img(world.img.as_ref(), |_x, _y, top, mid, bot| {
			let neighbourhood = [
				top.prev, top.curr, top.next, mid.prev, mid.next, bot.prev, bot.curr, bot.next,
			];
			self.0[mid.curr.id()] +=
				neighbourhood.into_iter().filter(|&n| n != mid.curr).count() as u32;
		});
	}

	#[inline(always)]
	pub fn get<C: CPMCell>(&self, cell: C) -> u32 {
		self.0[cell.id()]
	}
}

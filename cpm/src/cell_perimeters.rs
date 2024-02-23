use cellular_automata::world::World;
use loop9::loop9_img;

use crate::CPMCell;

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

	pub fn update<const W: usize, const H: usize, C: CPMCell>(
		&mut self,
		world: &World<W, H, C>,
		src: C,
		dest: C,
		_src_idx: usize,
		dest_idx: usize,
	) {
		let neighbourhood = world.get_neighbours_idx(dest_idx);
		let mut n_new = 0;
		let mut n_old = 0;
		for neighbour in neighbourhood.iter().map(|&i| world.get_cell(i)) {
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

	pub fn get<C: CPMCell>(&self, cell: C) -> u32 {
		self.0[cell.id()]
	}
}

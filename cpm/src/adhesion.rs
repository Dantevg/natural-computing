use cellular_automata::world::World;

use crate::CPM;

pub trait Adhesion<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_adhesion_penalty(&self, a: Self::C, b: Self::C) -> f32;

	/// Returns the adhesion energy for a single cell.
	fn adhesion(&self, world: &World<W, H, Self::C>, idx: usize, cell: Self::C) -> f32 {
		world
			.get_neighbours_idx(idx)
			.iter()
			.filter(|&neigh_idx| world.get_cell(*neigh_idx) != cell)
			.map(|&neigh_idx| self.get_adhesion_penalty(cell, world.get_cell(neigh_idx)))
			.sum()
	}

	/// Returns the delta adhesion energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn adhesion_delta(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		_src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		self.adhesion(world, dest_idx, src) - self.adhesion(world, dest_idx, dest)
	}
}

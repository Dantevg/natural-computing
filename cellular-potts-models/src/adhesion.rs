use cellular_automata::world::{Coord, World};

use crate::CPM;

pub trait Adhesion<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_adhesion_penalty(&self, a: Self::C, b: Self::C) -> f32;

	/// Returns the adhesion energy for a single cell.
	fn adhesion(&self, world: &World<W, H, Self::C>, idx: Coord, cell: Self::C) -> f32 {
		world
			.get_neighbours(idx)
			.iter()
			.filter(|&neigh| *neigh != cell)
			.map(|&neigh| self.get_adhesion_penalty(cell, neigh))
			.sum()
	}

	/// Returns the delta adhesion energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn adhesion_delta(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		_src_idx: Coord,
		dest_idx: Coord,
	) -> f32 {
		self.adhesion(world, dest_idx, src) - self.adhesion(world, dest_idx, dest)
	}
}

pub mod example;

use cellular_automata::{world::World, Cell};

pub trait CPM<const W: usize, const H: usize> {
	type C: Cell;
	fn hamiltonian(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		src_idx: usize,
		dest_idx: usize,
	) -> f32;

	fn get_temperature(&self) -> f32;

	fn step(&self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.metropolis(|w, src, dest, src_idx, dest_idx| {
			let hamiltonian = self.hamiltonian(w, src, dest, src_idx, dest_idx);
			if hamiltonian <= 0.0 {
				1.0
			} else {
				f32::exp(-hamiltonian / self.get_temperature())
			}
		})
	}
}

pub trait Adhesion<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_adhesion_penalty(&self) -> f32;

	/// Returns the adhesion energy for a single cell.
	fn adhesion(&self, world: &World<W, H, Self::C>, idx: usize, cell: Self::C) -> f32 {
		world
			.get_neighbours_idx(idx)
			.into_iter()
			.filter(|&neigh_idx| world.get_cell(*neigh_idx) != cell)
			.count() as f32
			* self.get_adhesion_penalty()
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

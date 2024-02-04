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

	fn update(&mut self, _src: Self::C, _dest: Self::C) {}

	fn step(&mut self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.metropolis(|w, src, dest, src_idx, dest_idx| {
			let hamiltonian = self.hamiltonian(w, src, dest, src_idx, dest_idx);
			if hamiltonian <= 0.0
				|| rand::random::<f32>() < f32::exp(-hamiltonian / self.get_temperature())
			{
				self.update(src, dest);
				src
			} else {
				dest
			}
		})
	}
}

pub trait Adhesion<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_adhesion_penalty(&self, a: Self::C, b: Self::C) -> f32;

	/// Returns the adhesion energy for a single cell.
	fn adhesion(&self, world: &World<W, H, Self::C>, idx: usize, cell: Self::C) -> f32 {
		world
			.get_neighbours_idx(idx)
			.into_iter()
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

pub trait Volume<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_volume_penalty(&self, cell: Self::C, volume: u32) -> f32;

	/// Returns the volume in the number of grid cells for a single cell, if
	/// that grid cell were to have the given `state`.
	fn volume(&self, world: &World<W, H, Self::C>, idx: usize, state: Self::C) -> u32;

	/// Returns the delta volume energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn volume_delta(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		let src_vol = self.volume(world, src_idx, src);
		let dest_vol = self.volume(world, dest_idx, dest);
		let src_gain =
			self.get_volume_penalty(src, src_vol + 1) - self.get_volume_penalty(src, src_vol);
		let dest_loss =
			self.get_volume_penalty(dest, dest_vol - 1) - self.get_volume_penalty(dest, dest_vol);
		src_gain + dest_loss
	}
}

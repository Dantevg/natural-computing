pub mod act;
pub mod adhesion;
pub mod cell_perimeters;
pub mod cell_volumes;
pub mod perimeter;
pub mod volume;

use crate::{
	world::{Coord, World},
	Cell,
};

pub trait CPMCell
where
	Self: Cell,
{
	const MAX_ID: usize;

	fn is_bg(&self) -> bool;
	fn id(&self) -> usize;
}

pub trait CPM<const W: usize, const H: usize> {
	type C: CPMCell;
	fn hamiltonian(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		src_idx: Coord,
		dest_idx: Coord,
	) -> f32;

	fn get_temperature(&self) -> f32;

	fn update(
		&mut self,
		_world: &World<W, H, Self::C>,
		src: Self::C,
		_dest: Self::C,
		_src_idx: Coord,
		_dest_idx: Coord,
	) -> Self::C {
		src
	}

	fn after_step(&mut self, _world: &mut World<W, H, Self::C>) {}

	fn step(&mut self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.metropolis(|w, src, dest, src_idx, dest_idx| {
			let hamiltonian = self.hamiltonian(w, src, dest, src_idx, dest_idx);
			if hamiltonian <= 0.0
				|| rand::random::<f32>() < f32::exp(-hamiltonian / self.get_temperature())
			{
				self.update(w, src, dest, src_idx, dest_idx)
			} else {
				dest
			}
		});
		self.after_step(world);
	}
}

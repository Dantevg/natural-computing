pub mod adhesion;
pub mod example;
pub mod perimeter;
pub mod volume;

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

	fn update(
		&mut self,
		_world: &World<W, H, Self::C>,
		_src: Self::C,
		_dest: Self::C,
		_src_idx: usize,
		_dest_idx: usize,
	) {
	}

	fn step(&mut self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.metropolis(|w, src, dest, src_idx, dest_idx| {
			let hamiltonian = self.hamiltonian(w, src, dest, src_idx, dest_idx);
			if hamiltonian <= 0.0
				|| rand::random::<f32>() < f32::exp(-hamiltonian / self.get_temperature())
			{
				self.update(w, src, dest, src_idx, dest_idx);
				src
			} else {
				dest
			}
		})
	}
}

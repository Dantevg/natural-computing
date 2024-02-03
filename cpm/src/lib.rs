pub mod example;

use cellular_automata::{world::World, Cell};

pub trait CPM<const W: usize, const H: usize> {
	type C: Cell;
	fn rule(&self, src: Self::C, dest: Self::C) -> Self::C;

	fn step(&self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.metropolis(|src, dest| self.rule(src, dest))
	}
}

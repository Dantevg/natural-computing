pub mod game_of_life;
pub mod grow;
pub mod sir;

use crate::{world::World, Cell};

pub trait Automaton<const W: usize, const H: usize> {
	type C: Cell;

	#[must_use]
	fn rule(&self, neighbourhood: [Self::C; 9]) -> Self::C;

	fn step(&self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.convolve(|n| self.rule(n));
	}
}

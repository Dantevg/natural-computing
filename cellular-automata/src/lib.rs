pub mod game_of_life;
pub mod grow;
pub mod sir;
pub mod world;

use world::World;

pub trait Cell
where
	Self: Clone + Copy + PartialEq + Eq,
{
	fn colour(&self) -> [u8; 4];
}

pub trait Automaton<const W: usize, const H: usize> {
	type C: Cell;
	fn rule(&self, neighbourhood: [Self::C; 9]) -> Self::C;

	fn step(&self, world: &mut World<W, H, Self::C>)
	where
		Self: Sized,
	{
		world.convolve(|n| self.rule(n))
	}
}

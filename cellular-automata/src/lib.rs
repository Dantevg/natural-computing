pub mod game_of_life;
pub mod grow;
pub mod sir;
pub mod world;

pub trait Cell
where
	Self: Clone + Copy,
{
	fn colour(&self) -> [u8; 4];
}

pub trait Automaton<const W: usize, const H: usize> {
	type S: Cell;
	fn rule(&self, neighbourhood: [Self::S; 9]) -> Self::S;
}

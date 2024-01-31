pub mod game_of_life;

use cellular_automata::Automaton;

use crate::game_of_life::{game_of_life, grid_to_string, LifeState};

fn main() {
	let mut a = Automaton::<LifeState, 8, 8> {
		grid: vec![[LifeState::Dead; 8]; 8]
			.into_boxed_slice()
			.try_into()
			.unwrap(),
		transition: game_of_life,
	};
	a.grid[3][3] = LifeState::Alive;
	a.grid[3][4] = LifeState::Alive;
	a.grid[4][3] = LifeState::Alive;
	a.grid[4][4] = LifeState::Alive;
	println!("{}", grid_to_string(&a.grid));
	a.step();
	println!("{}", grid_to_string(&a.grid));
}

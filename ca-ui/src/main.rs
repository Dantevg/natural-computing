pub mod game_of_life;

use cellular_automata::Automaton;
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::WindowBuilder,
};

use crate::game_of_life::{draw_grid, game_of_life, grid_to_string, LifeState};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

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

	let event_loop = EventLoop::new().unwrap();
	let window = WindowBuilder::new()
		.with_title("Game of Life")
		.with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
		.build(&event_loop)
		.unwrap();

	let mut pixels =
		Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &window)).unwrap();

	event_loop
		.run(move |event, window_target| match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => window_target.exit(),
			Event::WindowEvent {
				event: WindowEvent::RedrawRequested,
				..
			} => {
				draw_grid(&a.grid, pixels.frame_mut());
				pixels.render().unwrap();
			}
			_ => (),
		})
		.unwrap();
}

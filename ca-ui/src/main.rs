pub mod game_of_life;

use core::array;

use cellular_automata::Automaton;
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
	window::WindowBuilder,
};

use crate::game_of_life::{draw_grid, game_of_life, grid_to_string, LifeState};

const WIDTH: usize = 200;
const HEIGHT: usize = 150;
const SCALE: usize = 2;

fn main() {
	let mut automaton = Automaton {
		grid: Box::<[[LifeState; WIDTH]; HEIGHT]>::new(array::from_fn(|_| {
			array::from_fn(|_| rand::random())
		})),
		transition: game_of_life,
	};

	let mut running = false;
	let mut speed = 1;

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let window = WindowBuilder::new()
		.with_title("Game of Life")
		.with_inner_size(PhysicalSize::new(
			(WIDTH * SCALE) as u32,
			(HEIGHT * SCALE) as u32,
		))
		.build(&event_loop)
		.unwrap();

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(
			(WIDTH * SCALE) as u32,
			(HEIGHT * SCALE) as u32,
			surface_texture,
		)
		.unwrap()
	};

	event_loop
		.run(move |event, window_target| {
			if let Event::WindowEvent { event, .. } = event {
				match event {
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => {
						if running {
							for _ in 0..speed {
								automaton.step();
							}
							window.request_redraw();
						}
						draw_grid(&automaton.grid, pixels.frame_mut(), WIDTH * SCALE, SCALE);
						pixels.render().unwrap();
					}
					WindowEvent::KeyboardInput { event, .. } => match event
						.key_without_modifiers()
						.as_ref()
					{
						Key::Named(NamedKey::Space) if event.state == ElementState::Pressed => {
							running = !running;
							if running {
								window.request_redraw();
							}
						}
						Key::Named(NamedKey::ArrowLeft) if event.state == ElementState::Pressed => {
							speed = 1.max(speed - 1);
						}
						Key::Named(NamedKey::ArrowRight)
							if event.state == ElementState::Pressed =>
						{
							speed += 1;
						}
						Key::Character("p") if event.state == ElementState::Pressed => {
							println!("{}", grid_to_string(&automaton.grid));
						}
						_ => (),
					},
					_ => (),
				}
			}
		})
		.unwrap();
}

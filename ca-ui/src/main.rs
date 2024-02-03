use std::time::Instant;

use cellular_automata::{game_of_life::GameOfLife, Automaton, World};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
	window::WindowBuilder,
};

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;
const SCALE: usize = 1;

fn main() {
	// let middle_idx = WIDTH * HEIGHT / 2 + WIDTH / 2;
	// let mut automaton: Sir<WIDTH, HEIGHT> = Sir::new(
	// 	World::from_fn(|i| {
	// 		if i == middle_idx {
	// 			SirState::Infected
	// 		} else {
	// 			SirState::default()
	// 		}
	// 	}),
	// 	0.15,
	// );

	let automaton = GameOfLife;
	let mut world: World<_, WIDTH, HEIGHT> = World::from_fn(|_| rand::random(), true);

	let mut running = true;
	let mut speed = 8;

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
						let start_time = Instant::now();
						if running {
							for _ in 0..speed {
								world.step(&automaton);
							}
							window.request_redraw();
						}
						let update_time = Instant::now();
						world.draw(pixels.frame_mut(), WIDTH * SCALE, SCALE);
						pixels.render().unwrap();
						let draw_time = Instant::now();
						println!(
							"update: {:3}ms ({:2}ms/i)\ttotal: {:3}ms",
							update_time.duration_since(start_time).as_millis(),
							update_time.duration_since(start_time).as_millis() / speed,
							draw_time.duration_since(start_time).as_millis()
						)
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
						Key::Named(NamedKey::ArrowDown) if event.state == ElementState::Pressed => {
							speed = 1.max(speed / 2);
						}
						Key::Named(NamedKey::ArrowUp) if event.state == ElementState::Pressed => {
							speed *= 2;
						}
						Key::Named(NamedKey::ArrowRight)
							if event.state == ElementState::Pressed && !running =>
						{
							world.step(&automaton);
							window.request_redraw();
						}
						_ => (),
					},
					_ => (),
				}
			}
		})
		.unwrap();
}

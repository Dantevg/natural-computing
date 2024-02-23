use std::time::Instant;

use cellular_automata::world::World;
use cpm::{
	example::{ExampleCPM, ExampleCell},
	CPM,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
	window::WindowBuilder,
};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const SCALE: usize = 3;

fn main() {
	// let middle_idx = WIDTH * HEIGHT / 2 + WIDTH / 2;
	// let mut world: World<WIDTH, HEIGHT, _> = World::from_fn(
	// 	|_| {
	// 		if rand::random::<bool>() {
	// 			CPMCell(0xff)
	// 		} else {
	// 			CPMCell(0x00)
	// 		}
	// 	},
	// 	true,
	// );
	let mut world: World<WIDTH, HEIGHT, _> = World::default();
	// let mut rng = rand::thread_rng();
	// for i in 1..32 {
	// 	let x = (96 + rng.gen_range(0..32) * 2) as usize;
	// 	let y = (96 + rng.gen_range(0..32) * 2) as usize;
	// 	world.img[(x, y)] = ExampleCell(i);
	// }
	for x in 0..5 {
		for y in 0..5 {
			world.img[(x * WIDTH / 5, y * HEIGHT / 5)] =
				ExampleCell(x as u8 * 5 + y as u8 + 1, 80, true);
		}
	}
	for x in 0..13 {
		for y in 0..13 {
			world.img[(x * WIDTH / 13 + 8, y * HEIGHT / 13 + 8)] =
				ExampleCell(x as u8 * 13 + y as u8 + 1, 80, false);
		}
	}
	let mut model = ExampleCPM::new(
		20.0,  // these comments are here to keep the parameters on separate lines
		20.0,  //
		200,   //
		50.0,  //
		180,   //
		2.0,   //
		80,    //
		300.0, //
		&world,
	);

	let mut running = false;
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
								model.step(&mut world);
							}
							window.request_redraw();
						}
						let update_time = Instant::now();
						world.draw(pixels.frame_mut(), WIDTH * SCALE, SCALE);
						// TODO: render iter number, add keybind to save, option to save after #iter
						pixels.render().unwrap();
						let draw_time = Instant::now();
						if running {
							println!(
								"update: {:3}ms ({:2}ms/i)\ttotal: {:3}ms",
								update_time.duration_since(start_time).as_millis(),
								update_time.duration_since(start_time).as_millis() / speed,
								draw_time.duration_since(start_time).as_millis()
							)
						}
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
							let start_time = Instant::now();
							model.step(&mut world);
							let update_time = Instant::now();
							println!(
								"update: {:4}us",
								update_time.duration_since(start_time).as_micros()
							);
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

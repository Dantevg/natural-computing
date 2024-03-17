use std::time::Instant;

use boids::world::World;
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, KeyEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement as _,
	window::{Window, WindowBuilder},
};

use crate::{cli::Args, draw};

pub struct Ui {
	pub window: Window,
	pub pixels: Pixels,
	pub running: bool,
	pub prev_update: Instant,
}

pub fn handle_window_event(
	ui: &mut Ui,
	world: &mut World,
	args: &Args,
	event: WindowEvent,
	window_target: &EventLoopWindowTarget<()>,
) {
	match event {
		WindowEvent::CloseRequested => window_target.exit(),
		WindowEvent::RedrawRequested => {
			if ui.running {
				let start_time = Instant::now();
				world.update((start_time - ui.prev_update).as_secs_f32());
				ui.prev_update = start_time;
				println!(
					"{:3}ms",
					Instant::now().duration_since(start_time).as_millis(),
				);
			}
			draw(&world, ui.pixels.frame_mut(), args.width);
			ui.pixels.render().unwrap();
			ui.window.request_redraw();
		}
		WindowEvent::KeyboardInput {
			event: event @ KeyEvent {
				state: ElementState::Pressed,
				..
			},
			..
		} => match event.key_without_modifiers().as_ref() {
			Key::Named(NamedKey::Space) => {
				ui.running = !ui.running;
				if ui.running {
					ui.window.request_redraw();
				}
			}
			Key::Named(NamedKey::ArrowRight) if !ui.running => {
				let start_time = Instant::now();
				world.update(0.0167);
				ui.prev_update = start_time;
				println!(
					"{:3}ms",
					Instant::now().duration_since(start_time).as_millis()
				);
				ui.window.request_redraw();
			}
			Key::Named(NamedKey::Enter) if !ui.running => {
				let start_time = Instant::now();
				for _ in 0..100 {
					world.update(0.01);
				}
				println!(
					"{:4}ms",
					Instant::now().duration_since(start_time).as_millis()
				);
				ui.window.request_redraw();
				ui.prev_update = Instant::now();
			}
			_ => (),
		},
		_ => (),
	}
}

pub fn init_ui(args: &Args) -> (Ui, EventLoop<()>) {
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let window = WindowBuilder::new()
		.with_title("Boids")
		.with_inner_size(PhysicalSize::new(args.width, args.height))
		.build(&event_loop)
		.unwrap();

	let window_size = window.inner_size();
	let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
	let pixels = Pixels::new(args.width, args.height, surface_texture).unwrap();

	let ui = Ui {
		window,
		pixels,
		running: true,
		prev_update: Instant::now(),
	};

	(ui, event_loop)
}

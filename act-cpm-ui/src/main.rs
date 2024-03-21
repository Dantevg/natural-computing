// Do not create console window for release builds
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod act_cpm;
mod cli;

use std::{
	fs::{create_dir, File},
	path::Path,
	time::Instant,
};

use act_cpm::{ActCPM, ActCPMCell};

use cellular_automata::{cpm::CPM, world::World};

use clap::Parser as _;
use cli::Args;
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, KeyEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
	keyboard::{Key, ModifiersState, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
	window::{Window, WindowBuilder},
};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const SCALE: usize = 4;

struct Ui<Cpm: CPM<WIDTH, HEIGHT>> {
	pub world: World<WIDTH, HEIGHT, Cpm::C>,
	pub model: Cpm,
	pub window: Window,
	pub pixels: Pixels,

	pub running: bool,
	pub speed: u32,
	pub iter: u32,
}

fn main() {
	let args = Args::parse();

	let (mut ui, event_loop) = init(&args);

	let mut modifiers = ModifiersState::default();

	event_loop
		.run(move |event, window_target| {
			if let Event::WindowEvent { event, .. } = event {
				handle_window_event(&mut ui, &args, event, window_target, &mut modifiers);
			}
		})
		.unwrap();
}

fn handle_window_event<Cpm: CPM<WIDTH, HEIGHT>>(
	ui: &mut Ui<Cpm>,
	args: &Args,
	event: WindowEvent,
	window_target: &EventLoopWindowTarget<()>,
	modifiers: &mut ModifiersState,
) {
	match event {
		WindowEvent::CloseRequested => window_target.exit(),
		WindowEvent::RedrawRequested => {
			let start_time = Instant::now();
			if ui.running {
				for _ in 0..ui.speed {
					ui.model.step(&mut ui.world);
					ui.iter += 1;
					if args.output.is_some()
						&& args.save_interval > 0
						&& ui.iter % args.save_interval == 0
					{
						save_image(ui, args);
					}
					if args.iter.is_some_and(|max_iter| ui.iter >= max_iter) {
						window_target.exit();
					}
				}
				ui.window.request_redraw();
			}
			let update_time = Instant::now();
			ui.world.draw(ui.pixels.frame_mut(), WIDTH * SCALE, SCALE);
			ui.pixels.render().unwrap();
			let draw_time = Instant::now();
			if ui.running {
				if args.verbose {
					println!(
						"update:{:3}ms ({:2}ms/i) total:{:3}ms",
						update_time.duration_since(start_time).as_millis(),
						update_time.duration_since(start_time).as_millis() / u128::from(ui.speed),
						draw_time.duration_since(start_time).as_millis()
					);
				} else {
					print!(
						"\rupdate:{:3}ms ({:2}ms/i) i:{:6}",
						update_time.duration_since(start_time).as_millis(),
						update_time.duration_since(start_time).as_millis() / u128::from(ui.speed),
						ui.iter,
					);
				}
			}
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
			Key::Named(NamedKey::ArrowDown) => {
				ui.speed = 1.max(ui.speed / 2);
			}
			Key::Named(NamedKey::ArrowUp) => {
				ui.speed *= 2;
			}
			Key::Named(NamedKey::ArrowRight) if !ui.running => {
				let start_time = Instant::now();
				ui.model.step(&mut ui.world);
				ui.iter += 1;
				let update_time = Instant::now();
				println!(
					"update: {:4}us",
					update_time.duration_since(start_time).as_micros()
				);
				ui.window.request_redraw();
			}
			Key::Character("s") if modifiers.control_key() => {
				save_image(ui, args);
			}
			_ => (),
		},
		WindowEvent::ModifiersChanged(new_modifiers) => {
			*modifiers = new_modifiers.state();
		}
		_ => (),
	}
}

fn save_image<Cpm: CPM<WIDTH, HEIGHT>>(ui: &mut Ui<Cpm>, args: &Args) {
	ui.world.draw(ui.pixels.frame_mut(), WIDTH * SCALE, SCALE);
	save(
		&args.output.clone().unwrap_or_default(),
		&ui.pixels,
		ui.iter,
	);
	// Clear current line and put cursor at beginning of line (in case of non-verbose output)
	println!("\x1b[1K\rSaved image {}.png", ui.iter);
}

#[must_use]
fn init(args: &Args) -> (Ui<ActCPM>, EventLoop<()>) {
	let world = create_world(args);

	#[rustfmt::skip] // to keep the parameters on separate lines
	let model = ActCPM::new(
		args.temp,
		args.l_adhesion,
		args.volume,
		args.l_volume,
		args.perimeter,
		args.l_perimeter,
		args.max_act,
		args.l_act,
		&world,
	);

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

	let window_size = window.inner_size();
	let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
	let pixels = Pixels::new(
		(WIDTH * SCALE) as u32,
		(HEIGHT * SCALE) as u32,
		surface_texture,
	)
	.unwrap();

	let ui = Ui {
		world,
		model,
		window,
		pixels,
		running: true,
		speed: 16,
		iter: 0,
	};

	(ui, event_loop)
}

#[must_use]
fn create_world(args: &Args) -> World<200, 200, ActCPMCell> {
	let mut world: World<WIDTH, HEIGHT, _> = World::default();
	for x in 0..args.obstacle_grid {
		for y in 0..args.obstacle_grid {
			world.img[(
				x * WIDTH / args.obstacle_grid,
				y * HEIGHT / args.obstacle_grid,
			)] = ActCPMCell((x * args.obstacle_grid + y + 1) as u8, 80, true);
		}
	}
	for x in 0..args.cell_grid {
		for y in 0..args.cell_grid {
			world.img[(
				x * WIDTH / args.cell_grid + 8,
				y * HEIGHT / args.cell_grid + 8,
			)] = ActCPMCell((x * args.cell_grid + y + 1) as u8, 80, false);
		}
	}
	world
}

fn save(dir: &Path, pixels: &Pixels, i: u32) {
	if !dir.is_dir() && !dir.as_os_str().is_empty() {
		create_dir(dir).unwrap();
	}
	let filename = dir.join(format!("{i}.png"));
	let file = File::create(filename).unwrap();

	let mut png_encoder = png::Encoder::new(file, (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(pixels.frame()).unwrap();
}

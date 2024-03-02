mod cli;

use std::{
	fs::{create_dir, File},
	time::Instant,
};

use cellular_automata::world::World;

use clap::Parser as _;
use cli::Args;
use cpm::{
	example::{ExampleCPM, ExampleCell},
	CPMCell, CPM,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
	keyboard::{Key, NamedKey},
	platform::modifier_supplement::KeyEventExtModifierSupplement,
	window::{Window, WindowBuilder},
};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const SCALE: usize = 3;

struct Ui<C: CPMCell, Cpm: CPM<WIDTH, HEIGHT, C = C>> {
	pub world: World<WIDTH, HEIGHT, C>,
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

	event_loop
		.run(move |event, window_target| {
			if let Event::WindowEvent { event, .. } = event {
				handle_window_event(&mut ui, &args, event, window_target)
			}
		})
		.unwrap();
}

fn handle_window_event<C: CPMCell, Cpm: CPM<WIDTH, HEIGHT, C = C>>(
	ui: &mut Ui<C, Cpm>,
	args: &Args,
	event: WindowEvent,
	window_target: &EventLoopWindowTarget<()>,
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
						ui.world.draw(ui.pixels.frame_mut(), WIDTH * SCALE, SCALE);
						save(args, &ui.pixels, ui.iter);
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
						"update: {:3}ms ({:2}ms/i)\ttotal: {:3}ms",
						update_time.duration_since(start_time).as_millis(),
						update_time.duration_since(start_time).as_millis() / ui.speed as u128,
						draw_time.duration_since(start_time).as_millis()
					)
				} else {
					print!(
						"update: {:3}ms ({:2}ms/i)\ti: {}\r",
						update_time.duration_since(start_time).as_millis(),
						update_time.duration_since(start_time).as_millis() / ui.speed as u128,
						ui.iter,
					)
				}
			}
		}
		WindowEvent::KeyboardInput { event, .. } => match event.key_without_modifiers().as_ref() {
			Key::Named(NamedKey::Space) if event.state == ElementState::Pressed => {
				ui.running = !ui.running;
				if ui.running {
					ui.window.request_redraw();
				}
			}
			Key::Named(NamedKey::ArrowDown) if event.state == ElementState::Pressed => {
				ui.speed = 1.max(ui.speed / 2);
			}
			Key::Named(NamedKey::ArrowUp) if event.state == ElementState::Pressed => {
				ui.speed *= 2;
			}
			Key::Named(NamedKey::ArrowRight)
				if event.state == ElementState::Pressed && !ui.running =>
			{
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
			_ => (),
		},
		_ => (),
	}
}

fn init(args: &Args) -> (Ui<ExampleCell, ExampleCPM>, EventLoop<()>) {
	let world = create_world(args);
	let model = ExampleCPM::new(
		args.temp,        // these comments are here to keep the parameters on separate lines
		args.l_adhesion,  //
		args.volume,      //
		args.l_volume,    //
		args.perimeter,   //
		args.l_perimeter, //
		args.max_act,     //
		args.l_act,       //
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
		speed: 1,
		iter: 0,
	};

	(ui, event_loop)
}

fn create_world(args: &Args) -> World<200, 200, ExampleCell> {
	let mut world: World<WIDTH, HEIGHT, _> = World::default();
	for x in 0..args.obstacle_grid / 2 {
		for y in 0..args.obstacle_grid {
			world.img[(
				x * WIDTH / args.obstacle_grid,
				y * HEIGHT / args.obstacle_grid,
			)] = ExampleCell((x * args.obstacle_grid + y + 1) as u8, 80, true);
		}
	}
	for x in 0..args.cell_grid {
		for y in 0..args.cell_grid {
			world.img[(
				x * WIDTH / args.cell_grid + 8,
				y * HEIGHT / args.cell_grid + 8,
			)] = ExampleCell((x * args.cell_grid + y + 1) as u8, 80, false);
		}
	}
	world
}

fn save(args: &Args, pixels: &Pixels, i: u32) {
	let mut filename = args.output.as_ref().unwrap().clone();
	if !filename.is_dir() {
		create_dir(filename.clone()).unwrap();
	}
	filename.push(format!("{i}.png"));
	let file = File::create(filename).unwrap();

	let mut png_encoder = png::Encoder::new(file, (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(&pixels.frame()).unwrap();
}

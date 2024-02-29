use std::{
	fs::{create_dir, File},
	io::BufWriter,
	path::PathBuf,
	time::Instant,
};

use cellular_automata::world::World;
use clap::Parser;
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

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
	#[arg(short, long, value_name = "FILE")]
	output: Option<PathBuf>,

	#[arg(long, value_name = "ITERATIONS", default_value_t = 0)]
	save_interval: u32,

	#[arg(long, default_value_t = 13)]
	cell_grid: usize,

	#[arg(long, default_value_t = 5)]
	obstacle_grid: usize,

	#[arg(long, default_value_t = 20.0)]
	temp: f32,

	#[arg(long, default_value_t = 20.0)]
	l_adhesion: f32,

	#[arg(long, default_value_t = 200)]
	volume: u32,

	#[arg(long, default_value_t = 50.0)]
	l_volume: f32,

	#[arg(long, default_value_t = 180)]
	perimeter: u32,

	#[arg(long, default_value_t = 2.0)]
	l_perimeter: f32,

	#[arg(long, default_value_t = 80)]
	max_act: u8,

	#[arg(long, default_value_t = 300.0)]
	l_act: f32,

	#[arg(short, long, default_value_t = false)]
	verbose: bool,
}

fn main() {
	let args = Args::parse();

	let mut world: World<WIDTH, HEIGHT, _> = World::default();
	for x in 0..args.obstacle_grid {
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
	let mut model = ExampleCPM::new(
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

	let mut running = true;
	let mut speed = 1;
	let mut iter = 0;

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
								iter += 1;
								if args.output.is_some()
									&& args.save_interval > 0 && iter % args.save_interval == 0
								{
									save(&args, &pixels, iter);
								}
							}
							window.request_redraw();
						}
						let update_time = Instant::now();
						world.draw(pixels.frame_mut(), WIDTH * SCALE, SCALE);
						pixels.render().unwrap();
						let draw_time = Instant::now();
						if running {
							if args.verbose {
								println!(
									"update: {:3}ms ({:2}ms/i)\ttotal: {:3}ms",
									update_time.duration_since(start_time).as_millis(),
									update_time.duration_since(start_time).as_millis() / speed,
									draw_time.duration_since(start_time).as_millis()
								)
							} else {
								print!(
									"update: {:3}ms ({:2}ms/i)\ti: {iter}\r",
									update_time.duration_since(start_time).as_millis(),
									update_time.duration_since(start_time).as_millis() / speed,
								)
							}
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
							iter += 1;
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

fn save(args: &Args, pixels: &Pixels, i: u32) {
	let mut filename = args.output.as_ref().unwrap().clone();
	if !filename.is_dir() {
		create_dir(filename.clone()).unwrap();
	}
	filename.push(format!("{i}.png"));
	let file = File::create(filename).unwrap();
	let ref mut file_writer = BufWriter::new(file);

	let mut png_encoder =
		png::Encoder::new(file_writer, (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(&pixels.frame()).unwrap();
}

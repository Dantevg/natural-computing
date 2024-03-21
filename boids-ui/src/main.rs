mod cli;
mod ui;

use std::fs::File;

use boids::{world::World, Params};
use clap::Parser as _;
use cli::{Args, Cli};

use winit::event::Event;

fn main() {
	let args = Args::parse();

	let params = Params {
		alignment_strength: args.alignment,
		cohesion_strength: args.cohesion,
		separation_strength: args.separation,
	};
	let mut world: World = World::new(args.width, args.height, args.n_boids, params);

	if let Some(iter) = args.iter {
		let mut cli = Cli::new(&args);
		for _ in 0..iter {
			world.update(0.01);

			match args.log {
				Some(cli::LogType::Order) => println!("{}", world.order()),
				Some(cli::LogType::NNDist) => println!(
					"{}",
					world
						.nearest_neighbour_distances()
						.iter()
						.map(f32::to_string)
						.collect::<Vec<String>>()
						.join(",")
				),
				_ => (),
			}
		}

		if args.output.is_some() {
			save_image(&world, &mut cli.frame, &args);
		}
	} else {
		let (mut ui, event_loop) = ui::init(&args);

		event_loop
			.run(move |event, window_target| {
				if let Event::WindowEvent { event, .. } = event {
					ui::handle_window_event(&mut ui, &mut world, &args, event, window_target);
				}
			})
			.unwrap();
	}
}

fn save_image(world: &World, frame: &mut [u8], args: &Args) {
	draw(world, frame, args.width);
	let path = &args.output.clone().unwrap_or_default();
	let file = File::create(path).unwrap();

	let mut png_encoder = png::Encoder::new(file, args.width, args.height);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(frame).unwrap();
}

fn draw(world: &World, frame: &mut [u8], width: u32) {
	for pixel in frame.chunks_exact_mut(4) {
		pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
	}

	for boid in world.boids.iter() {
		let idx = (boid.pos.x as usize + boid.pos.y as usize * width as usize) * 4;
		frame[idx..idx + 4].copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
	}
}

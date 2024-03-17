use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
	version,
	about = "Boids

\x1b[1;4mKey bindings:\x1b[0m
	Space        toggle pause
	Arrow Right  single step (when paused)"
)]
pub struct Args {
	/// Where to save the output image after `--iter` steps.
	#[arg(short, long, value_name = "PATH")]
	pub output: Option<PathBuf>,

	/// Stop after this many simulation steps.
	#[arg(short, long, value_name = "ITER")]
	pub iter: Option<u32>,

	/// The width of the world.
	#[arg(short, long, default_value_t = 500)]
	pub width: u32,

	/// The height of the world.
	#[arg(short, long, default_value_t = 500)]
	pub height: u32,

	/// The number of boids
	#[arg(short, long, default_value_t = 300)]
	pub n_boids: u32,

	/// The strength of the alignment boid parameter.
	#[arg(long, default_value_t = 0.1)]
	pub alignment: f32,

	/// The strength of the cohesion boid parameter.
	#[arg(long, default_value_t = 0.005)]
	pub cohesion: f32,

	/// The strength of the separation boid parameter.
	#[arg(long, default_value_t = 1.0)]
	pub separation: f32,

	/// Log frame times.
	#[arg(short, long, default_value_t = false)]
	pub verbose: bool,
}

pub struct Cli {
	pub frame: Box<[u8]>,
}

impl Cli {
	pub fn new(args: &Args) -> Self {
		Self {
			frame: vec![0; (args.width * args.height * 4) as usize].into_boxed_slice(),
		}
	}
}

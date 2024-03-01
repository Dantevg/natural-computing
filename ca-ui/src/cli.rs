use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
	/// Path to the output directory. Also specify --save-interval.
	#[arg(short, long, value_name = "FILE", requires = "save_interval")]
	pub output: Option<PathBuf>,

	/// Interval (in simulation steps) in which to export images. Also specify --output.
	#[arg(
		long,
		value_name = "ITER",
		default_value_t = 0,
		hide_default_value = true,
		requires = "output"
	)]
	pub save_interval: u32,

	/// Stop after this many simulation steps.
	#[arg(short, long, value_name = "ITER")]
	pub iter: Option<u32>,

	/// Number of cells in one direction of the grid (total = cell-grid²).
	#[arg(long, value_name = "HOR_RES", default_value_t = 13)]
	pub cell_grid: usize,

	/// Number of obstacles in one direction of the grid (total = obstacle-grid²).
	#[arg(long, value_name = "HOR_RES", default_value_t = 5)]
	pub obstacle_grid: usize,

	/// Simulation temperature.
	#[arg(long, default_value_t = 20.0)]
	pub temp: f32,

	/// λ adhesion.
	#[arg(long, default_value_t = 20.0)]
	pub l_adhesion: f32,

	/// Target volume in number of pixels.
	#[arg(long, value_name = "PIXELS", default_value_t = 200)]
	pub volume: u32,

	/// λ volume.
	#[arg(long, default_value_t = 50.0)]
	pub l_volume: f32,

	/// Target perimeter in number of pixel edges.
	#[arg(long, value_name = "EDGES", default_value_t = 180)]
	pub perimeter: u32,

	/// λ perimeter.
	#[arg(long, default_value_t = 2.0)]
	pub l_perimeter: f32,

	/// Max act value.
	#[arg(long, default_value_t = 80)]
	pub max_act: u8,

	/// λ-act.
	#[arg(long, default_value_t = 300.0)]
	pub l_act: f32,

	/// Log frame times.
	#[arg(short, long, default_value_t = false)]
	pub verbose: bool,
}

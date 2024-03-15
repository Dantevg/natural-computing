use core::array;

use crate::{boid::Boid, Params};

pub const COHESION_RADIUS: f32 = 50.0;
pub const SEPARATION_RADIUS: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct World<const N_BOIDS: usize> {
	pub width: u32,
	pub height: u32,
	pub boids: [Boid; N_BOIDS],
	pub params: Params,
}

impl<const N_BOIDS: usize> World<N_BOIDS> {
	/// Creates a [`World`] with the given `width` and `height`, filled with
	/// randomly initialised [`Boid`]s.
	pub fn new(width: u32, height: u32, params: Params) -> Self {
		Self {
			width,
			height,
			boids: array::from_fn(|_| Boid::random(width, height)),
			params,
		}
	}

	/// Updates all [`Boid`]s in this [`World`] at once.
	///
	/// `dt` is the time in seconds between this update and the previous update.
	pub fn update(&mut self, dt: f32) {
		let boids = self.boids.clone();
		for mut boid in boids {
			boid.update(&self, dt);
		}
	}

	/// Returns all [`Boid`]s that are within a `radius` of the given `boid`.
	/// This includes the `boid` itself.
	///
	/// TODO: optimize using a K-d tree for example.
	pub fn neighbours(&self, boid: &Boid, radius: f32) -> Vec<&Boid> {
		self.boids
			.iter()
			.filter(|other| (boid.pos - other.pos).square_length() <= radius * radius)
			.collect()
	}

	/// Returns the order parameter, which is the average normalised velocity
	/// of the [`Boid`]s in this [`World`].
	pub fn order(&self) -> f32 {
		todo!()
	}

	/// Returns for each [`Boid`] the distance to its nearest neighbour.
	pub fn nearest_neighbour_distances(&self) -> Box<[f32; N_BOIDS]> {
		todo!()
	}

	pub fn draw(&self, frame: &mut [u8], frame_width: usize) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = i % frame_width;
			let y = i / frame_width;

			// if x < W && y < H {
			// 	pixel.copy_from_slice(&self.img[(x, y)].colour());
			// }
		}
	}
}

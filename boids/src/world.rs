use core::cmp::Ordering;

use euclid::default::Vector2D;

use crate::{boid::Boid, Params};

pub const COHESION_RADIUS: f32 = 50.0;
pub const SEPARATION_RADIUS: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct World {
	pub width: u32,
	pub height: u32,
	pub n_boids: u32,
	pub boids: Box<[Boid]>,
	pub params: Params,
}

impl World {
	/// Creates a [`World`] with the given `width` and `height`, filled with
	/// randomly initialised [`Boid`]s.
	#[must_use]
	pub fn new(width: u32, height: u32, n_boids: u32, params: Params) -> Self {
		Self {
			width,
			height,
			n_boids,
			boids: (0..n_boids)
				.map(|_| Boid::random(width, height))
				.collect::<Vec<Boid>>()
				.into_boxed_slice(),
			params,
		}
	}

	/// Updates all [`Boid`]s in this [`World`] at once.
	///
	/// `dt` is the time in seconds between this update and the previous update.
	pub fn update(&mut self, dt: f32) {
		let mut boids = self.boids.clone();
		for boid in boids.iter_mut() {
			boid.update(self, dt);
		}
		self.boids = boids;
	}

	/// Returns all [`Boid`]s that are within a `radius` of the given `boid`.
	/// This includes the `boid` itself.
	///
	/// TODO: optimize using a K-d tree for example.
	#[must_use]
	pub fn neighbours(&self, boid: &Boid, radius: f32) -> Vec<&Boid> {
		self.boids
			.iter()
			.filter(|other| (boid.pos - other.pos).square_length() <= radius * radius)
			.collect()
	}

	/// Returns the order metric, which is the average normalised velocity of
	/// the [`Boid`]s in this [`World`].
	#[must_use]
	pub fn order(&self) -> f32 {
		self.boids
			.iter()
			.map(Boid::dir)
			.sum::<Vector2D<f32>>()
			.length() / self.boids.len() as f32
	}

	/// Returns for each [`Boid`] the distance to its nearest neighbour.
	#[must_use]
	pub fn nearest_neighbour_distances(&self) -> Box<[f32]> {
		self.boids
			.iter()
			.map(|boid| {
				self.boids
					.iter()
					.filter(|&other| boid != other)
					.map(|other| self.dist_sq_wrapping(boid, other))
					.min_by(non_partial_cmp)
					.unwrap_or(0.0)
			})
			.collect()
	}

	/// Returns the distance between boids `a` and `b` in a wrapping world, such
	/// that two boids at opposite edges are regarded as close together.
	#[must_use]
	fn dist_sq_wrapping(&self, a: &Boid, b: &Boid) -> f32 {
		let mut dx = (b.pos.x - a.pos.x).abs();
		let mut dy = (b.pos.y - a.pos.y).abs();

		if dx > self.width as f32 / 2.0 {
			dx = self.width as f32 - dx;
		}
		if dy > self.height as f32 / 2.0 {
			dy = self.height as f32 - dy;
		}

		dx * dx + dy * dy
	}
}

/// Wraps a [`PartialOrd`] in a non-partial [`Ord`] by considering
/// non-comparable elements to be equal.
fn non_partial_cmp<T: PartialOrd>(a: &T, b: &T) -> Ordering {
	PartialOrd::partial_cmp(a, b).unwrap_or(Ordering::Equal)
}

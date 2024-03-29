use core::f32::consts::PI;

use euclid::{
	default::{Point2D, Vector2D},
	Angle,
};
use rand::Rng;

use crate::world::{filter_neighbours, World, COHESION_RADIUS, SEPARATION_RADIUS};

pub const SPEED: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boid {
	pub pos: Point2D<f32>,
	pub dir: Vector2D<f32>,
}

impl Boid {
	/// Creates a new [`Boid`] at the given position and direction vector.
	#[must_use]
	pub fn new(pos: Point2D<f32>, dir: Vector2D<f32>) -> Self {
		Self { pos, dir }
	}

	/// Creates a new [`Boid`] at a random position within the given `width` and
	/// `height`, and at a random angle.
	#[must_use]
	pub fn random(width: u32, height: u32) -> Self {
		let mut rng = rand::thread_rng();
		Self {
			pos: Point2D::new(
				rng.gen_range(0..width) as f32,
				rng.gen_range(0..height) as f32,
			),
			dir: Vector2D::from_angle_and_length(Angle::radians(rng.gen_range(0.0..PI * 2.0)), 1.0),
		}
	}

	/// Update this [`Boid`]'s position and angle according to boid rules.
	///
	/// `dt` is the time in seconds between this update and the previous update.
	pub fn update(&mut self, world: &World, dt: f32) {
		let neighbours = world.neighbours(self, COHESION_RADIUS);
		let alignment = self.alignment(&neighbours);
		let cohesion = self.cohesion(&neighbours);

		let too_close_neighbours = filter_neighbours(&neighbours, self.pos, SEPARATION_RADIUS);
		let separation = self.separation(&too_close_neighbours);

		self.dir = lerp_vecs(
			vec![self.dir, alignment, cohesion, separation],
			vec![
				1.0,
				world.params.alignment_strength,
				world.params.cohesion_strength,
				world.params.separation_strength,
			],
		)
		.normalize();

		self.pos += self.dir * SPEED * dt;

		// Wrap around
		self.pos = Point2D::new(
			self.pos.x.rem_euclid(world.width as f32),
			self.pos.y.rem_euclid(world.height as f32),
		);
	}

	/// Returns the alignment vector (the average angle) of this boid's
	/// neighbours.
	#[must_use]
	#[allow(clippy::unused_self)]
	fn alignment(&self, neighbours: &[Boid]) -> Vector2D<f32> {
		neighbours
			.iter()
			.map(|boid| boid.dir)
			.sum::<Vector2D<f32>>()
			/ neighbours.len() as f32
	}

	/// Returns the cohesion vector (pointing to the centre-point) of this boid's
	/// neighbours.
	#[must_use]
	fn cohesion(&self, neighbours: &[Boid]) -> Vector2D<f32> {
		let avg_pos = neighbours
			.iter()
			.map(|boid| boid.pos.to_vector())
			.sum::<Vector2D<f32>>()
			/ neighbours.len() as f32;
		avg_pos.to_point() - self.pos
	}

	/// Returns the separation vector (pointing away from the centre-point) of
	/// this boid's nearer neighbourhood.
	#[must_use]
	fn separation(&self, neighbours: &[Boid]) -> Vector2D<f32> {
		neighbours
			.iter()
			.filter(|&boid| boid != self)
			.map(|boid| {
				let away = self.pos - boid.pos;
				away / away.square_length()
			})
			.sum()
	}
}

#[must_use]
fn lerp_vecs(vecs: Vec<Vector2D<f32>>, ts: Vec<f32>) -> Vector2D<f32> {
	debug_assert_eq!(vecs.len(), ts.len());
	vecs.into_iter().zip(ts).map(|(vec, t)| vec * t).sum()
}

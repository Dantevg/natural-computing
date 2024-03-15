pub mod boid;
pub mod world;

#[derive(Debug, Clone)]
pub struct Params {
	pub alignment_strength: f32,
	pub cohesion_strength: f32,
	pub separation_strength: f32,
}

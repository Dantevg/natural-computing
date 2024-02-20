use cellular_automata::world::World;

use crate::CPM;

pub trait ActCell {
	fn get_activity(&self) -> u8;
}

pub trait Act<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
	Self::C: ActCell,
{
	fn get_act_penalty(&self, activity_delta: f32) -> f32;

	/// Returns the geometric mean of the activity in the neighbourhood of a cell.
	fn gm_act(&self, world: &World<W, H, Self::C>, idx: usize) -> f32 {
		let cell = world.get_cell(idx);
		let activities: Vec<_> = world
			.get_neighbours_idx(idx)
			.iter()
			.filter_map(|&neigh_idx| {
				let neigh = world.get_cell(neigh_idx);
				if neigh == cell {
					Some(neigh.get_activity() as f32)
				} else {
					None
				}
			})
			.collect();
		geometric_mean(&activities)
	}

	/// Returns the delta act energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn act_delta(
		&self,
		world: &World<W, H, Self::C>,
		_src: Self::C,
		_dest: Self::C,
		src_idx: usize,
		dest_idx: usize,
	) -> f32 {
		self.get_act_penalty(self.gm_act(world, src_idx) - self.gm_act(world, dest_idx))
	}
}

fn geometric_mean(slice: &[f32]) -> f32 {
	slice
		.iter()
		.fold(1.0, |a, b| a * b)
		.powf(1.0 / slice.len() as f32)
}

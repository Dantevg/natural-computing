use cellular_automata::world::{Coord, World};

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
	fn gm_act(&self, world: &World<W, H, Self::C>, idx: Coord) -> f32 {
		let cell = world.get_cell(idx);
		world
			.get_neighbours(idx)
			.iter()
			.filter_map(|&neigh| {
				if neigh == cell {
					Some(neigh.get_activity() as f32)
				} else {
					None
				}
			})
			.geometric_mean()
	}

	/// Returns the delta act energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn act_delta(
		&self,
		world: &World<W, H, Self::C>,
		_src: Self::C,
		_dest: Self::C,
		src_idx: Coord,
		dest_idx: Coord,
	) -> f32 {
		self.get_act_penalty(self.gm_act(world, src_idx) - self.gm_act(world, dest_idx))
	}
}

trait GeometricMean {
	fn geometric_mean(self) -> f32;
}

impl<T: Iterator<Item = f32>> GeometricMean for T {
	fn geometric_mean(self) -> f32 {
		let (len, x) = self.fold((0, 1.0), |(n, a), b| (n + 1, a * b));
		x.powf(1.0 / len as f32)
	}
}

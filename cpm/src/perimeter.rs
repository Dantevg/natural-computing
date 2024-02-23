use cellular_automata::world::{Coord, World};

use crate::CPM;

pub trait Perimeter<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_perimeter_penalty(&self, cell: Self::C, perimeter: u32) -> f32;

	/// Returns the perimeter in the number of grid cells for a single cell, if
	/// that grid cell were to have the given `state`.
	fn perimeter(&self, world: &World<W, H, Self::C>, idx: Coord, state: Self::C) -> u32;

	/// Returns the delta perimeter energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn perimeter_delta(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		src_idx: Coord,
		dest_idx: Coord,
	) -> f32 {
		let neighbourhood = world.get_neighbours(dest_idx);
		let mut src_perim_delta = 0;
		let mut dest_perim_delta = 0;
		for n in neighbourhood {
			src_perim_delta += if n == src { -1 } else { 1 };
			dest_perim_delta += if n == dest { 1 } else { -1 };
		}

		let src_perim = self.perimeter(world, src_idx, src) as i32;
		let dest_perim = self.perimeter(world, dest_idx, dest) as i32;
		let src_gain = self.get_perimeter_penalty(src, (src_perim + src_perim_delta) as u32)
			- self.get_perimeter_penalty(src, src_perim as u32);
		let dest_loss = self.get_perimeter_penalty(dest, (dest_perim + dest_perim_delta) as u32)
			- self.get_perimeter_penalty(dest, dest_perim as u32);
		src_gain + dest_loss
	}
}

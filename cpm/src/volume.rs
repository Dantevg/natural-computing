use cellular_automata::world::{Coord, World};

use crate::CPM;

pub trait Volume<const W: usize, const H: usize>
where
	Self: CPM<W, H>,
{
	fn get_volume_penalty(&self, cell: Self::C, volume: u32) -> f32;

	/// Returns the volume in the number of grid cells for a single cell, if
	/// that grid cell were to have the given `state`.
	fn volume(&self, world: &World<W, H, Self::C>, idx: Coord, state: Self::C) -> u32;

	/// Returns the delta volume energy for copying the cell at `src_idx` into
	/// `dest_idx`.
	fn volume_delta(
		&self,
		world: &World<W, H, Self::C>,
		src: Self::C,
		dest: Self::C,
		src_idx: Coord,
		dest_idx: Coord,
	) -> f32 {
		let src_vol = self.volume(world, src_idx, src);
		let dest_vol = self.volume(world, dest_idx, dest);
		let src_gain =
			self.get_volume_penalty(src, src_vol + 1) - self.get_volume_penalty(src, src_vol);
		let dest_loss = self.get_volume_penalty(dest, dest_vol.saturating_sub(1))
			- self.get_volume_penalty(dest, dest_vol);
		src_gain + dest_loss
	}
}

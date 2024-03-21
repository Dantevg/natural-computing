use cellular_automata::{
	cpm::{
		act::{Act, ActCell},
		adhesion::Adhesion,
		cell_perimeters::CellPerimeters,
		cell_volumes::CellVolumes,
		perimeter::Perimeter,
		volume::Volume,
		CPMCell, CPM,
	},
	world::{Coord, World},
	Cell,
};

#[derive(Clone, Copy, Default, Eq, Debug)]
pub struct ActCPMCell(
	/// Cell ID
	pub u8,
	/// Act value
	pub u8,
	/// Is obstacle
	pub bool,
);

impl ActCPMCell {
	#[inline(always)]
	#[must_use]
	pub fn is_obstacle(self) -> bool {
		self.2
	}
}

impl PartialEq for ActCPMCell {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.2 == other.2
	}
}

impl Cell for ActCPMCell {
	#[inline(always)]
	fn colour(&self) -> [u8; 4] {
		if self.is_bg() {
			[0xff, 0xff, 0xff, 0xff]
		} else if self.is_obstacle() {
			[0x88, 0x88, 0x88, 0xff]
		} else {
			[self.1 * (255 / 80), 0x00, 0x00, 0xff]
		}
	}
}

impl CPMCell for ActCPMCell {
	const MAX_ID: usize = u8::MAX as usize * 2;

	#[inline(always)]
	fn is_bg(&self) -> bool {
		self.0 == 0
	}

	#[inline(always)]
	fn id(&self) -> usize {
		self.0 as usize + (usize::from(self.2) * 256)
	}
}

impl ActCell for ActCPMCell {
	#[inline(always)]
	fn get_activity(&self) -> u8 {
		self.1
	}
}

pub struct ActCPM {
	temperature: f32,
	adhesion_penalty: f32,
	target_volume: u32,
	lambda_volume: f32,
	target_perimeter: u32,
	lambda_perimeter: f32,
	max_act: u8,
	lambda_act: f32,
	cell_volumes: CellVolumes,
	cell_perimeters: CellPerimeters,
}

impl ActCPM {
	#[must_use]
	pub fn new<const W: usize, const H: usize>(
		temperature: f32,
		adhesion_penalty: f32,
		target_volume: u32,
		lambda_volume: f32,
		target_perimeter: u32,
		lambda_perimeter: f32,
		max_act: u8,
		lambda_act: f32,
		world: &World<W, H, ActCPMCell>,
	) -> Self {
		Self {
			temperature,
			adhesion_penalty,
			target_volume,
			lambda_volume,
			target_perimeter,
			lambda_perimeter,
			max_act,
			lambda_act,
			cell_volumes: CellVolumes::from_world(world),
			cell_perimeters: CellPerimeters::from_world(world),
		}
	}
}

impl<const W: usize, const H: usize> CPM<W, H> for ActCPM {
	type C = ActCPMCell;

	#[inline(always)]
	fn get_temperature(&self) -> f32 {
		self.temperature
	}

	fn update(
		&mut self,
		world: &World<W, H, ActCPMCell>,
		src: ActCPMCell,
		dest: ActCPMCell,
		src_idx: Coord,
		dest_idx: Coord,
	) -> ActCPMCell {
		self.cell_volumes
			.update(world, src, dest, src_idx, dest_idx);
		self.cell_perimeters
			.update(world, src, dest, src_idx, dest_idx);
		if src.is_bg() {
			src
		} else {
			ActCPMCell(src.0, self.max_act, src.2)
		}
	}

	fn hamiltonian(
		&self,
		world: &World<W, H, ActCPMCell>,
		src: ActCPMCell,
		dest: ActCPMCell,
		src_idx: Coord,
		dest_idx: Coord,
	) -> f32 {
		let adhesion = self.adhesion_delta(world, src, dest, src_idx, dest_idx);
		let volume = self.volume_delta(world, src, dest, src_idx, dest_idx);
		let perimeter = self.perimeter_delta(world, src, dest, src_idx, dest_idx);
		let act = self.act_delta(world, src, dest, src_idx, dest_idx);
		adhesion + volume + perimeter + act
	}

	fn after_step(&mut self, world: &mut World<W, H, Self::C>) {
		for cell in world.img.pixels_mut() {
			if cell.1 > 0 {
				cell.1 -= 1;
			}
		}
		self.cell_volumes.recalculate(world);
		self.cell_perimeters.recalculate(world);
	}
}

impl<const W: usize, const H: usize> Adhesion<W, H> for ActCPM {
	#[inline(always)]
	fn get_adhesion_penalty(&self, a: ActCPMCell, b: ActCPMCell) -> f32 {
		if a.is_obstacle() != b.is_obstacle() && !a.is_bg() && !b.is_bg() {
			// TODO: un-hardcode this
			self.adhesion_penalty * 10.0
		} else {
			self.adhesion_penalty
		}
	}
}

impl<const W: usize, const H: usize> Volume<W, H> for ActCPM {
	fn get_volume_penalty(&self, cell: ActCPMCell, volume: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else if cell.is_obstacle() {
			// TODO: un-hardcode this
			self.lambda_volume * (volume as f32 - self.target_volume as f32 / 2.0).powi(2)
		} else {
			self.lambda_volume * (volume as f32 - self.target_volume as f32).powi(2)
		}
	}

	fn volume(&self, _world: &World<W, H, ActCPMCell>, _idx: Coord, state: ActCPMCell) -> u32 {
		self.cell_volumes.get(state)
	}
}

impl<const W: usize, const H: usize> Perimeter<W, H> for ActCPM {
	fn get_perimeter_penalty(&self, cell: ActCPMCell, perimeter: u32) -> f32 {
		if cell.is_bg() {
			0.0 // no penalty for background cells
		} else if cell.is_obstacle() {
			// TODO: un-hardcode this
			self.lambda_perimeter * (perimeter as f32).powi(2)
		} else {
			self.lambda_perimeter * (perimeter as f32 - self.target_perimeter as f32).powi(2)
		}
	}

	fn perimeter(&self, _world: &World<W, H, ActCPMCell>, _idx: Coord, state: ActCPMCell) -> u32 {
		self.cell_perimeters.get(state)
	}
}

impl<const W: usize, const H: usize> Act<W, H> for ActCPM {
	fn get_act_penalty(&self, activity_delta: f32) -> f32 {
		if self.max_act > 0 {
			-(self.lambda_act / f32::from(self.max_act)) * activity_delta
		} else {
			f32::INFINITY
		}
	}
}

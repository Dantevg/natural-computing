pub mod ca;
pub mod cpm;
pub mod world;

pub trait Cell
where
	Self: Clone + Copy + PartialEq + Eq,
{
	#[must_use]
	fn colour(&self) -> [u8; 4];
}

impl Cell for bool {
	fn colour(&self) -> [u8; 4] {
		if *self {
			[0xff, 0xff, 0xff, 0xff]
		} else {
			[0x00, 0x00, 0x00, 0xff]
		}
	}
}

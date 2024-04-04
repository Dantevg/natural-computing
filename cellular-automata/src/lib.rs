pub mod ca;
pub mod cpm;
pub mod world;

pub trait Cell: Clone + Copy + PartialEq + Eq {
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

/// Returns the number of neighbours where `filter` returns `true`. Does not
/// count the middle cell.
fn count_neighbours<C: Copy, F>(neighbourhood: [C; 9], filter: F) -> u8
where
	F: Fn(C) -> bool,
{
	u8::from(filter(neighbourhood[0]))
		+ u8::from(filter(neighbourhood[1]))
		+ u8::from(filter(neighbourhood[2]))
		+ u8::from(filter(neighbourhood[3]))
		// do not include self, only count neighbours
		+ u8::from(filter(neighbourhood[5]))
		+ u8::from(filter(neighbourhood[6]))
		+ u8::from(filter(neighbourhood[7]))
		+ u8::from(filter(neighbourhood[8]))
}

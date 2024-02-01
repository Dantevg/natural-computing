pub struct Automaton<S, const W: usize, const H: usize> {
	pub grid: Box<[[S; W]; H]>,
	pub transition: fn(state: &S, neighbours: [[Option<&S>; 3]; 3]) -> S,
}

impl<S, const W: usize, const H: usize> Automaton<S, W, H>
where
	S: Clone,
{
	pub fn new(transition: fn(state: &S, neighbours: [[Option<&S>; 3]; 3]) -> S) -> Self
	where
		S: Clone + Copy + Default + core::fmt::Debug,
	{
		Automaton {
			grid: vec![[Default::default(); W]; H]
				.into_boxed_slice()
				.try_into()
				.unwrap(),
			transition,
		}
	}

	pub fn step(&mut self) {
		let mut new_grid = self.grid.clone();
		for y in 0..H {
			for x in 0..W {
				new_grid[y][x] =
					(self.transition)(&self.grid[y][x], self.get_neighbours(x as i32, y as i32));
			}
		}
		self.grid = new_grid;
	}

	#[allow(unused)]
	fn get_opt(&self, x: usize, y: usize) -> Option<&S> {
		self.grid.get(y).and_then(|row| row.get(x))
	}

	fn get_wrapping(&self, x: i32, y: i32) -> &S {
		&self.grid[y.rem_euclid(H as i32) as usize][x.rem_euclid(W as i32) as usize]
	}

	fn get_neighbours(&self, x: i32, y: i32) -> [[Option<&S>; 3]; 3] {
		[
			[
				Some(self.get_wrapping(x - 1, y - 1)),
				Some(self.get_wrapping(x, y - 1)),
				Some(self.get_wrapping(x + 1, y - 1)),
			],
			[
				Some(self.get_wrapping(x - 1, y)),
				None, // exclude self from neighbours
				Some(self.get_wrapping(x + 1, y)),
			],
			[
				Some(self.get_wrapping(x - 1, y + 1)),
				Some(self.get_wrapping(x, y + 1)),
				Some(self.get_wrapping(x + 1, y + 1)),
			],
		]
	}
}

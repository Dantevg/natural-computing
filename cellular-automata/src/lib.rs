pub struct Automaton<S, const W: usize, const H: usize>
where
	S: Clone,
{
	pub grid: Box<[[S; W]; H]>,
	pub transition: fn(state: &S, neighbours: [[Option<&S>; 3]; 3]) -> S,
}

impl<S, const W: usize, const H: usize> Automaton<S, W, H>
where
	S: Clone,
{
	pub fn step(&mut self) {
		let mut new_grid = self.grid.clone();
		for y in 0..W {
			for x in 0..H {
				new_grid[y][x] = (self.transition)(&self.grid[y][x], self.get_neighbours(x, y));
			}
		}
		self.grid = new_grid;
	}

	fn get_opt(&self, x: usize, y: usize) -> Option<&S> {
		self.grid.get(y).and_then(|row| row.get(x))
	}

	fn get_neighbour(&self, x: usize, y: usize, x_off: isize, y_off: isize) -> Option<&S> {
		let x = x.checked_add_signed(x_off)?;
		let y = y.checked_add_signed(y_off)?;
		self.get_opt(x, y)
	}

	fn get_neighbours(&self, x: usize, y: usize) -> [[Option<&S>; 3]; 3] {
		[
			[
				self.get_neighbour(x, y, -1, -1),
				self.get_neighbour(x, y, 0, -1),
				self.get_neighbour(x, y, 1, -1),
			],
			[
				self.get_neighbour(x, y, -1, 0),
				None, // exclude self from neighbours
				self.get_neighbour(x, y, 1, 0),
			],
			[
				self.get_neighbour(x, y, -1, 1),
				self.get_neighbour(x, y, 0, 1),
				self.get_neighbour(x, y, 1, 1),
			],
		]
	}
}

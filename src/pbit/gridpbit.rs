#[derive(Clone, Copy)]
pub struct GridPbit {
	row: usize,
	col: usize,
}

impl GridPbit {
	pub fn new(row: usize, col: usize) -> GridPbit {
		GridPbit { row, col }
	}

	pub fn coordinates(&self) -> (usize, usize) {
		(self.row, self.col)
	}
}
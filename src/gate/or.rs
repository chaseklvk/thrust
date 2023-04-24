use crate::{gate::Gate, pbit::GridPbit};
use na::{DMatrix};

pub struct Or {
	pbits: Vec<GridPbit>,
	weight: DMatrix<i32>,
	bias: Vec<i32>,
}

impl Or {
	pub fn new(m1: GridPbit, m2: GridPbit, m3: GridPbit) -> Or {
		let weight = DMatrix::<i32>::from_vec(3, 3, vec![0, -1, 2, -1, 0, 2, 2, 2, 0]);
		let bias = vec![-1, -1, 2];

		Or { pbits: vec![m1, m2, m3], weight, bias }
	}
}

impl Gate for Or {
	fn pbits(&self) -> &Vec<GridPbit> {
		&self.pbits
	}

	fn shape(&self) -> (usize, usize) {
		(3, 3)
	}

	fn weight(&self) -> &DMatrix<i32> {
		&self.weight
	}

	fn bias(&self) -> &Vec<i32> {
		&self.bias
	}
}
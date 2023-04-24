use na::DMatrix;
use crate::gate::Gate;
use crate::pbit::GridPbit;

pub struct Copy {
	pbits: Vec<GridPbit>,
	weight: DMatrix<i32>,
	bias: Vec<i32>,
}

impl Copy {
	pub fn new(m1: GridPbit, m2: GridPbit) -> Copy {
		let weight = DMatrix::<i32>::from_vec(2, 2, vec![0, 1, 1, 0]);
		let bias = vec![0, 0];
		Copy { pbits: vec![m1, m2], weight, bias }
	}
}

impl Gate for Copy {
	fn pbits(&self) -> &Vec<GridPbit> {
		&self.pbits
	}

	fn shape(&self) -> (usize, usize) {
		(2, 2)
	}

	fn weight(&self) -> &DMatrix<i32> {
		&self.weight
	}

	fn bias(&self) -> &Vec<i32> {
		&self.bias
	}
}
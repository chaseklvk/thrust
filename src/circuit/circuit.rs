use na::{DMatrix};
use crate::gate::Gate;

pub struct Circuit {
	pub gates: Vec<Box<dyn Gate>>,
	pub weight: DMatrix<i32>,
	pub bias: Vec<i32>,
}

impl Circuit {
	pub fn new() -> Circuit {
		Circuit { gates: Vec::new(), weight: DMatrix::<i32>::from_vec(1, 1, vec![0]), bias: Vec::new() }
	}

	pub fn from_vector(gates: Vec<Box<dyn Gate>>) -> Circuit {
		Circuit { gates, weight: DMatrix::<i32>::from_vec(1, 1, vec![0]), bias: Vec::new() }
	}

	pub fn append(&mut self, gate: Box<dyn Gate>) {
		self.gates.push(gate);
	}

	pub fn compile(&mut self) {
		// Compose circuit, nothing to do if size is 1
		if self.gates.len() == 1 {
			self.weight = self.gates[0].weight().clone();
			self.bias = self.gates[0].bias().clone();
			return;
		}

		// Compute the grid space
		let (max_row, max_col) = self.grid_space();
		let to_linear = |row: usize, col: usize| -> usize {
			row * (max_col + 1) + col
		};

		// Compilation parameters
		let mut visited = vec![false; (max_row + 1) * (max_col + 1)];
		let mut weight: DMatrix<i32> = self.gates[0].weight().clone();
		let mut bias: Vec<i32> = self.gates[0].bias().clone();


		// Visit each gate
		for gate in self.gates.iter() {
			for pbit in gate.pbits() {
				let index = to_linear(pbit.coordinates().0, pbit.coordinates().1);
				let other_weight = gate.weight();
				let other_bias = gate.bias();

				if visited[index] {
					(weight, bias) = self.compose_weights_and_bias(index, &weight, &bias, other_weight, other_bias);
				}

				visited[index] = true;
			}
		}

		self.weight = weight;
		self.bias = bias;
	}

	fn compose_weights_and_bias(
		&self, 
		linear_index: usize, 
		weight: &DMatrix<i32>, 
		bias: &Vec<i32>,
		other_weight: &DMatrix<i32>,
		other_bias: &Vec<i32>
	) -> (DMatrix<i32>, Vec<i32>) {
		// Figure out new size of weight matrix, these are square matrices
		let (other_row, _) = other_weight.shape();

		let additional_columns = other_row - 1;

		let mut new_weight = weight.clone();
		let mut new_other_weight = other_weight.clone();

		for i in 0..additional_columns {
			 new_weight = new_weight.insert_column(linear_index + i + 1, 0);
			 new_weight = new_weight.insert_row(linear_index + i + 1, 0);
			 new_other_weight = new_other_weight.insert_column(i, 0);
			 new_other_weight = new_other_weight.insert_row(i, 0);
		}

		// Compose weights
		new_weight += new_other_weight;

		let mut new_bias = bias.clone();
		let new_other_bias = other_bias.clone();

		new_bias[linear_index] += new_other_bias[0];
		for (i, bias_value) in new_other_bias.iter().enumerate() {
			if i == 0 {
				continue;
			}

			new_bias.push(*bias_value);
		}

		(new_weight, new_bias)
	}

	fn grid_space(&self) -> (usize, usize) {
		let mut x = 0;
		let mut y = 0;
		for gate in self.gates.iter() {
			for pbit in gate.pbits().iter() {
				if pbit.coordinates().0 > x {
					x = pbit.coordinates().0;
				}
				if pbit.coordinates().1 > y {
					y = pbit.coordinates().1;
				}
			}
		}

		(x, y)
	}
}
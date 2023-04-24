use na::DMatrix;

use crate::pbit::GridPbit;

pub trait Gate {
	fn pbits(&self) -> &Vec<GridPbit>;
	fn shape(&self) -> (usize, usize);
	fn weight(&self)-> &DMatrix<i32>;
	fn bias(&self) -> &Vec<i32>;
}
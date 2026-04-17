//! extensions

use crate::float_type::float;



pub trait IndexOfMaxMin<T> {
	fn index_of_max(&self) -> Option<usize>;
	fn index_of_min(&self) -> Option<usize>;
}
impl<T: PartialOrd> IndexOfMaxMin<T> for Vec<T> {
	fn index_of_max(&self) -> Option<usize> {
		let mut option_index_of_max = None;
		for i in 0..self.len() {
			match option_index_of_max {
				None => {
					option_index_of_max = Some(i);
				}
				Some(index_of_max) if self[i] > self[index_of_max] => {
					option_index_of_max = Some(i);
				}
				_ => {}
			}
		}
		option_index_of_max
	}
	fn index_of_min(&self) -> Option<usize> {
		let mut option_index_of_min = None;
		for i in 0..self.len() {
			match option_index_of_min {
				None => {
					option_index_of_min = Some(i);
				}
				Some(index_of_min) if self[i] < self[index_of_min] => {
					option_index_of_min = Some(i);
				}
				_ => {}
			}
		}
		option_index_of_min
	}
}



pub trait Into_<T> {
	fn into_(self) -> T;
}
impl<T> Into_<T> for T { fn into_(self) -> T { self } }
// impl<T, S: Into<T>> MyInto<T> for S { fn into_(self) -> T { self.into() } }
impl Into_<float> for i32 { fn into_(self) -> float { self as float } }



pub trait AddSubModulo {
	fn add_mod(self, n: Self, modulo: Self) -> Self;
	fn sub_mod(self, n: Self, modulo: Self) -> Self;
	fn inc_mod(self, modulo: Self) -> Self;
	fn dec_mod(self, modulo: Self) -> Self;
}
impl AddSubModulo for u32 {
	fn add_mod(self, n: Self, modulo: Self) -> Self {
		(self + n).rem_euclid(modulo)
	}
	fn sub_mod(self, n: Self, modulo: Self) -> Self {
		((self as i32) - (n as i32)).rem_euclid(modulo as i32) as u32
	}
	fn inc_mod(self, modulo: Self) -> Self {
		self.add_mod(1, modulo)
	}
	fn dec_mod(self, modulo: Self) -> Self {
		self.sub_mod(1, modulo)
	}
}



pub trait BoolSelect<T> {
	fn select(self, true_val: T, false_val: T) -> T;
}
impl<T> BoolSelect<T> for bool {
	fn select(self, true_val: T, false_val: T) -> T {
		if self { true_val } else { false_val }
	}
}


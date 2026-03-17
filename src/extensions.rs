//! extensions



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

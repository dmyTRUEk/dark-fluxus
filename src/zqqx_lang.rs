//! zqqx lang

use crate::float;



pub struct ZqqxLangChar {
	values: [float; 25],
}
impl ZqqxLangChar {
	pub fn new(values: [float; 25]) -> Self {
		Self { values }
	}
}


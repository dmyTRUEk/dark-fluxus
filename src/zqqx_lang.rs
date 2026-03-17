//! zqqx lang

use crate::extensions::IndexOfMaxMin;



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZqqxChar {
	values: [i8; 25],
}
impl ZqqxChar {
	pub fn new(values: [i8; 25]) -> Self {
		Self { values }
	}

	pub fn quantize(&self) -> [u8; 25] {
		self.values.map(|v| (v >= 0) as u8)
	}
}



pub struct ZqqxLang {
	chars: Vec<ZqqxChar>,
}
impl ZqqxLang {
	pub fn new() -> Self {
		// TODO: some preseted?
		Self { chars: vec![] }
	}

	pub fn add_or_quantize(&mut self, new_char: ZqqxChar) -> ZqqxChar {
		if self.chars.is_empty() {
			self.chars.push(new_char);
			return new_char;
		}
		let similarities: Vec<i32> = self.chars.iter()
			.map(|char| {
				let mut sum: i32 = 0;
				for (a, b) in char.values.iter().zip(new_char.values) {
					sum += (*a as i32) * (b as i32);
				}
				sum
			})
			.collect();
		let index_of_best = similarities.index_of_max().unwrap();
		// dbg!(similarities[index_of_best]);
		// NOTE: higher threshold => more characters
		const THRESHOLD: i32 = 50_000;
		if similarities[index_of_best] > THRESHOLD {
			self.chars[index_of_best]
		} else {
			self.chars.push(new_char);
			// dbg!(self.chars.len());
			new_char
		}
	}
}


//! frame buffer (vec of pixels)

use std::ops::Index;

use crate::colors::{BLACK, Color};



pub struct FrameBuffer {
	pub w: u32, pub h: u32,
	pub buf: Vec<u32>,
}

impl FrameBuffer {
	pub fn new(w: u32, h: u32) -> Self {
		Self { w, h, buf: vec![BLACK.0; (w as usize) * (h as usize)] }
	}

	pub fn get_wh(&self) -> (u32, u32) {
		(self.w, self.h)
	}

	pub fn is_resized(&self, (w, h): (usize, usize)) -> bool {
		(w as u32, h as u32) != (self.w, self.h)
	}
	/// returns true if resized
	pub fn resize(&mut self, (w, h): (usize, usize)) -> bool {
		let is_resized = self.is_resized((w, h));
		if is_resized {
			self.resize_unchecked((w, h));
		}
		is_resized
	}
	pub fn resize_unchecked(&mut self, (w, h): (usize, usize)) {
		self.w = w as u32;
		self.h = h as u32;
		self.buf.resize(w * h, BLACK.0);
	}

	pub fn clear(&mut self) {
		self.fill(BLACK);
	}
	pub fn fill(&mut self, color: Color) {
		self.buf.fill(color.0);
	}
}

// impl Index<(u32, u32)> for FrameBuffer {
// 	type Output = u32;
// 	fn index(&self, (w, h): (u32, u32)) -> &Self::Output {
// 		self.buf[]
// 	}
// }


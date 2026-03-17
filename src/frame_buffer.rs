//! frame buffer (vec of pixels)

use crate::colors::{BLACK, Color};



pub struct FrameBuffer {
	pub buf: Vec<u32>,
}
impl FrameBuffer {
	pub fn new(w: u32, h: u32) -> Self {
		Self { buf: vec![BLACK.0; (w as usize) * (h as usize)] }
	}

	pub fn clear(&mut self) {
		self.fill(BLACK);
	}
	pub fn fill(&mut self, color: Color) {
		self.buf.fill(color.0);
	}
}


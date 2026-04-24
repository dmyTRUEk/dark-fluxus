//! Color via 3 * u8

use wgpu::Color;

use crate::float_type::float;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorU8 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl ColorU8 {
	pub const BLACK  : Self = Self::new(0, 0, 0);
	pub const GRAY   : Self = Self::new(128, 128, 128);
	pub const WHITE  : Self = Self::new(255, 255, 255);
	pub const RED    : Self = Self::new(255, 0, 0);
	pub const GREEN  : Self = Self::new(0, 255, 0);
	pub const BLUE   : Self = Self::new(0, 0, 255);
	pub const CYAN   : Self = Self::new(0, 255, 255);
	pub const MAGENTA: Self = Self::new(255, 0, 255);
	pub const YELLOW : Self = Self::new(255, 255, 0);
	pub const GRAY_64: Self = Self::new(64, 64, 64);
	pub const GRAY_32: Self = Self::new(32, 32, 32);

	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b }
	}
	pub const fn from_int(_n: u32) -> Self {
		todo!()
	}
	pub const fn gray(n: u8) -> Self {
		Self::new(n, n, n)
	}

	pub fn to_array(self) -> [float; 3] {
		[self.r, self.g, self.b].map(|c| (c as float) / 255.)
	}
}

// impl From<ColorU8> for Color {
// 	fn from(ColorU8 { r, g, b }: ColorU8) -> Self {
// 		Color {
// 			r: r as f64,
// 			g: g as f64,
// 			b: b as f64,
// 			a: 1.
// 		}
// 	}
// }


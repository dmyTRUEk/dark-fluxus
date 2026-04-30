//! Color via 3 * u8

use wgpu::Color;


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
	pub const GRAY_16: Self = Self::new(16, 16, 16);
	pub const GRAY_8 : Self = Self::new(8, 8, 8);
	pub const GRAY_4 : Self = Self::new(4, 4, 4);
	pub const GRAY_2 : Self = Self::new(2, 2, 2);
	pub const GRAY_1 : Self = Self::new(1, 1, 1);
	pub const SKY         : Self = Self::new(128, 255, 255);
	pub const PINK        : Self = Self::new(255, 128, 255);
	pub const LIGHT_YELLOW: Self = Self::new(255, 255, 128);
	pub const SKY_64 : Self = Self::new(64, 255, 255);
	pub const SKY_32 : Self = Self::new(32, 255, 255);
	pub const PINK_64: Self = Self::new(255, 64, 255);
	pub const PINK_32: Self = Self::new(255, 32, 255);
	pub const DARK_RED  : Self = Self::new(128, 0, 0);
	pub const DARK_GREEN: Self = Self::new(0, 128, 0);
	pub const DARK_BLUE : Self = Self::new(0, 0, 128);
	pub const DARK_RED_64  : Self = Self::new(64, 0, 0);
	pub const DARK_RED_32  : Self = Self::new(32, 0, 0);
	pub const DARK_GREEN_64: Self = Self::new(0, 64, 0);
	pub const DARK_GREEN_32: Self = Self::new(0, 32, 0);
	pub const DARK_BLUE_64 : Self = Self::new(0, 0, 64);
	pub const DARK_BLUE_32 : Self = Self::new(0, 0, 32);
	pub const ORANGE : Self = Self::new(255, 128, 0);
	pub const FUCHSIA: Self = Self::new(255, 0, 128);
	//pub const ?    : Self = Self::new(128, 255, 0);
	//pub const ?    : Self = Self::new(0, 255, 128);
	pub const PURPLE : Self = Self::new(128, 0, 255);
	//pub const ?    : Self = Self::new(0, 128, 255);
	pub const DARK_ORANGE : Self = Self::new(128, 64, 0);
	pub const DARK_FUCHSIA: Self = Self::new(128, 0, 64);
	pub const DARK_PURPLE : Self = Self::new(64, 0, 128);
	pub const DARK_ORANGE_64 : Self = Self::new(64, 32, 0);
	pub const DARK_ORANGE_32 : Self = Self::new(32, 16, 0);
	pub const DARK_FUCHSIA_64: Self = Self::new(64, 0, 32);
	pub const DARK_FUCHSIA_32: Self = Self::new(32, 0, 16);
	pub const DARK_PURPLE_64 : Self = Self::new(32, 0, 64);
	pub const DARK_PURPLE_32 : Self = Self::new(16, 0, 32);

	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b }
	}
	pub const fn from_int(n: u32) -> Self {
		Self {
			r: ((n & 0x00ff0000) >> 16) as u8,
			g: ((n & 0x0000ff00) >> 8) as u8,
			b: (n & 0x000000ff) as u8,
		}
	}
	pub const fn gray(n: u8) -> Self {
		Self::new(n, n, n)
	}

	pub fn to_array(self) -> [f32; 3] {
		[self.r, self.g, self.b].map(|c| (c as f32) / 255.)
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


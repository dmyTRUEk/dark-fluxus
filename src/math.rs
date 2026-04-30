//! math

use crate::math_aliases::{max, min};


pub fn lerp(t: f32, a: f32, b: f32) -> f32 { a + (b - a) * t }
pub fn unlerp(x: f32, a: f32, b: f32) -> f32 { (x - a) / (b - a) }

pub fn min_max(x: f32, y: f32) -> (f32, f32) { (min(x,y), max(x,y)) }

pub trait MoreClamps {
	fn clamp_0_1(self) -> Self;
	fn clamp_around_05(self, radius: Self) -> Self;
}
impl MoreClamps for f32 {
	fn clamp_0_1(self) -> Self {
		self.clamp(0., 1.)
	}
	fn clamp_around_05(self, radius: Self) -> Self {
		self.clamp(0.5 - radius, 0.5 + radius)
	}
}


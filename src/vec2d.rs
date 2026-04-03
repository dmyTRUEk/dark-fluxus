//! math vector 2d

use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use sdl3::render::FPoint;

use crate::float;



pub type Vec2f = Vec2d<float>;
pub type Vec2i = Vec2d<i32>;

#[macro_export] macro_rules! vec2 { ($x:expr, $y:expr) => { Vec2d::new($x, $y) }; }
#[macro_export] macro_rules! vec2x { ($x:expr) => { Vec2d::new($x, 0) }; }
#[macro_export] macro_rules! vec2y { ($y:expr) => { Vec2d::new(0, $y) }; }



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2d<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2d<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
	pub fn from(x: impl Into<T>, y: impl Into<T>) -> Self {
		Self { x: x.into(), y: y.into() }
	}
}



impl<T: Add<T, Output=T>> Add<Self> for Vec2d<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y)
	}
}
impl<T: Add<T, Output=T> + Clone> Add<T> for Vec2d<T> {
	type Output = Self;
	fn add(self, rhs: T) -> Self::Output {
		Self::new(self.x + rhs.clone(), self.y + rhs)
	}
}

impl<T: AddAssign<T>> AddAssign<Self> for Vec2d<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
impl<T: AddAssign<T> + Clone> AddAssign<T> for Vec2d<T> {
	fn add_assign(&mut self, rhs: T) {
		self.x += rhs.clone();
		self.y += rhs;
	}
}

// impl<T: Mul<T, Output=T>> Mul<Self> for Vec2d<T> {
// 	type Output = Self;
// 	fn mul(self, rhs: Self) -> Self::Output {
// 		Self::new(self.x * rhs.x, self.y * rhs.y)
// 	}
// }
impl<T: Mul<T, Output=T> + Clone> Mul<T> for Vec2d<T> {
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self::new(self.x * rhs.clone(), self.y * rhs)
	}
}

// impl<T: MulAssign<T>> MulAssign<Self> for Vec2d<T> {
// 	fn mul_assign(&mut self, rhs: Self) {
// 		self.x *= rhs.x;
// 		self.y *= rhs.y;
// 	}
// }
impl<T: MulAssign<T> + Clone> MulAssign<T> for Vec2d<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs.clone();
		self.y *= rhs;
	}
}

// impl<T: Div<T, Output=T>> Div<Self> for Vec2d<T> {
// 	type Output = Self;
// 	fn div(self, rhs: Self) -> Self::Output {
// 		Self::new(self.x / rhs.x, self.y / rhs.y)
// 	}
// }
impl<T: Div<T, Output=T> + Clone> Div<T> for Vec2d<T> {
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		Self::new(self.x / rhs.clone(), self.y / rhs)
	}
}





impl From<Vec2f> for FPoint {
	fn from(v: Vec2f) -> Self {
		FPoint::new(v.x, v.y)
	}
}
impl From<Vec2i> for FPoint {
	fn from(v: Vec2i) -> Self {
		FPoint::new(v.x as float, v.y as float)
	}
}


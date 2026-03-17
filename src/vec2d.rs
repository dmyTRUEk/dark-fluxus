//! vector 2 float

use std::ops::{Add, AddAssign, Div, Mul, MulAssign};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2d<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2d<T> {
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}



impl<T: Add<T, Output=T>> Add<Vec2d<T>> for Vec2d<T> {
	type Output = Vec2d<T>;
	fn add(self, rhs: Vec2d<T>) -> Self::Output {
		Vec2d::new(self.x + rhs.x, self.y + rhs.y)
	}
}
impl<T: Add<T, Output=T> + Clone> Add<T> for Vec2d<T> {
	type Output = Vec2d<T>;
	fn add(self, rhs: T) -> Self::Output {
		Vec2d::new(self.x + rhs.clone(), self.y + rhs)
	}
}

impl<T: AddAssign<T>> AddAssign<Vec2d<T>> for Vec2d<T> {
	fn add_assign(&mut self, rhs: Vec2d<T>) {
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

// impl<T: Mul<T, Output=T>> Mul<Vec2d<T>> for Vec2d<T> {
// 	type Output = Vec2d<T>;
// 	fn mul(self, rhs: Vec2d<T>) -> Self::Output {
// 		Vec2d::new(self.x * rhs.x, self.y * rhs.y)
// 	}
// }
impl<T: Mul<T, Output=T> + Clone> Mul<T> for Vec2d<T> {
	type Output = Vec2d<T>;
	fn mul(self, rhs: T) -> Self::Output {
		Vec2d::new(self.x * rhs.clone(), self.y * rhs)
	}
}

// impl<T: MulAssign<T>> MulAssign<Vec2d<T>> for Vec2d<T> {
// 	fn mul_assign(&mut self, rhs: Vec2d<T>) {
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

// impl<T: Div<T, Output=T>> Div<Vec2d<T>> for Vec2d<T> {
// 	type Output = Vec2d<T>;
// 	fn div(self, rhs: Vec2d<T>) -> Self::Output {
// 		Vec2d::new(self.x / rhs.x, self.y / rhs.y)
// 	}
// }
impl<T: Div<T, Output=T> + Clone> Div<T> for Vec2d<T> {
	type Output = Vec2d<T>;
	fn div(self, rhs: T) -> Self::Output {
		Vec2d::new(self.x / rhs.clone(), self.y / rhs)
	}
}


//! vector 3 float

use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use crate::{float, vec2d::Vec2d};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3d<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3d<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}

	pub fn yz(self) -> Vec2d<T> { Vec2d::new(self.y, self.z) }
	pub fn xz(self) -> Vec2d<T> { Vec2d::new(self.x, self.z) }
	pub fn xy(self) -> Vec2d<T> { Vec2d::new(self.x, self.y) }
}

impl<T: Copy + Add<T, Output=T> + Mul<T, Output=T>> Vec3d<T> {
	pub fn dot(self, other: Self) -> T {
		self.x*other.x + self.y*other.y + self.z*other.z
	}
	pub fn project_2d(self, a: Vec3d<T>, b: Vec3d<T>) -> Vec2d<T> {
		Vec2d::new(self.dot(a), self.dot(b))
	}
	pub fn project_3d(self, a: Vec3d<T>, b: Vec3d<T>, c: Vec3d<T>) -> Vec3d<T> {
		Vec3d::new(self.dot(a), self.dot(b), self.dot(c))
	}
}

impl Vec3d<float> {
	pub fn norm2(self) -> float { self.dot(self) }
	pub fn norm(self) -> float { self.dot(self).sqrt() }
	pub fn normed(self) -> Self { self / self.norm() }
}



impl<T: Add<T, Output=T>> Add<Vec3d<T>> for Vec3d<T> {
	type Output = Vec3d<T>;
	fn add(self, rhs: Vec3d<T>) -> Self::Output {
		Vec3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}
impl<T: Add<T, Output=T> + Clone> Add<T> for Vec3d<T> {
	type Output = Vec3d<T>;
	fn add(self, rhs: T) -> Self::Output {
		Vec3d::new(self.x + rhs.clone(), self.y + rhs.clone(), self.z + rhs)
	}
}

impl<T: AddAssign<T>> AddAssign<Vec3d<T>> for Vec3d<T> {
	fn add_assign(&mut self, rhs: Vec3d<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}
impl<T: AddAssign<T> + Clone> AddAssign<T> for Vec3d<T> {
	fn add_assign(&mut self, rhs: T) {
		self.x += rhs.clone();
		self.y += rhs.clone();
		self.z += rhs;
	}
}

// impl<T: Mul<T, Output=T>> Mul<Vec3d<T>> for Vec3d<T> {
// 	type Output = Vec3d<T>;
// 	fn mul(self, rhs: Vec3d<T>) -> Self::Output {
// 		Vec3d::new(self.x * rhs.x, self.y * rhs.y)
// 	}
// }
impl<T: Mul<T, Output=T> + Clone> Mul<T> for Vec3d<T> {
	type Output = Vec3d<T>;
	fn mul(self, rhs: T) -> Self::Output {
		Vec3d::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
	}
}

// impl<T: MulAssign<T>> MulAssign<Vec3d<T>> for Vec3d<T> {
// 	fn mul_assign(&mut self, rhs: Vec3d<T>) {
// 		self.x *= rhs.x;
// 		self.y *= rhs.y;
// 		self.z *= rhs.z;
// 	}
// }
impl<T: MulAssign<T> + Clone> MulAssign<T> for Vec3d<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs.clone();
		self.y *= rhs.clone();
		self.z *= rhs;
	}
}

// impl<T: Div<T, Output=T>> Div<Vec3d<T>> for Vec3d<T> {
// 	type Output = Vec3d<T>;
// 	fn div(self, rhs: Vec3d<T>) -> Self::Output {
// 		Vec3d::new(self.x / rhs.x, self.y / rhs.y)
// 	}
// }
impl<T: Div<T, Output=T> + Clone> Div<T> for Vec3d<T> {
	type Output = Vec3d<T>;
	fn div(self, rhs: T) -> Self::Output {
		Vec3d::new(self.x / rhs.clone(), self.y / rhs.clone(), self.z / rhs)
	}
}


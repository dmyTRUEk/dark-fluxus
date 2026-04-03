//! math vector 3d

use std::{ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign}, process::Output};

use crate::{extensions::Into_, float, vec2d::Vec2d};



pub type Vec3f = Vec3d<float>;

#[macro_export] macro_rules! vec3 { ($x:expr, $y:expr, $z:expr $(,)?) => { Vec3d::from($x, $y, $z) }; }
#[macro_export] macro_rules! vec3x { ($x:expr) => { Vec3d::from($x, 0, 0) }; }
#[macro_export] macro_rules! vec3y { ($y:expr) => { Vec3d::from(0, $y, 0) }; }
#[macro_export] macro_rules! vec3z { ($z:expr) => { Vec3d::from(0, 0, $z) }; }
#[macro_export] macro_rules! vec3yz { ($y:expr, $z:expr) => { Vec3d::from(0, $y, $z) }; }
#[macro_export] macro_rules! vec3xz { ($x:expr, $z:expr) => { Vec3d::from($x, 0, $z) }; }
#[macro_export] macro_rules! vec3xy { ($x:expr, $y:expr) => { Vec3d::from($x, $y, 0) }; }



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3d<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3d<T> {
	pub const fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
	pub fn from(x: impl Into_<T>, y: impl Into_<T>, z: impl Into_<T>) -> Self {
		Self { x: x.into_(), y: y.into_(), z: z.into_() }
	}
	pub fn yz(self) -> Vec2d<T> { Vec2d::new(self.y, self.z) }
	pub fn xz(self) -> Vec2d<T> { Vec2d::new(self.x, self.z) }
	pub fn xy(self) -> Vec2d<T> { Vec2d::new(self.x, self.y) }
}

impl<T> Vec3d<T> where T: Copy + Add<T,Output=T> + Mul<T,Output=T> {
	pub fn dot(self, other: Self) -> T {
		self.x*other.x + self.y*other.y + self.z*other.z
	}
	pub fn project_2d(self, a: Self, b: Self) -> Vec2d<T> {
		Vec2d::new(self.dot(a), self.dot(b))
	}
	pub fn project_3d(self, a: Self, b: Self, c: Self) -> Self {
		Self::new(self.dot(a), self.dot(b), self.dot(c))
	}
}
impl<T> Vec3d<T> where T: Copy + Mul<T,Output=T> + Sub<T,Output=T> + Into_<T> {
	pub fn cross(self, other: Self) -> Self {
		Self::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
		)
	}
}

impl Vec3f {
	pub fn norm2(self) -> float { self.dot(self) }
	pub fn norm(self) -> float { self.dot(self).sqrt() }
	pub fn normed(self) -> Self { self / self.norm() }
	pub fn normlize(&mut self) { *self = self.normed() }
}



impl<T> Add<Self> for Vec3d<T> where T: Add<T,Output=T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}
impl<T> Add<T> for Vec3d<T> where T: Copy + Add<T,Output=T> {
	type Output = Self;
	fn add(self, rhs: T) -> Self::Output {
		Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
	}
}

impl<T> AddAssign<Self> for Vec3d<T> where T: AddAssign<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}
impl<T> AddAssign<T> for Vec3d<T> where T: Copy + AddAssign<T> {
	fn add_assign(&mut self, rhs: T) {
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
	}
}

impl<T> Sub<Self> for Vec3d<T> where T: Sub<T,Output=T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}
impl<T> Sub<T> for Vec3d<T> where T: Copy + Sub<T,Output=T> {
	type Output = Self;
	fn sub(self, rhs: T) -> Self::Output {
		Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
	}
}

impl<T> SubAssign<Self> for Vec3d<T> where T: SubAssign<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}
impl<T> SubAssign<T> for Vec3d<T> where T: Copy + SubAssign<T> {
	fn sub_assign(&mut self, rhs: T) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}

impl<T> Mul<Self> for Vec3d<T> where T: Copy + Add<T,Output=T> + Mul<T,Output=T> {
	type Output = T;
	fn mul(self, rhs: Self) -> Self::Output {
		self.dot(rhs)
	}
}
// impl<T> Mul<Self> for Vec3d<T> where T: Copy + Mul<T,Output=T> + Sub<T,Output=T> {
// 	type Output = Self;
// 	fn mul(self, rhs: Self) -> Self::Output {
// 		self.cross(rhs)
// 	}
// }
impl<T> Mul<T> for Vec3d<T> where T: Copy + Mul<T,Output=T> {
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}
// impl<T> Mul<Vec3d<T>> for T where T: Mul<T,Output=T> {
// 	type Output = Vec3d<T>;
// 	fn mul(self, rhs: Vec3d<T>) -> Self::Output {
// 		todo!()
// 	}
// }

// impl<T> MulAssign<Self> for Vec3d<T> where T: MulAssign<T> {
// 	fn mul_assign(&mut self, rhs: Self) {
// 		self.x *= rhs.x;
// 		self.y *= rhs.y;
// 		self.z *= rhs.z;
// 	}
// }
impl<T> MulAssign<T> for Vec3d<T> where T: Copy + MulAssign<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

// impl<T> Div<Self> for Vec3d<T> where T: Div<T,Output=T> {
// 	type Output = Self;
// 	fn div(self, rhs: Self) -> Self::Output {
// 		Self::new(self.x / rhs.x, self.y / rhs.y)
// 	}
// }
impl<T> Div<T> for Vec3d<T> where T: Copy + Div<T,Output=T> {
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}


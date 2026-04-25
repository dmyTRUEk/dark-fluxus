//! math vector 3d

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use rand::{RngExt, rngs::ThreadRng};

use crate::extensions::{BoolSelect, Into_};
use crate::math_aliases::{cos, sin};
use crate::vec2d::Vec2d;



pub type Vec3f = Vec3d<f32>;
pub type Vec3i = Vec3d<i32>;

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
	pub fn with_x(self, x: T) -> Self { Self { x, ..self } }
	pub fn with_y(self, y: T) -> Self { Self { y, ..self } }
	pub fn with_z(self, z: T) -> Self { Self { z, ..self } }
	pub fn yz(self) -> Vec2d<T> { Vec2d::new(self.y, self.z) }
	pub fn zy(self) -> Vec2d<T> { Vec2d::new(self.z, self.y) }
	pub fn xz(self) -> Vec2d<T> { Vec2d::new(self.x, self.z) }
	pub fn zx(self) -> Vec2d<T> { Vec2d::new(self.z, self.x) }
	pub fn xy(self) -> Vec2d<T> { Vec2d::new(self.x, self.y) }
	pub fn yx(self) -> Vec2d<T> { Vec2d::new(self.y, self.x) }
}

impl<T> Vec3d<T> where T: Add<T,Output=T> + Mul<T,Output=T> + Copy {
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
impl<T> Vec3d<T> where T: Mul<T,Output=T> + Sub<T,Output=T> + Into_<T> + Copy {
	pub fn cross(self, other: Self) -> Self {
		Self::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
		)
	}
}

impl Vec3f {
	pub const ORT_X: Self = Self::from_x(1.);
	pub const ORT_Y: Self = Self::from_y(1.);
	pub const ORT_Z: Self = Self::from_z(1.);
	pub const fn from_x(x: f32) -> Self { Self { x, y: 0., z: 0. } }
	pub const fn from_y(y: f32) -> Self { Self { x: 0., y, z: 0. } }
	pub const fn from_z(z: f32) -> Self { Self { x: 0., y: 0., z } }
	pub const fn from_yz(y: f32, z: f32) -> Self { Self { x: 0., y, z } }
	pub const fn from_xz(x: f32, z: f32) -> Self { Self { x, y: 0., z } }
	pub const fn from_xy(x: f32, y: f32) -> Self { Self { x, y, z: 0. } }
	pub fn random_unit_cube(rng: &mut ThreadRng) -> Self {
		Self {
			x: rng.random_range(-1. ..= 1.),
			y: rng.random_range(-1. ..= 1.),
			z: rng.random_range(-1. ..= 1.),
		}
	}
	pub fn random_unit(rng: &mut ThreadRng) -> Self {
		Self::random_unit_cube(rng).normed()
	}
	pub fn norm2(self) -> f32 { self.dot(self) }
	pub fn norm(self) -> f32 { self.norm2().sqrt() }
	pub fn normed(self) -> Self { self / self.norm() }
	pub fn normalize(&mut self) { *self = self.normed() }
	pub fn normed_to(self, len: f32) -> Self { self.normed() * len }
	pub fn normalize_to(&mut self, len: f32) { *self = self.normed_to(len) }
	pub fn dist2_to(self, other: Self) -> f32 { (self - other).norm2() }
	pub fn dist_to(self, other: Self) -> f32 { self.dist2_to(other).sqrt() }
	pub const fn _0yz(self) -> Self { Self { x: 0., ..self } }
	pub const fn x0z(self) -> Self { Self { y: 0., ..self } }
	pub const fn xy0(self) -> Self { Self { z: 0., ..self } }
	pub fn rotate_around(self, other: Self, angle: f32) -> Self {
		// src: https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space
		// src: https://en.wikipedia.org/wiki/Rodrigues'_rotation_formula
		self * cos(angle) + other.cross(self) * sin(angle) + other * (other * self) * (1. - cos(angle))
	}
	pub fn flip_x_if(self, condition: bool) -> Self { Self { x: condition.select(-self.x, self.x), ..self } }
	pub fn flip_y_if(self, condition: bool) -> Self { Self { y: condition.select(-self.y, self.y), ..self } }
	pub fn flip_z_if(self, condition: bool) -> Self { Self { z: condition.select(-self.z, self.z), ..self } }
}



impl<T> Add<Self> for Vec3d<T> where T: Add<T,Output=T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}
impl<T> Add<T> for Vec3d<T> where T: Add<T,Output=T> + Copy {
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
impl<T> AddAssign<T> for Vec3d<T> where T: AddAssign<T> + Copy {
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
impl<T> Sub<T> for Vec3d<T> where T: Sub<T,Output=T> + Copy {
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
impl<T> SubAssign<T> for Vec3d<T> where T: SubAssign<T> + Copy {
	fn sub_assign(&mut self, rhs: T) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}

impl<T> Mul<Self> for Vec3d<T> where T: Add<T,Output=T> + Mul<T,Output=T> + Copy {
	type Output = T;
	fn mul(self, rhs: Self) -> Self::Output {
		self.dot(rhs)
	}
}
// impl<T> Mul<Self> for Vec3d<T> where T: Sub<T,Output=T> + Mul<T,Output=T> + Copy {
// 	type Output = Self;
// 	fn mul(self, rhs: Self) -> Self::Output {
// 		self.cross(rhs)
// 	}
// }
impl<T> Mul<T> for Vec3d<T> where T: Mul<T,Output=T> + Copy {
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
impl<T> MulAssign<T> for Vec3d<T> where T: MulAssign<T> + Copy {
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
impl<T> Div<T> for Vec3d<T> where T: Div<T,Output=T> + Copy {
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}





impl<T> From<(T, T, T)> for Vec3d<T> {
	fn from((x, y, z): (T, T, T)) -> Self {
		Self { x, y, z }
	}
}
impl<T> From<Vec3d<T>> for (T, T, T) {
	fn from(v: Vec3d<T>) -> Self {
		(v.x, v.y, v.z)
	}
}


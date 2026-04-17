//! math vector 2d

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use rand::{RngExt, rngs::ThreadRng};
use sdl3::render::FPoint;

use crate::{extensions::Into_, float_type::float, vec3d::{Vec3d, Vec3f}};



pub type Vec2f = Vec2d<float>;
pub type Vec2i = Vec2d<i32>;

#[macro_export] macro_rules! vec2 { ($x:expr, $y:expr) => { Vec2d::new($x, $y) }; }
#[macro_export] macro_rules! vec2x { ($x:expr) => { Vec2d::from_x($x, 0) }; }
#[macro_export] macro_rules! vec2y { ($y:expr) => { Vec2d::from_y(0, $y) }; }



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2d<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2d<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
	pub fn from(x: impl Into_<T>, y: impl Into_<T>) -> Self {
		Self { x: x.into_(), y: y.into_() }
	}
	pub fn with_x(self, x: T) -> Self { Self { x, ..self } }
	pub fn with_y(self, y: T) -> Self { Self { y, ..self } }
	pub fn txy(self, t: T) -> Vec3d<T> { Vec3d::new(t, self.x, self.y) }
	pub fn tyx(self, t: T) -> Vec3d<T> { Vec3d::new(t, self.y, self.x) }
	pub fn xty(self, t: T) -> Vec3d<T> { Vec3d::new(self.x, t, self.y) }
	pub fn ytx(self, t: T) -> Vec3d<T> { Vec3d::new(self.y, t, self.x) }
	pub fn xyt(self, t: T) -> Vec3d<T> { Vec3d::new(self.x, self.y, t) }
	pub fn yxt(self, t: T) -> Vec3d<T> { Vec3d::new(self.y, self.x, t) }
}
impl<T> Vec2d<T> where T: Add<T,Output=T> + Mul<T,Output=T> + Copy {
	pub fn dot(self, other: Self) -> T {
		self.x*other.x + self.y*other.y
	}
}



impl Vec2f {
	pub const ORT_X: Self = Self::from_x(1.);
	pub const ORT_Y: Self = Self::from_y(1.);
	pub const fn from_x(x: float) -> Self { Self { x, y: 0. } }
	pub const fn from_y(y: float) -> Self { Self { x: 0., y } }
	pub fn random_unit_cube(rng: &mut ThreadRng) -> Self {
		Self {
			x: rng.random_range(-1. ..= 1.),
			y: rng.random_range(-1. ..= 1.),
		}
	}
	pub fn random_unit(rng: &mut ThreadRng) -> Self {
		Self::random_unit_cube(rng).normed()
	}
	pub fn norm2(self) -> float { self.dot(self) }
	pub fn norm(self) -> float { self.norm2().sqrt() }
	pub fn normed(self) -> Self { self / self.norm() }
	pub fn normalize(&mut self) { *self = self.normed() }
	pub fn normed_to(self, len: float) -> Self { self.normed() * len }
	pub fn normalize_to(&mut self, len: float) { *self = self.normed_to(len) }
	pub fn dist2_to(self, other: Self) -> float { (self - other).norm2() }
	pub fn dist_to(self, other: Self) -> float { self.dist2_to(other).sqrt() }
	pub fn _0xy(self) -> Vec3f { self.txy(0.) }
	pub fn _0yx(self) -> Vec3f { self.tyx(0.) }
	pub fn x0y(self) -> Vec3f { self.xty(0.) }
	pub fn y0x(self) -> Vec3f { self.ytx(0.) }
	pub fn xy0(self) -> Vec3f { self.xyt(0.) }
	pub fn yx0(self) -> Vec3f { self.yxt(0.) }
	pub fn rotate(self, _angle: float) -> Self {
		todo!()
	}
}



impl<T> Add<Self> for Vec2d<T> where T: Add<T,Output=T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y)
	}
}
impl<T> Add<T> for Vec2d<T> where T: Add<T,Output=T> + Copy {
	type Output = Self;
	fn add(self, rhs: T) -> Self::Output {
		Self::new(self.x + rhs, self.y + rhs)
	}
}

impl<T> AddAssign<Self> for Vec2d<T> where T: AddAssign<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
impl<T> AddAssign<T> for Vec2d<T> where T: AddAssign<T> + Copy {
	fn add_assign(&mut self, rhs: T) {
		self.x += rhs;
		self.y += rhs;
	}
}

impl<T> Sub<Self> for Vec2d<T> where T: Sub<T,Output=T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self::new(self.x - rhs.x, self.y - rhs.y)
	}
}
impl<T> Sub<T> for Vec2d<T> where T: Sub<T,Output=T> + Copy {
	type Output = Self;
	fn sub(self, rhs: T) -> Self::Output {
		Self::new(self.x - rhs, self.y - rhs)
	}
}

impl<T> SubAssign<Self> for Vec2d<T> where T: SubAssign<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}
impl<T> SubAssign<T> for Vec2d<T> where T: SubAssign<T> + Copy {
	fn sub_assign(&mut self, rhs: T) {
		self.x -= rhs;
		self.y -= rhs;
	}
}

impl<T> Mul<Self> for Vec2d<T> where T: Add<T,Output=T> + Mul<T,Output=T> + Copy {
	type Output = T;
	fn mul(self, rhs: Self) -> Self::Output {
		self.dot(rhs)
	}
}
// impl<T> Mul<Self> for Vec2d<T> where T: Sub<T,Output=T> + Mul<T,Output=T> + Copy {
// 	type Output = Self;
// 	fn mul(self, rhs: Self) -> Self::Output {
// 		Self::new(self.x * rhs.x, self.y * rhs.y)
// 	}
// }
impl<T> Mul<T> for Vec2d<T> where T: Mul<T,Output=T> + Copy {
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self::new(self.x * rhs, self.y * rhs)
	}
}
// impl<T> Mul<Vec2d<T>> for T where T: Mul<T,Output=T> {
// 	type Output = Vec2d<T>;
// 	fn mul(self, rhs: Vec2d<T>) -> Self::Output {
// 		todo!()
// 	}
// }

// impl<T> MulAssign<Self> for Vec2d<T> where: MulAssign<T> {
// 	fn mul_assign(&mut self, rhs: Self) {
// 		self.x *= rhs.x;
// 		self.y *= rhs.y;
// 	}
// }
impl<T> MulAssign<T> for Vec2d<T> where T: MulAssign<T> + Copy {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

// impl<T> Div<Self> for Vec2d<T> where T: Div<T,Output=T> {
// 	type Output = Self;
// 	fn div(self, rhs: Self) -> Self::Output {
// 		Self::new(self.x / rhs.x, self.y / rhs.y)
// 	}
// }
impl<T> Div<T> for Vec2d<T> where T: Div<T,Output=T> + Copy {
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		Self::new(self.x / rhs, self.y / rhs)
	}
}





impl<T> From<(T, T)> for Vec2d<T> {
	fn from((x, y): (T, T)) -> Self {
		Self { x, y }
	}
}
impl<T> From<Vec2d<T>> for (T, T) {
	fn from(v: Vec2d<T>) -> Self {
		(v.x, v.y)
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


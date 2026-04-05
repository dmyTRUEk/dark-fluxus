//! math vector 2d

use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use rand::{RngExt, rngs::ThreadRng};
use sdl3::render::FPoint;

use crate::{extensions::Into_, float_type::float, vec3d::Vec3f};



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
	pub fn from(x: impl Into_<T>, y: impl Into_<T>) -> Self {
		Self { x: x.into_(), y: y.into_() }
	}
}
impl<T> Vec2d<T> where T: Copy + Add<T,Output=T> + Mul<T,Output=T> {
	pub fn dot(self, other: Self) -> T {
		self.x*other.x + self.y*other.y
	}
}



impl Vec2f {
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
	pub fn normlize(&mut self) { *self = self.normed() }
	pub fn normed_to(self, len: float) -> Self { self.normed() * len }
	pub const fn txy(self, t: float) -> Vec3f { Vec3f { x: t, y: self.x, z: self.y } }
	pub const fn tyx(self, t: float) -> Vec3f { Vec3f { x: t, y: self.y, z: self.x } }
	pub const fn xty(self, t: float) -> Vec3f { Vec3f { x: self.x, y: t, z: self.y } }
	pub const fn ytx(self, t: float) -> Vec3f { Vec3f { x: self.y, y: t, z: self.x } }
	pub const fn xyt(self, t: float) -> Vec3f { Vec3f { x: self.x, y: self.y, z: t } }
	pub const fn yxt(self, t: float) -> Vec3f { Vec3f { x: self.y, y: self.x, z: t } }
	pub const fn _0xy(self) -> Vec3f { self.txy(0.) }
	pub const fn _0yx(self) -> Vec3f { self.tyx(0.) }
	pub const fn x0y(self) -> Vec3f { self.xty(0.) }
	pub const fn y0x(self) -> Vec3f { self.ytx(0.) }
	pub const fn xy0(self) -> Vec3f { self.xyt(0.) }
	pub const fn yx0(self) -> Vec3f { self.yxt(0.) }
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


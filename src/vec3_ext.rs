//! Vec3 extensions

use glam::Vec3;
use rand::{RngExt, rngs::ThreadRng};

use crate::extensions::{BoolSelect, Into_};


pub trait ExtVec3 {
	fn from_x(x: impl Into_<f32>) -> Self;
	fn from_y(y: impl Into_<f32>) -> Self;
	fn from_z(z: impl Into_<f32>) -> Self;
	fn from_yz(y: impl Into_<f32>, z: impl Into_<f32>) -> Self;
	fn from_xz(x: impl Into_<f32>, z: impl Into_<f32>) -> Self;
	fn from_xy(x: impl Into_<f32>, y: impl Into_<f32>) -> Self;
	fn random_unit_cube(rng: &mut ThreadRng) -> Self;
	fn random_unit(rng: &mut ThreadRng) -> Self;
	fn flip_x_if(self, flip: bool) -> Self;
	fn flip_y_if(self, flip: bool) -> Self;
	fn flip_z_if(self, flip: bool) -> Self;
}
impl ExtVec3 for Vec3 {
	fn from_x(x: impl Into_<f32>) -> Self { Self::ZERO.with_x(x.into_()) }
	fn from_y(y: impl Into_<f32>) -> Self { Self::ZERO.with_y(y.into_()) }
	fn from_z(z: impl Into_<f32>) -> Self { Self::ZERO.with_z(z.into_()) }
	fn from_yz(y: impl Into_<f32>, z: impl Into_<f32>) -> Self { Self::new(0., y.into_(), z.into_()) }
	fn from_xz(x: impl Into_<f32>, z: impl Into_<f32>) -> Self { Self::new(x.into_(), 0., z.into_()) }
	fn from_xy(x: impl Into_<f32>, y: impl Into_<f32>) -> Self { Self::new(x.into_(), y.into_(), 0.) }
	fn random_unit_cube(rng: &mut ThreadRng) -> Self {
		Self {
			x: rng.random_range(-1. ..= 1.),
			y: rng.random_range(-1. ..= 1.),
			z: rng.random_range(-1. ..= 1.),
		}
	}
	fn random_unit(rng: &mut ThreadRng) -> Self {
		Self::random_unit_cube(rng).normalize()
	}
	fn flip_x_if(self, flip: bool) -> Self { self.with_x(flip.select(-self.x, self.x)) }
	fn flip_y_if(self, flip: bool) -> Self { self.with_y(flip.select(-self.y, self.y)) }
	fn flip_z_if(self, flip: bool) -> Self { self.with_z(flip.select(-self.z, self.z)) }
}


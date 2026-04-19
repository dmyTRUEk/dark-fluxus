//! Vec3 extensions

use glam::Vec3;
use rand::{RngExt, rngs::ThreadRng};

use crate::{extensions::BoolSelect, float_type::float};


pub trait ExtVec3 {
	fn from_x(x: float) -> Self;
	fn from_y(y: float) -> Self;
	fn from_z(z: float) -> Self;
	fn from_yz(y: float, z: float) -> Self;
	fn from_xz(x: float, z: float) -> Self;
	fn from_xy(x: float, y: float) -> Self;
	fn random_unit_cube(rng: &mut ThreadRng) -> Self;
	fn random_unit(rng: &mut ThreadRng) -> Self;
	fn flip_x_if(self, flip: bool) -> Self;
	fn flip_y_if(self, flip: bool) -> Self;
	fn flip_z_if(self, flip: bool) -> Self;
}
impl ExtVec3 for Vec3 {
	fn from_x(x: float) -> Self { Self::ZERO.with_x(x) }
	fn from_y(y: float) -> Self { Self::ZERO.with_y(y) }
	fn from_z(z: float) -> Self { Self::ZERO.with_z(z) }
	fn from_yz(y: float, z: float) -> Self { Self::new(0., y, z) }
	fn from_xz(x: float, z: float) -> Self { Self::new(x, 0., z) }
	fn from_xy(x: float, y: float) -> Self { Self::new(x, y, 0.) }
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


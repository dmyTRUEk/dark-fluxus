//! lorenz attractor

use crate::{float, vec3d::Vec3f};



#[derive(Debug)]
pub struct LorenzAttractor {
	pub sigma: float,
	pub rho: float,
	pub beta: float,
	pub x: float,
	pub y: float,
	pub z: float,
}
impl LorenzAttractor {
	pub fn new() -> Self {
		Self {
			sigma: 10., rho: 28., beta: 8./3.,
			x: 0., y: 1., z: 1.,
		}
	}

	pub fn offset_params(mut self, s: float, r: float, b: float) -> Self {
		self.sigma += s;
		self.rho += r;
		self.beta += b;
		self
	}
	pub fn set_xyz(mut self, x: float, y: float, z: float) -> Self {
		self.x = x;
		self.y = y;
		self.z = z;
		self
	}
	pub fn set_xyz_as_vec3d(mut self, v: Vec3f) -> Self {
		self.x = v.x;
		self.y = v.y;
		self.z = v.z;
		self
	}
	pub fn offset_xyz(mut self, dx: float, dy: float, dz: float) -> Self {
		self.x += dx;
		self.y += dy;
		self.z += dz;
		self
	}

	pub fn step(&mut self, step_size: float) {
		let LorenzAttractor { sigma, rho, beta, x, y, z } = *self;
		let dx = sigma * (y - x);
		let dy = x * (rho - z) - y;
		let dz = x * y - beta * z;
		self.x += dx * step_size;
		self.y += dy * step_size;
		self.z += dz * step_size;
	}

	pub fn get_xyz_as_tuple(&self) -> (float, float, float) {
		(self.x, self.y, self.z)
	}
	pub fn get_xyz_as_vec3d(&self) -> Vec3f {
		Vec3f::from(self.x, self.y, self.z)
	}
	pub fn get_linear_combination(&self, cx: float, cy: float, cz: float) -> float {
		cx * self.x + cy * self.y + cz * self.z
	}
}


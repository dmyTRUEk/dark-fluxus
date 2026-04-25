//! lorenz attractor

use glam::Vec3;



#[derive(Debug, Clone)]
pub struct LorenzAttractor {
	pub sigma: f32,
	pub rho: f32,
	pub beta: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
impl LorenzAttractor {
	pub fn new() -> Self {
		Self {
			sigma: 10., rho: 28., beta: 8./3.,
			x: 0., y: 1., z: 1.,
		}
	}

	pub fn offset_params(mut self, s: f32, r: f32, b: f32) -> Self {
		self.sigma += s;
		self.rho   += r;
		self.beta  += b;
		self
	}
	pub fn offset_params_(self, delta_srb: impl Into<Vec3>) -> Self {
		let delta_srb: Vec3 = delta_srb.into();
		self.offset_params(delta_srb.x, delta_srb.y, delta_srb.z)
	}
	pub fn set_xyz(mut self, x: f32, y: f32, z: f32) -> Self {
		self.x = x;
		self.y = y;
		self.z = z;
		self
	}
	pub fn set_xyz_(self, v: impl Into<Vec3>) -> Self {
		let v: Vec3 = v.into();
		self.set_xyz(v.x, v.y, v.z)
	}
	pub fn offset_xyz(mut self, dx: f32, dy: f32, dz: f32) -> Self {
		self.x += dx;
		self.y += dy;
		self.z += dz;
		self
	}
	pub fn offset_xyz_(self, d: impl Into<Vec3>) -> Self {
		let d: Vec3 = d.into();
		self.offset_xyz(d.x, d.y, d.z)
	}

	pub fn step(&mut self, step_size: f32) {
		let LorenzAttractor { sigma, rho, beta, x, y, z } = *self;
		let dx = sigma * (y - x);
		let dy = x * (rho - z) - y;
		let dz = x * y - beta * z;
		self.x += dx * step_size;
		self.y += dy * step_size;
		self.z += dz * step_size;
	}

	pub fn get_xyz_as_tuple(&self) -> (f32, f32, f32) {
		(self.x, self.y, self.z)
	}
	pub fn get_xyz_as_vec3d(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}
	pub fn get_linear_combination(&self, cx: f32, cy: f32, cz: f32) -> f32 {
		cx * self.x + cy * self.y + cz * self.z
	}
	pub fn get_linear_combination_(&self, c: impl Into<Vec3>) -> f32 {
		let c = c.into();
		self.get_linear_combination(c.x, c.y, c.z)
	}
}


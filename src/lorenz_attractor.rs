//! lorenz attractor

use crate::float;



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
			x: 0., y: 0., z: 0.,
		}
	}

	pub fn step(&mut self, step_size: float) {
		let LorenzAttractor { sigma, rho, beta, x, y, z } = *self;
		let dx = sigma * (y - x);
		let dy = x * (rho - z) - y;
		let dz = x * y - beta * z;
		self.x += dx;
		self.y += dy;
		self.z += dz;
	}

	pub fn get_linear_combination(&self, cx: float, cy: float, cz: float) -> float {
		cx * self.x + cy * self.y + cz * self.z
	}
}


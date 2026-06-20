//! camera

use std::f32::consts::FRAC_PI_2;

use glam::{Mat3, Mat4, Quat, Vec3, vec3};
use rand::RngExt;

use crate::InputState; // main
use crate::math_aliases::{asigmoid, atan, sigmoid, tan};



#[derive(Debug)]
pub struct Camera {
	pub position: Vec3,
	orientation: Quat,
	pub aspect_ratio: f32,
	pub fov_x: f32,
	near: f32,
	far: f32,
	pub movement_type: MovementType,
	is_unlimited_fov: bool,
	is_shaky_fov: bool,
}

impl Camera {
	const GROUNDED_CAMERA_Y: f32 = 1.5;
	const DEFAULT_POSITION: Vec3 = vec3(0., Self::GROUNDED_CAMERA_Y, 0.);

	pub fn new(aspect_ratio: f32) -> Self {
		Self {
			position: vec3(0., Self::GROUNDED_CAMERA_Y, 0.),
			orientation: Quat::IDENTITY,
			aspect_ratio,
			fov_x: 100_f32.to_radians(),
			near: 0.1,
			far: 10_000.,
			movement_type: MovementType::Grounded,
			is_unlimited_fov: false,
			is_shaky_fov: false,
		}
	}

	pub fn toggle_unlimited_fov(&mut self) {
		self.is_unlimited_fov = !self.is_unlimited_fov;
	}
	pub fn toggle_shaky_fov(&mut self) {
		self.is_shaky_fov = !self.is_shaky_fov;
	}

	fn forward(&self) -> Vec3 {
		self.orientation * Vec3::NEG_Z
	}
	fn right(&self) -> Vec3 {
		self.orientation * Vec3::X
	}
	fn up(&self) -> Vec3 {
		self.orientation * Vec3::Y
	}
	/// returns (forward, up, right) vectors
	pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
		(self.forward(), self.up(), self.right())
	}

	pub fn view_matrix(&self) -> Mat4 {
		Mat4::look_to_rh(
			self.position,
			self.forward(),
			self.up(),
		)
	}

	pub fn proj_matrix(&self) -> Mat4 {
		Mat4::perspective_rh(
			self.fov_y_from_x(),
			self.aspect_ratio,
			self.near,
			self.far,
		)
	}

	// TODO(optim): "cache" the value (store in self)
	fn fov_y_from_x(&self) -> f32 {
		2.0 * atan(tan(self.fov_x * 0.5) / self.aspect_ratio)
	}

	pub fn update(&mut self, input: &mut InputState, dt: f32, rng: &mut impl RngExt) {
		// TODO: first update position or orientation?
		self.update_position(input, dt);
		self.update_orientation(input, dt);
		self.update_fov(input, dt, rng);
	}

	fn update_position(&mut self, input: &InputState, dt: f32) {
		let mut move_speed: f32 = 15.;
		if input.is_fast_move {
			move_speed *= 5.;
		}
		if input.forward {
			match self.movement_type {
				MovementType::Grounded |
				MovementType::FlyingMClike => {
					let forward_in_xz_plane = self.forward().with_y(0.).normalize();
					self.position += forward_in_xz_plane * move_speed * dt;
				}
				MovementType::FlyingGMlike |
				MovementType::FpvLike => {
					self.position += self.forward() * move_speed * dt;
				}
			}
		}
		if input.back {
			match self.movement_type {
				MovementType::Grounded |
				MovementType::FlyingMClike => {
					let forward_in_xz_plane = self.forward().with_y(0.).normalize();
					self.position -= forward_in_xz_plane * move_speed * dt;
				}
				MovementType::FlyingGMlike |
				MovementType::FpvLike => {
					self.position -= self.forward() * move_speed * dt;
				}
			}
		}
		if input.left {
			self.position -= self.right() * move_speed * dt;
		}
		if input.right {
			self.position += self.right() * move_speed * dt;
		}
		if input.up {
			match self.movement_type {
				MovementType::Grounded => {
					// TODO?
				}
				MovementType::FlyingMClike |
				MovementType::FlyingGMlike => {
					self.position += Vec3::Y * move_speed * dt;
				}
				MovementType::FpvLike => {
					self.position += self.up() * move_speed * dt;
				}
			}
		}
		if input.down {
			match self.movement_type {
				MovementType::Grounded => {
					// TODO?
				}
				MovementType::FlyingMClike |
				MovementType::FlyingGMlike => {
					self.position -= Vec3::Y * move_speed * dt;
				}
				MovementType::FpvLike => {
					self.position -= self.up() * move_speed * dt;
				}
			}
		}
	}

	fn update_orientation(&mut self, input: &mut InputState, dt: f32) {
		const SENSITIVITY: f32 = 0.007; // TODO!: adjust sensitivity
		const ROLL_SPEED: f32 = 2.;
		let sensitivity = SENSITIVITY * self.fov_x;

		let yaw = input.mouse_dx * sensitivity; // NOTE: "dt" is in mouse_dx

		const MAX_PITCH: f32 = FRAC_PI_2 * 0.99;
		let pitch_delta = input.mouse_dy * sensitivity; // NOTE: "dt" is in mouse_dy
		let forward = self.forward();
		let current_pitch = forward.y.asin();
		let new_pitch = (current_pitch - pitch_delta).clamp(-MAX_PITCH, MAX_PITCH);
		let clamped_pitch = current_pitch - new_pitch;

		let mut roll = 0.;
		match self.movement_type {
			MovementType::Grounded => {}
			MovementType::FlyingMClike => {}
			MovementType::FlyingGMlike => {}
			MovementType::FpvLike => {
				if input.roll_left {
					roll -= ROLL_SPEED * dt;
				}
				if input.roll_right {
					roll += ROLL_SPEED * dt;
				}
				if input.reset_roll {
					roll = 0.;
				}
			}
		}

		let yaw_q = Quat::from_rotation_y(-yaw); // world-space yaw // TODO: must be camera-space? bc camera rotation is incorrect with roll
		let pitch_q = Quat::from_axis_angle(self.right(), -clamped_pitch); // local-space pitch
		let roll_q = Quat::from_axis_angle(self.forward(), roll); // local-space roll
		self.orientation = yaw_q * pitch_q * roll_q * self.orientation;
		self.orientation = self.orientation.normalize();

		input.mouse_dx = 0.;
		input.mouse_dy = 0.;
	}

	fn update_fov(&mut self, input: &InputState, dt: f32, rng: &mut impl RngExt) {
		const FOV_MIN: f32 = 1e-1_f32.to_radians();
		const FOV_MAX: f32 = 170_f32.to_radians();
		const FOV_RANGE: f32 = FOV_MAX - FOV_MIN;
		const FOV_CHANGE_SPEED: f32 = 3.;

		if input.zoom_in {
			if self.is_unlimited_fov {
				self.fov_x -= dt;
			} else {
				self.fov_x = FOV_MIN + FOV_RANGE * sigmoid(asigmoid((self.fov_x-FOV_MIN)/FOV_RANGE) - FOV_CHANGE_SPEED*dt);
			}
		}
		if input.zoom_out {
			if self.is_unlimited_fov {
				self.fov_x += dt;
			} else {
				self.fov_x = FOV_MIN + FOV_RANGE * sigmoid(asigmoid((self.fov_x-FOV_MIN)/FOV_RANGE) + FOV_CHANGE_SPEED*dt);
			}
		}

		if self.is_shaky_fov {
			self.fov_x += rng.random_range(-0.1 ..= 0.1) * dt;
				// .clamp(1_f32.to_radians(), 170_f32.to_radians());
		}
		if !self.is_unlimited_fov {
			self.fov_x = self.fov_x.clamp(FOV_MIN*1.1, FOV_MAX/1.1);
		}
	}

	fn reset_roll(&mut self) {
		let forward = self.forward().normalize();
		// Pick a reference up that is not parallel to forward.
		let reference_up = if forward.y.abs() > 0.999 { Vec3::Z } else { Vec3::Y };
		// Build a level basis with the same forward direction.
		let right = forward.cross(reference_up).normalize();
		let up = right.cross(forward).normalize();
		// Camera local axes -> world axes:
		// local +X = right
		// local +Y = up
		// local -Z = forward
		let rot = Mat3::from_cols(right, up, -forward);
		self.orientation = Quat::from_mat3(&rot).normalize();
	}

	pub fn reset_position(&mut self) {
		self.position = Self::DEFAULT_POSITION;
	}

	pub fn next_movement_type(&mut self) {
		// clean up:
		match self.movement_type { // #bqooaj
			MovementType::Grounded => {}
			MovementType::FlyingMClike => {}
			MovementType::FlyingGMlike => {}
			MovementType::FpvLike => {
				self.position.y = Self::GROUNDED_CAMERA_Y;
				self.reset_roll();
			}
		}
		// next movement type:
		self.movement_type.next();
	}
}



#[derive(Debug)]
pub enum MovementType {
	// order is important, check #bqooaj
	Grounded,
	FlyingMClike,
	FlyingGMlike,
	FpvLike,
}
impl MovementType {
	fn next(&mut self) {
		use MovementType::*;
		*self = match self {
			// order is important, check #bqooaj
			Grounded => FlyingMClike,
			FlyingMClike => FlyingGMlike,
			FlyingGMlike => FpvLike,
			FpvLike => Grounded,
		};
	}
	pub fn to_str_uppercase(&self) -> &'static str {
		use MovementType::*;
		match self {
			Grounded => "GROUNDED",
			FlyingMClike => "FLYING MC LIKE",
			FlyingGMlike => "FLYING GM LIKE",
			FpvLike => "FPV LIKE",
		}
	}
}


//! weird game

#![allow(
	clippy::collapsible_if,
)]

#![deny(
	unused_variables,
)]

use std::array;

use minifb::{Key, Window, WindowOptions};
use rand::{Rng, RngExt, SeedableRng, distr::weighted::WeightedIndex, rng, rngs::StdRng};

mod colors;
mod extensions;
mod font_rendering;
mod frame_buffer;
mod lorenz_attractor;
mod math_aliases;
mod vec2d;
mod vec3d;
mod zqqx_lang;

use colors::*;
use extensions::*;
use font_rendering::*;
use frame_buffer::*;
use lorenz_attractor::*;
use math_aliases::*;
use vec2d::*;
use vec3d::*;
use zqqx_lang::*;





fn main() {
	let mut buffer = FrameBuffer::new(1600, 900);

	let mut window = Window::new(
		concat!("Weird Game v", env!("CARGO_PKG_VERSION")),
		buffer.w as usize, buffer.h as usize,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		}
	).expect("unable to create window");

	window.set_target_fps(60);
	window.update_with_my_buffer(&buffer);

	#[allow(unused_variables)]
	let mut rng = rng();

	#[allow(unused_variables)]
	let mut frame_n: u64 = 0;
	let mut is_paused: bool = false;
	let mut scale: u32 = 1;
	let mut lorenz_attractor = LorenzAttractor::new();//.offset_params(0.01, -0.01, 0.001);
	let mut last_points: Vec<Vec3d<float>> = vec![];
	let mut zqqx_lang = ZqqxLang::new();

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let mut is_redraw_needed: bool = false;

		// handle resizing
		if buffer.resize(window.get_size()) {
			//if verbose { println!("Resized to {w}x{h}") }
			is_redraw_needed = true;
		}

		// handle inputs
		if window.is_key_pressed_repeat(Key::Space) {
			is_paused = !is_paused;
		}
		if window.is_key_pressed_repeat(Key::I) {
			if scale > 1 {
				scale -= 1;
				is_redraw_needed = true;
			}
		}
		if window.is_key_pressed_repeat(Key::O) {
			scale += 1;
			is_redraw_needed = true;
		}

		if !is_paused {
			lorenz_attractor.step(1e-2);
			is_redraw_needed = true;
		}

		// render new frame
		if is_redraw_needed {
			frame_n += 1;
			buffer.clear();

			let (w, h) = buffer.get_wh();
			buffer.render_text(
				&format!("X: {}", lorenz_attractor.x),
				(0, 10),
				RED,
				3,
				(w, h),
			);
			buffer.render_text(
				&format!("Y: {}", lorenz_attractor.y),
				(0, 40),
				GREEN,
				3,
				(w, h),
			);
			buffer.render_text(
				&format!("Z: {}", lorenz_attractor.z),
				(0, 70),
				BLUE,
				3,
				(w, h),
			);

			last_points.push(lorenz_attractor.get_xyz_as_vec3d());

			const SCALE: float = 5.;

			// projection 1: drop X
			for v in last_points.iter() {
				let mut v = *v;
				v *= SCALE;
				let mut v = v.yz();
				v += Vec2d::new(w as float, h as float) / 4.;
				let Vec2d { x, y } = v;
				let (x, y) = (x as u32, y as u32);
				buffer[(x, y)] = WHITE.0;
			}

			// projection 2: drop Y
			for v in last_points.iter() {
				let mut v = *v;
				v *= SCALE;
				let mut v = v.xz();
				v += Vec2d::new((w*3) as float, h as float) / 4.;
				let Vec2d { x, y } = v;
				let (x, y) = (x as u32, y as u32);
				buffer[(x, y)] = WHITE.0;
			}

			// projection 3: drop Z
			for v in last_points.iter() {
				let mut v = *v;
				v *= SCALE;
				let mut v = v.xy();
				v += Vec2d::new(w as float, (h*3) as float) / 4.;
				let Vec2d { x, y } = v;
				let (x, y) = (x as u32, y as u32);
				buffer[(x, y)] = WHITE.0;
			}

			// projection 4
			for v in last_points.iter() {
				let mut v = *v;
				v *= SCALE;
				let mut v = v.project_2d(Vec3d::new(1.,1.,0.).normed(), Vec3d::new(0.,-1.,-1.).normed());
				v += Vec2d::new((w*3) as float, (h*3) as float) / 4.;
				let Vec2d { x, y } = v;
				let (x, y) = (x as u32, y as u32);
				buffer[(x, y)] = WHITE.0;
			}

			// zqqx lang
			for char_n in 0..5 {
				let scale: u8 = 5;
				let zqqx_char: [i8; 25] = array::from_fn(|i| {
					let (i, j) = (i % 5, i / 5);
					let cx = char_n as float;
					let cy = ((i+j*5) as float).sqrt();
					// let cz = ((j+i*5) as float).ln_1p();
					let cz = (frame_n as float).ln_1p().ln_1p().ln_1p();
					let coefs = Vec3d::new(cx, cy, cz).normed();
					let t = lorenz_attractor.get_linear_combination(coefs.x, coefs.y, coefs.z);
					let t = t.rem_euclid(1.);
					(t * 255. - 128.) as i8
				});
				let bitmap = zqqx_lang.add_or_quantize(ZqqxChar::new(zqqx_char));
				buffer.render_custom_char(
					bitmap.quantize(),
					((w as i32) - 200 + (((char_n*7)*scale) as i32), 10),
					WHITE,
					scale,
					(w, h),
				);
			}

			window.update_with_my_buffer(&buffer);
		} // end of render
		else {
			window.update();
		}
	} // end of main loop
}

const UNABLE_TO_UPDATE_WINDOW_BUFFER: &str = "unable to update window buffer";













#[allow(non_camel_case_types)]
type float = f64;



trait ExtWindowIsKeyPressed {
	fn is_key_pressed_once(&self, key: Key) -> bool;
	fn is_key_pressed_repeat(&self, key: Key) -> bool;
}
impl ExtWindowIsKeyPressed for Window {
	fn is_key_pressed_once(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::No)
	}
	fn is_key_pressed_repeat(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::Yes)
	}
}



trait ExtWindowUpdateWithMyBuffer {
	fn update_with_my_buffer(&mut self, buffer: &FrameBuffer);
}
impl ExtWindowUpdateWithMyBuffer for Window {
	fn update_with_my_buffer(&mut self, buffer: &FrameBuffer) {
		self.update_with_buffer(
			&buffer.buf,
			buffer.w as usize,
			buffer.h as usize
		).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);
	}
}


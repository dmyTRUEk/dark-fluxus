//! weird game

#![allow(
	clippy::collapsible_if,
)]

#![deny(
	unused_variables,
)]

use minifb::{Key, Window, WindowOptions};
use rand::{distr::weighted::WeightedIndex, rng, rngs::StdRng, Rng, SeedableRng};

mod colors;
mod font_rendering;
mod frame_buffer;
mod lorenz_attractor;

use colors::*;
use font_rendering::*;
use frame_buffer::*;
use lorenz_attractor::*;





fn main() {
	let mut buffer = FrameBuffer::new(1600, 900);

	let mut window = Window::new(
		concat!("Stockmarket Simulator v", env!("CARGO_PKG_VERSION")),
		buffer.w as usize, buffer.h as usize,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		}
	).expect("unable to create window");

	window.set_target_fps(60);
	window.update_with_buffer(&buffer.buf, buffer.w as usize, buffer.h as usize).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);

	#[allow(unused_variables)]
	let mut frame_n: u64 = 0;
	let mut is_paused: bool = false;
	let mut scale: u32 = 1;
	let mut lorenz_attractor = LorenzAttractor::new();
	let mut last_points: Vec<(float, float, float)> = vec![];

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
			lorenz_attractor.step(0.001);
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
				4,
				(w, h),
			);
			buffer.render_text(
				&format!("Y: {}", lorenz_attractor.y),
				(0, 50),
				GREEN,
				4,
				(w, h),
			);
			buffer.render_text(
				&format!("Z: {}", lorenz_attractor.z),
				(0, 90),
				BLUE,
				4,
				(w, h),
			);

			last_points.push(lorenz_attractor.get_xyz());

			// for p in last_points {
			// 	let x = p.0 as i32;
			// 	let y = p.1 as i32;
			// 	if x < 0 || y < 0 { continue }
			// 	let (x, y) = (x as )
			// 	buffer[]
			// }

			window.update_with_buffer(&buffer.buf, w as usize, h as usize).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);
		} // end of render
		else {
			window.update();
		}
	} // end of main loop
}

const UNABLE_TO_UPDATE_WINDOW_BUFFER: &str = "unable to update window buffer";













#[allow(non_camel_case_types)]
type float = f64;



trait WindowExtIsKeyPressed {
	fn is_key_pressed_once(&self, key: Key) -> bool;
	fn is_key_pressed_repeat(&self, key: Key) -> bool;
}
impl WindowExtIsKeyPressed for Window {
	fn is_key_pressed_once(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::No)
	}
	fn is_key_pressed_repeat(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::Yes)
	}
}


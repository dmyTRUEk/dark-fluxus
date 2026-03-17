//! weird game

#![allow(
	clippy::collapsible_if,
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
	let (mut w, mut h) = (1600, 900);
	let mut buffer = FrameBuffer::new(w, h);

	let mut window = Window::new(
		concat!("Stockmarket Simulator v", env!("CARGO_PKG_VERSION")),
		w as usize, h as usize,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		}
	).expect("unable to create window");

	window.set_target_fps(60);
	window.update_with_buffer(&buffer.buf, w as usize, h as usize).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);

	let mut frame_n: u64 = 0;
	let mut is_paused: bool = false;
	let mut scale: u32 = 1;

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let mut is_redraw_needed: bool = true;

		// handle resizing
		if let (w_new, h_new) = window.get_size() && (w_new as u32, h_new as u32) != (w, h) {
			(w, h) = (w_new as u32, h_new as u32);
			buffer.buf.resize(w_new * h_new, 0);
			//if verbose { println!("Resized to {w}x{h}") }
			is_redraw_needed = true;
		}

		// handle inputs
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

		// render new frame
		if is_redraw_needed {
			frame_n += 1;

			buffer.clear();

			buffer.render_text(
				&format!("{frame_n}"),
				((w/2) as i32, (h/2) as i32),
				WHITE,
				5,
				(w, h),
			);

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


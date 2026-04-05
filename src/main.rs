//! dark fluxus

#![allow(
	clippy::collapsible_if,
	clippy::just_underscores_and_digits,
	clippy::let_and_return,
	clippy::useless_format,
)]

#![deny(
	irrefutable_let_patterns,
	unreachable_patterns,
	unused_must_use,
	unused_results,
	unused_variables,
)]

#![feature(
	vec_from_fn,
)]

use std::{thread::sleep, time::{Duration, SystemTime}};

// use encoding_rs::UTF_8;
// use llama_cpp_2::{context::params::LlamaContextParams, llama_backend::LlamaBackend, llama_batch::LlamaBatch, model::{AddBos, LlamaModel, params::LlamaModelParams}, sampling::LlamaSampler};
use rand::{RngExt, rng, rngs::ThreadRng};
use sdl3::{event::Event, keyboard::{KeyboardState, Keycode, Scancode}, pixels::Color, render::{FPoint, FRect}};

mod consts;
mod extensions;
mod float_type;
mod font_rendering;
mod lorenz_attractor;
mod math_aliases;
mod typesafe_rng;
mod utils_io;
mod vec2d;
mod vec2D;
mod vec3d;
mod zqqx_lang;

use consts::*;
// use extensions::*;
use float_type::*;
use font_rendering::*;
use lorenz_attractor::*;
use math_aliases::*;
use typesafe_rng::*;
// use utils_io::*;
use vec2D::*;
use vec2d::*;
use vec3d::*;
// use zqqx_lang::*;





fn main() {
	#[allow(unused_variables)]
	let mut rng = rng();

	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let mut window = video_subsystem
		.window(&format!("Dark Fluxus v{}", env!("CARGO_PKG_VERSION")), 1600, 900) // alt: tenebrous
		.position_centered()
		.resizable()
		.fullscreen()
		// .input_grabbed() // ?
		// .opengl()
		// .vulkan()
		.build()
		.unwrap();

	let _ = window.set_mouse_grab(true);
	sdl_context.mouse().show_cursor(false);
	sdl_context.mouse().set_relative_mouse_mode(&window, true);


	let pause_menu_items = { use PauseMenuItem::*; vec![
		Quit,
	]};
	let mut is_paused = false;
	let mut pause_menu_item_index: u32 = 0;


	const CHUNKS_N: u32 = 5;
	let render_distance: u32 = 2;
	let mut chunks = Vec2D::<Chunk>::from_fn(CHUNKS_N, CHUNKS_N, |_x, _z| {
		Chunk::new_random(&mut rng)
	});
	// println!("chunks.len = {}", chunks.iter().count());

	const GROUNDED_CAMERA_Y: float = 1.5;
	let mut camera = Camera {
		pos: vec3![0., GROUNDED_CAMERA_Y, 0.],
		forward: vec3![0., 0., 1.],
		up: vec3![0., 1., 0.],
		fov: 90. * DEG_TO_RAD,
	};
	let mut current_chunk_x = 0;
	let mut current_chunk_z = 0;

	let mut movement_type = MovementType::Grounded;

	#[allow(unused_variables)]
	let mut tick_n: u64 = 0;
	let mut frame_n: u64 = 0;
	let mut is_extra_info_shown = true;

	// let mut zqqx_lang = ZqqxLang::new();


	let mut canvas = window.into_canvas();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	let _ = canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();

	'main_loop: loop {
		let tick_frame_begin_timestamp = SystemTime::now();

		tick_n += 1;

		let mut is_redraw_needed: bool = frame_n == 0; // TODO: or `renderable_objects.is_time_dependent` exists

		const DELTA_TIME: float = 0.01; // TODO

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => { break 'main_loop }
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					is_paused = !is_paused;
					is_redraw_needed = true;
				}
				Event::MouseMotion { xrel, yrel, .. } if !is_paused => {
					const ROTATION_SPEED: float = 0.6;
					// left/right
					camera.forward += camera.basis().0 * xrel * camera.fov * DELTA_TIME * ROTATION_SPEED;
					camera.forward.normlize();
					// up/down
					camera.forward -= camera.basis().1 * yrel * camera.fov * DELTA_TIME * ROTATION_SPEED;
					camera.forward.normlize();
					is_redraw_needed = true;
				}
				// TODO: use MouseWheel for FOV?
				Event::KeyDown { keycode: Some(Keycode::F5), .. } if !is_paused => {
					movement_type.next();
					match movement_type { // #bqooaj
						MovementType::Grounded => {
							camera.pos.y = GROUNDED_CAMERA_Y;
							camera.reset_roll();
							is_redraw_needed = true;
						}
						MovementType::FlyingMClike => {}
						MovementType::FlyingGMlike => {}
						MovementType::FpvLike => {}
					}
				}
				Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
					is_extra_info_shown = !is_extra_info_shown;
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Up), .. } if is_paused => {
					pause_menu_item_index = ((pause_menu_item_index as i32) - 1).rem_euclid(pause_menu_items.len() as i32) as u32;
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Down), .. } if is_paused => {
					pause_menu_item_index = (pause_menu_item_index + 1).rem_euclid(pause_menu_items.len() as u32);
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Return), .. } if is_paused => {
					use PauseMenuItem::*;
					match pause_menu_items[pause_menu_item_index as usize] {
						Quit => {
							break 'main_loop
						}
						Text(_) => {}
					}
				}
				_ => {}
			}
		}
		let keyboard = event_pump.keyboard_state();

		// handle inputs:
		let mut move_speed: float = 20.;
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::LShift, Scancode::RShift]) {
			move_speed *= 3.;
		}
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::Up, Scancode::W, Scancode::P]) {
			match movement_type {
				MovementType::Grounded |
				MovementType::FlyingMClike => {
					let forward_in_xz_plane = camera.forward.x0z().normed();
					camera.pos += forward_in_xz_plane * DELTA_TIME * move_speed;
				}
				MovementType::FlyingGMlike |
				MovementType::FpvLike => {
					camera.pos += camera.forward * DELTA_TIME * move_speed;
				}
			}
			is_redraw_needed = true;
		}
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::Down, Scancode::S, Scancode::Semicolon]) {
			match movement_type {
				MovementType::Grounded |
				MovementType::FlyingMClike => {
					let forward_in_xz_plane = camera.forward.x0z().normed();
					camera.pos -= forward_in_xz_plane * DELTA_TIME * move_speed;
				}
				MovementType::FlyingGMlike |
				MovementType::FpvLike => {
					camera.pos -= camera.forward * DELTA_TIME * move_speed;
				}
			}
			is_redraw_needed = true;
		}
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::Left, Scancode::A, Scancode::L]) {
			camera.pos -= camera.basis().0 * DELTA_TIME * move_speed;
			is_redraw_needed = true;
		}
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::Right, Scancode::D, Scancode::Apostrophe]) {
			camera.pos += camera.basis().0 * DELTA_TIME * move_speed;
			is_redraw_needed = true;
		}
		if !is_paused && keyboard.is_scancode_pressed(Scancode::Space) {
			match movement_type {
				MovementType::Grounded => {
					// TODO?
				}
				MovementType::FlyingMClike |
				MovementType::FlyingGMlike => {
					camera.pos += vec3y!(1) * DELTA_TIME * move_speed;
				}
				MovementType::FpvLike => {
					camera.pos += camera.basis().1 * DELTA_TIME * move_speed;
				}
			}
			is_redraw_needed = true;
		}
		if !is_paused && keyboard.is_scancodes_pressed_any(&[Scancode::LCtrl, Scancode::LAlt, Scancode::RCtrl, Scancode::RAlt]) {
			match movement_type {
				MovementType::Grounded => {
					// TODO?
				}
				MovementType::FlyingMClike |
				MovementType::FlyingGMlike => {
					camera.pos -= vec3y!(1) * DELTA_TIME * move_speed;
				}
				MovementType::FpvLike => {
					camera.pos -= camera.basis().1 * DELTA_TIME * move_speed;
				}
			}
			is_redraw_needed = true;
		}
		const ROLL_SPEED: float = 1.;
		if !is_paused && keyboard.is_scancode_pressed(Scancode::Q) {
			match movement_type {
				MovementType::Grounded => {}
				MovementType::FlyingMClike => {}
				MovementType::FlyingGMlike => {}
				MovementType::FpvLike => {
					camera.up -= camera.basis().0 * DELTA_TIME * ROLL_SPEED;
					camera.up.normlize();
					is_redraw_needed = true;
				}
			}
		}
		if !is_paused && keyboard.is_scancode_pressed(Scancode::E) {
			match movement_type {
				MovementType::Grounded => {}
				MovementType::FlyingMClike => {}
				MovementType::FlyingGMlike => {}
				MovementType::FpvLike => {
					camera.up += camera.basis().0 * DELTA_TIME * ROLL_SPEED;
					camera.up.normlize();
					is_redraw_needed = true;
				}
			}
		}
		if !is_paused && keyboard.is_scancode_pressed(Scancode::R) {
			// reset camera roll
			match movement_type {
				MovementType::Grounded => {}
				MovementType::FlyingMClike => {}
				MovementType::FlyingGMlike => {}
				MovementType::FpvLike => {
					camera.reset_roll();
					is_redraw_needed = true;
				}
			}
		}

		const MIN_FOV: float = 1e-1 * DEG_TO_RAD;
		const MAX_FOV: float = 170. * DEG_TO_RAD;
		const FOV_RANGE: float = MAX_FOV - MIN_FOV;
		const FOV_CHANGE_SPEED: float = 0.03;
		if keyboard.is_scancode_pressed(Scancode::I) {
			// camera.fov -= DELTA;
			camera.fov = MIN_FOV + FOV_RANGE * sigmoid(asigmoid((camera.fov-MIN_FOV)/FOV_RANGE) - FOV_CHANGE_SPEED);
			is_redraw_needed = true;
		}
		if keyboard.is_scancode_pressed(Scancode::O) {
			// camera.fov += DELTA;
			camera.fov = MIN_FOV + FOV_RANGE * sigmoid(asigmoid((camera.fov-MIN_FOV)/FOV_RANGE) + FOV_CHANGE_SPEED);
			is_redraw_needed = true;
		}

		// physics update:
		if !is_paused /* TODO: && exist what needs to be updated */ {
			for (_x, _z, chunk) in chunks.iter_mut() {
				for (_pos, ro) in chunk.renderable_objects.iter_mut() {
					ro.update(DELTA_TIME);
				}
			}
			is_redraw_needed = true;
		}
		{
			if camera.pos.x < -CHUNK_SIZE_HALF {
				camera.pos.x += CHUNK_SIZE;
				current_chunk_x -= 1;
			}
			else if camera.pos.x > CHUNK_SIZE_HALF {
				camera.pos.x -= CHUNK_SIZE;
				current_chunk_x += 1;
			}
			if camera.pos.z < -CHUNK_SIZE_HALF {
				camera.pos.z += CHUNK_SIZE;
				current_chunk_z -= 1;
			}
			else if camera.pos.z > CHUNK_SIZE_HALF {
				camera.pos.z -= CHUNK_SIZE;
				current_chunk_z += 1;
			}
		}

		// render new frame:
		if is_redraw_needed {
			frame_n += 1;

			// canvas.set_draw_color(Color::RGB(((frame_n) % 255) as u8, (((frame_n+64)/2) % 255) as u8, (255 - (frame_n/3) % 255) as u8));
			canvas.set_draw_color(Color::BLACK);
			canvas.clear();

			// dbg!(&camera);

			let (w, h) = canvas.window().size();
			let (wi, _hi) = (w as i32, h as i32);
			let (wf, hf) = (w as float, h as float);
			let (wfh, hfh) = (wf / 2., hf / 2.);
			// let wh_ratio = wf / hf;
			// let hw_ratio = hf / wf;

			for (dx, dz, _x, _z, chunk) in chunks.iter_around_wrapping(current_chunk_x, current_chunk_z, render_distance) {
				canvas.set_draw_color(chunk.color);
				const STEP: float = 1.;
				let mut x = -CHUNK_SIZE_HALF * (1. - 1e-2);
				while x < CHUNK_SIZE_HALF {
					let mut z = -CHUNK_SIZE_HALF * (1. - 1e-2);
					while z < CHUNK_SIZE_HALF {
						let lines = [
							(Vec3f::new((dx as float)*CHUNK_SIZE+x-STEP/3., 0., (dz as float)*CHUNK_SIZE+z-STEP/3.),
							 Vec3f::new((dx as float)*CHUNK_SIZE+x+STEP/3., 0., (dz as float)*CHUNK_SIZE+z+STEP/3.)),
							(Vec3f::new((dx as float)*CHUNK_SIZE+x-STEP/3., 0., (dz as float)*CHUNK_SIZE+z+STEP/3.),
							 Vec3f::new((dx as float)*CHUNK_SIZE+x+STEP/3., 0., (dz as float)*CHUNK_SIZE+z-STEP/3.)),
						];
						for line in lines.iter() {
							if let Some((a,b)) = camera.project_line(line, wf, hf) {
								canvas.draw_line(a,b).unwrap();
							}
						}
						z += STEP;
					}
					x += STEP;
				}
			}
			for (dx, dz, _x, _z, chunk) in chunks.iter_around_wrapping(current_chunk_x, current_chunk_z, render_distance) {
				canvas.set_draw_color(chunk.color);
				for (pos, ro) in chunk.renderable_objects.iter() {
					use SdlRenderableShape::*;
					let shift: Vec3f = *pos + Vec3f::from_xz((dx as float)*CHUNK_SIZE, (dz as float)*CHUNK_SIZE);
					match ro.get_renderable_shape() {
						Points(points) => {
							let projected_points: Vec<FPoint> = points.iter()
								.map(|&p| p + shift)
								.flat_map(|p| {
									camera.project_point(p, wf, hf).map(|p| p.into())
								}).collect::<Vec<_>>();
							canvas.draw_points(projected_points.as_slice()).unwrap();
						}
						Lines(lines) => {
							for line in lines.iter() {
								let line = (line.0 + shift, line.1 + shift);
								if let Some((a,b)) = camera.project_line(&line, wf, hf) {
									canvas.draw_line(a,b).unwrap();
								}
							}
						}
						Chain(chain) => {
							let projected_chain: Vec<FPoint> = chain.iter()
								.map(|&p| p + shift)
								.flat_map(|p| {
									camera.project_point(p, wf, hf).map(|p| p.into())
								}).collect::<Vec<_>>();
							canvas.draw_lines(projected_chain.as_slice()).unwrap();
						}
					}
				}
			}

			if is_extra_info_shown {
				let text_size = 3;
				canvas.set_draw_color(Color::GRAY);
				canvas.render_text(&format!("XYZ: {:.3}, {:.3}, {:.3}", camera.pos.x, camera.pos.y, camera.pos.z), (5,5), text_size);
				canvas.render_text(&format!("FOV: {:.3}", camera.fov * RAD_TO_DEG), (5,5+35), text_size);
				canvas.render_text(&format!("MOVE TYPE: {}", movement_type.to_str_uppercase()), (5,5+35*2), text_size);

				// // zqqx lang
				// for char_n in 0..5 {
				// 	let scale: u8 = 5;
				// 	let zqqx_char: [i8; 25] = array::from_fn(|i| {
				// 		let (i, j) = (i % 5, i / 5);
				// 		let cx = char_n as float;
				// 		let cy = ((i+j*5) as float).sqrt();
				// 		// let cz = ((j+i*5) as float).ln_1p();
				// 		let cz = (frame_n as float).ln_1p().ln_1p().ln_1p();
				// 		let coefs = vec3![cx, cy, cz].normed();
				// 		let t = lorenz_attractor.get_linear_combination(coefs.x, coefs.y, coefs.z);
				// 		let t = t.rem_euclid(1.);
				// 		(t * 255. - 128.) as i8
				// 	});
				// 	let bitmap = zqqx_lang.add_or_quantize(ZqqxChar::new(zqqx_char));
				// 	buffer.render_custom_char(
				// 		bitmap.quantize(),
				// 		((buffer.w as i32) - 200 + (((char_n*7)*scale) as i32), 10),
				// 		WHITE,
				// 		scale,
				// 	);
				// }

				let frame_end_timestamp = SystemTime::now();
				let frametime = frame_end_timestamp.duration_since(tick_frame_begin_timestamp).unwrap();
				let fps = 1. / frametime.as_secs_f64();
				// if fps < 60. { panic!() }
				let fps_text = format!("\"FPS\": {fps:.1}");
				canvas.render_text(&fps_text, (wi - 5 - (fps_text.len() as i32) * (text_size as i32) * 6, 5), text_size);
			}

			if is_paused {
				const PADDING: float = 50.;
				const ITEM_Y: float = 80.;
				const ITEMS_N: u32 = 7;
				debug_assert_eq!(1, ITEMS_N % 2);
				const MENU_SIZE_X: float = 800.;
				const MENU_SIZE_Y: float = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as float);
				const ITEM_X: float = MENU_SIZE_X - 2.*PADDING;
				canvas.set_draw_color(Color::BLACK);
				canvas.fill_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
				canvas.set_draw_color(Color::WHITE);
				canvas.draw_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
				const ITEM_UNSELECTED_COLOR: Color = Color::GRAY;
				const ITEM_SELECTED_COLOR: Color = Color::WHITE;
				// const ITEM_TEXT_COLOR: Color = Color::GREEN;
				const ITEM_TEXT_SIZE: u8 = 5;
				const ITEM_INNER_PADDING: float = (ITEM_Y - (ITEM_TEXT_SIZE as float)*(FONT_H as float)) / 2.;
				canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
				let i_init: u32 = pause_menu_item_index.saturating_sub((ITEMS_N-1)/2);
				let mut i: u32 = i_init;
				while i - i_init < ITEMS_N && i < pause_menu_items.len() as u32 {
					let menu_item = &pause_menu_items[i as usize];
					if i == pause_menu_item_index {
						canvas.set_draw_color(ITEM_SELECTED_COLOR);
					}
					let item_cx = wfh;
					let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
					canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
					canvas.render_text(menu_item.to_str(), ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
					if i == pause_menu_item_index {
						canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
					}
					i += 1;
				}
			}

			let _ = canvas.present();
		}

		let tick_end_timestamp = SystemTime::now();
		let ticktime = tick_end_timestamp.duration_since(tick_frame_begin_timestamp).unwrap();
		let target_fps = 60;
		if ticktime < Duration::new(0, 1_000_000_000u32 / target_fps) {
			sleep(Duration::new(0, 1_000_000_000u32 / target_fps) - ticktime);
		}
	}
}





enum PauseMenuItem {
	Quit,
	Text(String), // just for test
}
impl PauseMenuItem {
	fn to_str(&self) -> &str {
		use PauseMenuItem::*;
		match self {
			Quit => "QUIT",
			Text(text) => text,
		}
	}
}




#[derive(Debug)]
enum MovementType {
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
	fn to_str_uppercase(&self) -> &'static str {
		use MovementType::*;
		match self {
			Grounded => "GROUNDED",
			FlyingMClike => "FLYING MC LIKE",
			FlyingGMlike => "FLYING GM LIKE",
			FpvLike => "FPV LIKE",
		}
	}
}





#[derive(Debug)]
enum SdlRenderableShape {
	Points(Vec<Vec3f>),
	Lines(Vec<(Vec3f, Vec3f)>),
	Chain(Vec<Vec3f>),
}

#[derive(Debug)]
enum RenderableObject {
	Cube { size: float },
	LorenzAttractor { size: float, la: LorenzAttractor, last_points: Vec<Vec3f>, max_len: u32 },
	// SpinningText?
	Monolith { sizes: Vec<float> },
	RotatingSimplex { points_rotplanes_rotvels: Vec<(Vec3f, Vec3f, float)> },
}
impl RenderableObject {
	fn is_need_update(&self) -> bool {
		use RenderableObject::*;
		match self {
			| LorenzAttractor { .. }
			| RotatingSimplex { .. }
			=> true,
			| Cube { .. }
			| Monolith { .. }
			=> false,
		}
	}
	fn is_time_dependent(&self) -> bool {
		use RenderableObject::*;
		match self {
			| LorenzAttractor { .. }
			| RotatingSimplex { .. }
			=> true,
			| Cube { .. }
			| Monolith { .. }
			=> false,
		}
	}
	fn update(&mut self, delta_time: float) {
		use RenderableObject::*;
		match self {
			LorenzAttractor { la, last_points, max_len, .. } => {
				last_points.push(la.get_xyz_as_vec3d());
				if last_points.len() as u32 > *max_len {
					let _ = last_points.remove(0);
				}
				la.step(1e-2);
			}
			RotatingSimplex { points_rotplanes_rotvels } => {
				for (point, rotation_plane, rotation_velocity) in points_rotplanes_rotvels.iter_mut() {
					// TODO: this causes them to gradually grow in size, so use only starting point and current phase, which is %2pi
					*point += point.cross(*rotation_plane) * *rotation_velocity * delta_time;
				}
			}
			| Cube { .. }
			| Monolith { .. }
			=> {}
		}
	}
	fn get_renderable_shape(&self) -> SdlRenderableShape {
		use RenderableObject::*;
		use SdlRenderableShape::*;
		match self {
			Cube { size } => {
				let s = size / 2.;
				Lines(vec![
					(vec3!(-s,-s,-s), vec3!(-s,-s, s)),
					(vec3!(-s,-s,-s), vec3!(-s, s,-s)),
					(vec3!(-s, s, s), vec3!(-s,-s, s)),
					(vec3!(-s, s, s), vec3!(-s, s,-s)),
					//
					(vec3!( s,-s,-s), vec3!( s,-s, s)),
					(vec3!( s,-s,-s), vec3!( s, s,-s)),
					(vec3!( s, s, s), vec3!( s,-s, s)),
					(vec3!( s, s, s), vec3!( s, s,-s)),
					//
					(vec3!(-s,-s,-s), vec3!( s,-s,-s)),
					(vec3!( s, s, s), vec3!(-s, s, s)),
					(vec3!(-s,-s, s), vec3!( s,-s, s)),
					(vec3!(-s, s,-s), vec3!( s, s,-s)),
				])
			}
			LorenzAttractor { size, last_points, .. } => {
				Chain(last_points.iter().map(|&p| p * *size).collect())
			}
			Monolith { sizes } => {
				Lines(sizes.iter().map(|size| {
					let s = size / 2.;
					vec![
						(vec3!(-s,-s,-s), vec3!(-s,-s, s)),
						(vec3!(-s,-s,-s), vec3!(-s, s,-s)),
						(vec3!(-s, s, s), vec3!(-s,-s, s)),
						(vec3!(-s, s, s), vec3!(-s, s,-s)),
						//
						(vec3!( s,-s,-s), vec3!( s,-s, s)),
						(vec3!( s,-s,-s), vec3!( s, s,-s)),
						(vec3!( s, s, s), vec3!( s,-s, s)),
						(vec3!( s, s, s), vec3!( s, s,-s)),
						//
						// (vec3!(-s,-s,-s), vec3!( s,-s,-s)),
						// (vec3!( s, s, s), vec3!(-s, s, s)),
						// (vec3!(-s,-s, s), vec3!( s,-s, s)),
						// (vec3!(-s, s,-s), vec3!( s, s,-s)),
					]
				}).flatten().collect())
			}
			RotatingSimplex { points_rotplanes_rotvels } => {
				let mut lines = vec![];
				for i in 0 .. points_rotplanes_rotvels.len() {
					for j in i+1 .. points_rotplanes_rotvels.len() {
						let a = points_rotplanes_rotvels[i].0;
						let b = points_rotplanes_rotvels[j].0;
						lines.push((a, b));
					}
				}
				Lines(lines)
			}
		}
	}
}





const CHUNK_SIZE: float = 10.;
const CHUNK_SIZE_HALF: float = CHUNK_SIZE / 2.;
struct Chunk {
	color: Color,
	renderable_objects: Vec<(Vec3f, RenderableObject)>,
}
impl Chunk {
	fn new_random(rng: &mut ThreadRng) -> Self {
		Chunk {
			// color: Color::RGB(255/(CHUNKS_N as u8)*(1 + x as u8), 255/(CHUNKS_N as u8)*(1 + z as u8), 0), // for dbg
			color: Color::RGB(rng.random(), rng.random(), rng.random()),
			renderable_objects: {
				use V5::*;
				match rng.random_variant_weighted([3., 1., 0.5, 0.1, 0.5]) {
					_1 => vec![],
					_2 => Vec::from_fn(
						rng.random_range(0 ..= 5),
						|_i| (
							Vec3f::new(
								rng.random_range(-CHUNK_SIZE_HALF ..= CHUNK_SIZE_HALF),
								rng.random_range(1. ..= 9.),
								rng.random_range(-CHUNK_SIZE_HALF ..= CHUNK_SIZE_HALF),
							),
							RenderableObject::Cube { size: rng.random_range(0.3 ..= 3.) }
						)
					),
					_3 => vec![(
						Vec3f::new(-0.5, rng.random_range(1. ..= 9.), -4.),
						RenderableObject::LorenzAttractor {
							size: rng.random_range(0.1 ..= 0.2),
							la: LorenzAttractor::new().offset_params_(
								Vec3f::random_unit_cube(rng) * 0.1,
							).set_xyz_(
								Vec3f::random_unit(rng) * rng.random_range(0.1 ..= 0.2),
							),
							last_points: vec![],
							max_len: 10_f32.powf(rng.random_range(2. ..= 4.)).round() as u32,
						}
					)],
					_4 => vec![(
						Vec3f::from_y(rng.random_range(1. ..= 3.)),
						RenderableObject::Monolith {
							sizes: Vec::from_fn(
								rng.random_range(5 ..= 20),
								|_i| rng.random_range(0.5 ..= 2.7_f32).powi(2)
							),
						}
					)],
					_5 => vec![(
						Vec3f::from_y(rng.random_range(1. ..= 5.)),
						RenderableObject::RotatingSimplex {
							points_rotplanes_rotvels: {
								macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
								let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
								(0..rng.random_range(4 ..= 10)).map(|_i| (
									Vec3f::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
									Vec3f::random_unit(rng),
									rng.random_range(0.5 ..= 1.4_f32).powi(2)
								)).collect()
							},
						}
					)]
				}
			}
		}
	}
}





#[derive(Debug)]
struct Camera {
	pos: Vec3f,
	forward: Vec3f,
	up: Vec3f,
	fov: float, // in radians
}
const NEAR: float = 0.1;
impl Camera {
	fn reset_roll(&mut self) {
		self.up = vec3y!(1.);
	}

	/// returns (right, up, forward) vectors
	fn basis(&self) -> (Vec3f, Vec3f, Vec3f) {
		let f = self.forward.normed();
		let r = f.cross(self.up).normed();
		let u = r.cross(f);
		(r, u, f)
	}

	fn project_line(
		&self,
		line: &(Vec3f, Vec3f),
		width: float,
		height: float,
		// near: float,
	) -> Option<(Vec2f, Vec2f)> {
		let (a, b) = self.clip_line_near(line.0, line.1, NEAR)?;
		let pa = self.project_point(a, width, height)?;
		let pb = self.project_point(b, width, height)?;
		clip_line_viewport(pa, pb, width, height)
	}

	fn clip_line_near(&self, a: Vec3f, b: Vec3f, near: float) -> Option<(Vec3f, Vec3f)> {
		let (_, _, forward) = self.basis();
		let da = (a - self.pos) * forward;
		let db = (b - self.pos) * forward;
		if da >= near && db >= near { return Some((a, b)); }
		if da < near && db < near { return None; }
		let t = (near - da) / (db - da);
		let intersect = Vec3f::new(
			a.x + (b.x - a.x) * t,
			a.y + (b.y - a.y) * t,
			a.z + (b.z - a.z) * t,
		);
		if da < near {
			Some((intersect, b))
		} else {
			Some((a, intersect))
		}
	}

	fn project_point(&self, p: Vec3f, width: float, height: float) -> Option<Vec2f> {
		let (right, up, forward) = self.basis();

		// world -> camera space
		let rel = p - self.pos;

		let x = rel * right;
		let y = rel * up;
		let z = rel * forward;

		// behind camera
		if z <= 0. { return None; }

		let aspect = width / height;
		let f = 1. / (self.fov * 0.5).tan();

		// perspective projection (NDC)
		let nx = (x / z) * f / aspect;
		let ny = (y / z) * f;

		// to screen pixels
		let sx = (nx + 1.) * 0.5 * width;
		let sy = (1. - (ny + 1.) * 0.5) * height;

		Some(Vec2f { x: sx, y: sy })
	}
}

// TODO: rename
const _INSIDE: u8 = 0;
const _LEFT: u8 = 1;
const _RIGHT: u8 = 2;
const _BOTTOM: u8 = 4;
const _TOP: u8 = 8;

fn clip_line_viewport(mut a: Vec2f, mut b: Vec2f, w: float, h: float) -> Option<(Vec2f, Vec2f)> {
	let mut out_a = compute_outcode(a, w, h);
	let mut out_b = compute_outcode(b, w, h);
	loop {
		if (out_a | out_b) == 0 { return Some((a, b)); }
		if (out_a & out_b) != 0 { return None; }
		let out = if out_a != 0 { out_a } else { out_b };
		let mut x = 0.;
		let mut y = 0.;
		if (out & _TOP) != 0 {
			x = a.x + (b.x - a.x) * (0. - a.y) / (b.y - a.y);
			y = 0.;
		} else if (out & _BOTTOM) != 0 {
			x = a.x + (b.x - a.x) * (h - a.y) / (b.y - a.y);
			y = h;
		} else if (out & _RIGHT) != 0 {
			y = a.y + (b.y - a.y) * (w - a.x) / (b.x - a.x);
			x = w;
		} else if (out & _LEFT) != 0 {
			y = a.y + (b.y - a.y) * (0. - a.x) / (b.x - a.x);
			x = 0.;
		}
		if out == out_a {
			a = Vec2f::new(x, y);
			out_a = compute_outcode(a, w, h);
		} else {
			b = Vec2f::new(x, y);
			out_b = compute_outcode(b, w, h);
		}
	}
}

fn compute_outcode(p: Vec2f, w: float, h: float) -> u8 {
	let mut code = _INSIDE;
	if p.x < 0. { code |= _LEFT; } else if p.x > w { code |= _RIGHT; }
	if p.y < 0. { code |= _TOP; } else if p.y > h { code |= _BOTTOM; }
	code
}





trait SdlFRectFromCenterSize {
	fn from_center_size(cx: float, cy: float, sx: float, sy: float) -> Self;
	fn from_center_size_(c: impl Into<Vec2f>, s: impl Into<Vec2f>) -> Self;
}
impl SdlFRectFromCenterSize for FRect {
	fn from_center_size(cx: float, cy: float, sx: float, sy: float) -> Self {
		Self {
			x: cx - sx/2.,
			y: cy - sy/2.,
			w: sx,
			h: sy,
		}
	}
	fn from_center_size_(c: impl Into<Vec2f>, s: impl Into<Vec2f>) -> Self {
		let c = c.into();
		let s = s.into();
		Self::from_center_size(c.x, c.y, s.x, s.y)
	}
}



trait SdlKeyboardExtIsScancodesPressed {
	fn is_scancodes_pressed_any(&self, scancodes: &[Scancode]) -> bool;
	fn is_scancodes_pressed_all(&self, scancodes: &[Scancode]) -> bool;
}
impl SdlKeyboardExtIsScancodesPressed for KeyboardState<'_> {
	fn is_scancodes_pressed_any(&self, scancodes: &[Scancode]) -> bool {
		for scancode in scancodes {
			if self.is_scancode_pressed(*scancode) {
				return true
			}
		}
		false
	}
	fn is_scancodes_pressed_all(&self, scancodes: &[Scancode]) -> bool {
		for scancode in scancodes {
			if !self.is_scancode_pressed(*scancode) {
				return false
			}
		}
		true
	}
}


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

use std::{f32::consts::{GOLDEN_RATIO, PI, TAU}, thread::sleep, time::{Duration, SystemTime}};

use rand::{RngExt, rng, rngs::ThreadRng};
use sdl3::{event::Event, keyboard::{KeyboardState, Keycode, Scancode}, pixels::Color, render::{FPoint, FRect}};

mod consts;
mod extensions;
mod float_type;
mod font_rendering;
mod lorenz_attractor;
mod math;
mod math_aliases;
mod typesafe_rng;
mod utils_io;
mod vec2d;
mod vec2D;
mod vec3d;
mod zqqx_lang;

use consts::*;
use extensions::*;
use float_type::*;
use font_rendering::*;
use lorenz_attractor::*;
use math::*;
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
		Back,
		ToggleUnlimitedFov,
	]};
	let mut is_paused = false;
	let mut pause_menu_item_index: u32 = 0;


	let help_lines = [
		"controls:",
		"f1 - show help screen",
		"wasd/arrows/pl;' - move",
		"shift - move fast",
		"space/ctrl/alt - fly up/down",
		"tab/i - open inventory",
		"f3 - toggle info overlay",
		"f5 - change movement mode",
	].map(|s| s.to_uppercase());
	let mut is_help_opened = false;
	let mut help_item_index: u32 = 0;


	let mut dimension = Dimension::Base;
	const DIM_BASE_LA_SPEED: float = 1e-5;
	let mut dim_base_la_for_floor_color = LorenzAttractor::new()
		.offset_params_(Vec3f::random_unit_cube(&mut rng) * 0.1)
		.offset_xyz(30., 0., 0.);
	fn base_color(la: &LorenzAttractor) -> u8 {
		let x = la.get_linear_combination(1., 1., 1.);
		let c = x.clamp(1., 80.) as u8;
		c
	}


	let inventory_items = { use InventoryItem::*; vec![
		SurfaceWorld,
	]};
	let mut is_inventory_opened = false;
	let mut inventory_item_index: u32 = 0;
	fn gen_surface_world_param(rng: &mut ThreadRng) -> (float, float, float, float) {
		// returns amplitude, phase, cx, cz
		(
			rng.random_range(0. ..= 3_f32).powi(2),
			rng.random_range(0. ..= 2.*PI),
			rng.random_range(-2. ..= 2.),
			rng.random_range(-2. ..= 2.),
		)
	}
	fn gen_surface_world_params(rng: &mut ThreadRng) -> Vec<(float, float, float, float)> {
		Vec::from_fn(
			rng.random_range(1. ..= 10_f32).powi(2).round() as usize,
			|_i| gen_surface_world_param(rng)
		)
	}
	let mut surface_world_params = gen_surface_world_params(&mut rng);


	const CHUNKS_N: u32 = 17;
	let render_distance: u32 = 2;
	let mut chunks = Vec2D::<Chunk>::from_fn(CHUNKS_N, CHUNKS_N, |_x, _z| {
		Chunk::new_random(&mut rng)
	});
	// println!("chunks.len = {}", chunks.iter().count());

	const GROUNDED_CAMERA_Y: float = 1.5;
	const CAMERA_DEFAULT_POSITION: Vec3f = Vec3f::from_y(GROUNDED_CAMERA_Y);
	let mut camera = Camera {
		pos: CAMERA_DEFAULT_POSITION,
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
	let mut is_unlimited_fov = false;

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

		let is_overlay = is_paused || is_inventory_opened || is_help_opened;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => { break 'main_loop }
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					// TODO(feat): close other menus?
					is_paused = !is_paused;
					is_redraw_needed = true;
				}
				Event::MouseMotion { xrel, yrel, .. } if !is_overlay => {
					const ROTATION_SPEED: float = 0.6;
					// left/right
					camera.forward += camera.basis().0 * xrel * camera.fov * DELTA_TIME * ROTATION_SPEED;
					camera.forward.normalize();
					// up/down
					camera.forward -= camera.basis().1 * yrel * camera.fov * DELTA_TIME * ROTATION_SPEED;
					camera.forward.normalize();
					is_redraw_needed = true;
				}
				// TODO: use MouseWheel for FOV?
				Event::KeyDown { keycode: Some(Keycode::F5), .. } if !is_overlay => {
					movement_type.next();
					match movement_type { // #bqooaj
						MovementType::Grounded => {
							camera.pos.y = GROUNDED_CAMERA_Y;
							camera.reset_roll();
						}
						MovementType::FlyingMClike => {}
						MovementType::FlyingGMlike => {}
						MovementType::FpvLike => {}
					}
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::F1), .. } if !is_paused && !is_inventory_opened => {
					is_help_opened = !is_help_opened;
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::F3), .. } if !is_overlay => {
					is_extra_info_shown = !is_extra_info_shown;
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::I | Keycode::Tab), .. } if !is_paused && !is_help_opened => {
					is_inventory_opened = !is_inventory_opened;
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Up), .. } if is_overlay => {
					if is_paused {
						pause_menu_item_index = pause_menu_item_index.dec_mod(pause_menu_items.len() as u32);
					}
					else if is_help_opened {
						help_item_index = help_item_index.dec_mod(help_lines.len() as u32);
					}
					else if is_inventory_opened {
						inventory_item_index = inventory_item_index.dec_mod(inventory_items.len() as u32);
					}
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Down), .. } if is_overlay => {
					if is_paused {
						pause_menu_item_index = pause_menu_item_index.inc_mod(pause_menu_items.len() as u32);
					}
					else if is_help_opened {
						help_item_index = help_item_index.inc_mod(help_lines.len() as u32);
					}
					else if is_inventory_opened {
						inventory_item_index = inventory_item_index.inc_mod(inventory_items.len() as u32);
					}
					is_redraw_needed = true;
				}
				Event::KeyDown { keycode: Some(Keycode::Return), .. } if is_overlay => {
					if is_paused {
						use PauseMenuItem::*;
						match pause_menu_items[pause_menu_item_index as usize] {
							Quit => {
								break 'main_loop
							}
							Back => {
								dimension = Dimension::Base;
								camera.pos = CAMERA_DEFAULT_POSITION;
								current_chunk_x = 0;
								current_chunk_z = 0;
							}
							ToggleUnlimitedFov => {
								is_unlimited_fov = !is_unlimited_fov;
								if !is_unlimited_fov {
									camera.fov = camera.fov.clamp(FOV_MIN*1.1, FOV_MAX/1.1);
								}
							}
							Text(_) => {}
						}
						is_paused = false;
						is_redraw_needed = true;
					}
					else if is_inventory_opened {
						use InventoryItem::*;
						match inventory_items[inventory_item_index as usize] {
							SurfaceWorld => {
								dimension = Dimension::SurfaceWorld;
								surface_world_params = gen_surface_world_params(&mut rng);
								is_redraw_needed = true;
							}
							Text(_) => {}
						}
						is_inventory_opened = false;
					}
				}
				_ => {}
			}
		}
		let keyboard = event_pump.keyboard_state();

		// handle inputs:
		let mut move_speed: float = 20.;
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::LShift, Scancode::RShift]) {
			move_speed *= 3.;
		}
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::Up, Scancode::W, Scancode::P]) {
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
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::Down, Scancode::S, Scancode::Semicolon]) {
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
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::Left, Scancode::A, Scancode::L]) {
			camera.pos -= camera.basis().0 * DELTA_TIME * move_speed;
			is_redraw_needed = true;
		}
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::Right, Scancode::D, Scancode::Apostrophe]) {
			camera.pos += camera.basis().0 * DELTA_TIME * move_speed;
			is_redraw_needed = true;
		}
		if !is_overlay && keyboard.is_scancode_pressed(Scancode::Space) {
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
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::LCtrl, Scancode::LAlt, Scancode::RCtrl, Scancode::RAlt]) {
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
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::Q, Scancode::O]) {
			match movement_type {
				MovementType::Grounded => {}
				MovementType::FlyingMClike => {}
				MovementType::FlyingGMlike => {}
				MovementType::FpvLike => {
					camera.up -= camera.basis().0 * DELTA_TIME * ROLL_SPEED;
					camera.up.normalize();
					is_redraw_needed = true;
				}
			}
		}
		if !is_overlay && keyboard.is_scancodes_pressed_any(&[Scancode::E, Scancode::LeftBracket]) {
			match movement_type {
				MovementType::Grounded => {}
				MovementType::FlyingMClike => {}
				MovementType::FlyingGMlike => {}
				MovementType::FpvLike => {
					camera.up += camera.basis().0 * DELTA_TIME * ROLL_SPEED;
					camera.up.normalize();
					is_redraw_needed = true;
				}
			}
		}
		if !is_overlay && keyboard.is_scancode_pressed(Scancode::R) {
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

		const FOV_MIN: float = 1e-1 * DEG_TO_RAD;
		const FOV_MAX: float = 170. * DEG_TO_RAD;
		const FOV_RANGE: float = FOV_MAX - FOV_MIN;
		const FOV_CHANGE_SPEED: float = 3. * DELTA_TIME;
		if keyboard.is_scancode_pressed(Scancode::Equals) {
			if is_unlimited_fov {
				camera.fov -= DELTA_TIME;
			} else {
				camera.fov = FOV_MIN + FOV_RANGE * sigmoid(asigmoid((camera.fov-FOV_MIN)/FOV_RANGE) - FOV_CHANGE_SPEED);
			}
			is_redraw_needed = true;
		}
		if keyboard.is_scancode_pressed(Scancode::Minus) {
			if is_unlimited_fov {
				camera.fov += DELTA_TIME;
			} else {
				camera.fov = FOV_MIN + FOV_RANGE * sigmoid(asigmoid((camera.fov-FOV_MIN)/FOV_RANGE) + FOV_CHANGE_SPEED);
			}
			is_redraw_needed = true;
		}

		// physics update:
		if !is_paused /* TODO: && exist what needs to be updated */ {
			match dimension {
				Dimension::Base => {
					dim_base_la_for_floor_color.step(DIM_BASE_LA_SPEED);
					for (_x, _z, chunk) in chunks.iter_mut() {
						for (_pos, ro) in chunk.renderable_objects.iter_mut() {
							ro.update(DELTA_TIME);
						}
					}
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
					is_redraw_needed = true;
				}
				Dimension::SurfaceWorld => {}
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

			match dimension {
				Dimension::Base => {
					for (dx, dz, _x, _z, _chunk) in chunks.iter_around_wrapping(current_chunk_x, current_chunk_z, render_distance) {
						const STEP: float = 1.;
						let mut x = -CHUNK_SIZE_HALF * (1. - 1e-2);
						while x < CHUNK_SIZE_HALF {
							let mut z = -CHUNK_SIZE_HALF * (1. - 1e-2);
							while z < CHUNK_SIZE_HALF {
								let pos = Vec3f::from_xz((dx as float)*CHUNK_SIZE + x, (dz as float)*CHUNK_SIZE + z);
								canvas.set_draw_color({
									let c = base_color(&dim_base_la_for_floor_color);
									let pos_rel_to_cam = pos - camera.pos;
									// TODO: better attenuation curve
									let c = ((c as float) / (1. + 2e-3*pos_rel_to_cam.norm2())) as u8;
									Color::RGB(c, c, c)
								});
								let lines = [
									(Vec3f::new(pos.x - STEP/3., 0., pos.z - STEP/3.),
									 Vec3f::new(pos.x + STEP/3., 0., pos.z + STEP/3.)),
									(Vec3f::new(pos.x - STEP/3., 0., pos.z + STEP/3.),
									 Vec3f::new(pos.x + STEP/3., 0., pos.z - STEP/3.)),
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
						for (pos, ro) in chunk.renderable_objects.iter() {
							use SdlRenderableShape::*;
							let shift: Vec3f = *pos + Vec3f::from_xz((dx as float)*CHUNK_SIZE, (dz as float)*CHUNK_SIZE);
							canvas.set_draw_color({
								let Color { r, g, b, .. } = chunk.color;
								let pos_rel_to_cam = shift - camera.pos;
								// TODO: better attenuation curve
								let r = ((r as float) / (1. + 1e-2*pos_rel_to_cam.norm2())) as u8;
								let g = ((g as float) / (1. + 1e-2*pos_rel_to_cam.norm2())) as u8;
								let b = ((b as float) / (1. + 1e-2*pos_rel_to_cam.norm2())) as u8;
								Color::RGB(r, g, b)
							});
							for renderable_shape in ro.get_renderable_shapes(&camera) {
								match renderable_shape {
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
					}
				}
				Dimension::SurfaceWorld => {
					const MESH_SIZE: u32 = 30;
					const MESH_STEP: float = 0.9;
					const LODS: &[(u32, Color)] = &[
						(4, Color::RGB(64, 64, 64)),
						(2, Color::GRAY),
						(1, Color::WHITE),
					];
					let params = &surface_world_params;
					fn surface_at(x: float, z: float, params: &[(f32, f32, f32, f32)]) -> float {
						params.iter().map(|(amplitude, phase, cx, cz)| {
							amplitude * sin(phase + cx*x + cz*z) / (params.len() as float)//.powf(*amplitude)
						}).sum()
					}
					for (lod_n, lod_color) in LODS {
						canvas.set_draw_color(*lod_color);
						let mesh_step = MESH_STEP * (*lod_n as float);
						let cx = camera.pos.x - (MESH_SIZE as float - 1.) * mesh_step / 2.;
						let cz = camera.pos.z - (MESH_SIZE as float - 1.) * mesh_step / 2.;
						let surface = Vec2D::from_fn(MESH_SIZE, MESH_SIZE, |x, z| {
							let x = (x as float) * mesh_step;
							let z = (z as float) * mesh_step;
							surface_at(x + cx - cx.rem_euclid(mesh_step), z + cz - cz.rem_euclid(mesh_step), params)
						});
						let cx = cx - cx.rem_euclid(mesh_step);
						let cz = cz - cz.rem_euclid(mesh_step);
						// TODO(optim): use draw_lines/chain
						// let mut lines_x = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
						// let mut lines_z = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
						for z in 0..MESH_SIZE-1 {
							let zf = (z as float) * mesh_step;
							for x in 0..MESH_SIZE-1 {
								let xf = (x as float) * mesh_step;
								let line1 = (vec3![xf+cx, surface[(x,z)], zf+cz], vec3![xf+cx+mesh_step, surface[(x+1,z)], zf+cz]);
								if let Some((a,b)) = camera.project_line(&line1, wf, hf) {
									canvas.draw_line(a,b).unwrap();
								}
								let line2 = (vec3![xf+cx, surface[(x,z)], zf+cz], vec3![xf+cx, surface[(x,z+1)], zf+cz+mesh_step]);
								if let Some((a,b)) = camera.project_line(&line2, wf, hf) {
									canvas.draw_line(a,b).unwrap();
								}
							}
						}
						for x in 0..MESH_SIZE-1 {
							let z = MESH_SIZE-1;
							let zf = (z as float) * mesh_step;
							let xf = (x as float) * mesh_step;
							let line1 = (vec3![xf+cx, surface[(x,z)], zf+cz], vec3![xf+cx+mesh_step, surface[(x+1,z)], zf+cz]);
							if let Some((a,b)) = camera.project_line(&line1, wf, hf) {
								canvas.draw_line(a,b).unwrap();
							}
						}
						for z in 0..MESH_SIZE-1 {
							let x = MESH_SIZE-1;
							let xf = (x as float) * mesh_step;
							let zf = (z as float) * mesh_step;
							let line2 = (vec3![xf+cx, surface[(x,z)], zf+cz], vec3![xf+cx, surface[(x,z+1)], zf+cz+mesh_step]);
							if let Some((a,b)) = camera.project_line(&line2, wf, hf) {
								canvas.draw_line(a,b).unwrap();
							}
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

			if is_help_opened {
				const PADDING: float = 30.;
				const ITEM_Y: float = 30.;
				const ITEMS_N: u32 = 15;
				debug_assert_eq!(1, ITEMS_N % 2);
				const MENU_SIZE_X: float = 1000.;
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
				let i_init: u32 = help_item_index.saturating_sub((ITEMS_N-1)/2);
				let mut i: u32 = i_init;
				while i - i_init < ITEMS_N && i < help_lines.len() as u32 {
					let menu_item = &help_lines[i as usize];
					if i == help_item_index {
						canvas.set_draw_color(ITEM_SELECTED_COLOR);
					}
					let item_cx = wfh;
					let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
					// canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
					canvas.render_text(menu_item, ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
					if i == help_item_index {
						canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
					}
					i += 1;
				}
			}

			if is_inventory_opened {
				const PADDING: float = 30.;
				const ITEM_Y: float = 50.;
				const ITEMS_N: u32 = 11;
				debug_assert_eq!(1, ITEMS_N % 2);
				const MENU_SIZE_X: float = 900.;
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
				let i_init: u32 = inventory_item_index.saturating_sub((ITEMS_N-1)/2);
				let mut i: u32 = i_init;
				while i - i_init < ITEMS_N && i < inventory_items.len() as u32 {
					let inventory_item = &inventory_items[i as usize];
					if i == inventory_item_index {
						canvas.set_draw_color(ITEM_SELECTED_COLOR);
					}
					let item_cx = wfh;
					let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
					canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
					canvas.render_text(inventory_item.to_str(), ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
					if i == inventory_item_index {
						canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
					}
					i += 1;
				}
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





enum Dimension {
	Base, // TODO: rename? Home, RotatingBH
	// BaseAlt, // TODO: rename? HomeAlt, StaticBH
	SurfaceWorld, // TODO(feat): function
}
impl Dimension {
	fn to_str(&self) -> &str {
		use Dimension::*;
		match self {
			Base => "BASE",
			SurfaceWorld => "SURFACE WORLD",
		}
	}
}





enum InventoryItem {
	SurfaceWorld, // TODO(feat): function
	Text(String), // just for test
}
impl InventoryItem {
	fn to_str(&self) -> &str {
		use InventoryItem::*;
		match self {
			SurfaceWorld => "SURFACE WORLD",
			Text(text) => text,
		}
	}
}





enum PauseMenuItem {
	Quit,
	Back,
	ToggleUnlimitedFov,
	Text(String), // just for test
}
impl PauseMenuItem {
	fn to_str(&self) -> &str {
		use PauseMenuItem::*;
		match self {
			Quit => "QUIT",
			Back => "BACK",
			ToggleUnlimitedFov => "TOGGLE UNLIMITED FOV",
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
	RotatingSimplex { initpoints_rotplanes_rotvels_phases: Vec<(Vec3f, Vec3f, float, float)> },
	RotatingIcosahedron { size: float, global_rotvel: float, rotplanes_rotvels_angles: Vec<(Vec3f, float, float)> },
	Kitty { size: float, rotvel: float, phase: float },
	Graph3d { connect_n: u32, global_rotvel: float, initpoints_rotplanes_rotvels_phases: Vec<(Vec3f, Vec3f, float, float)> },
}
impl RenderableObject {
	fn is_time_dependent(&self) -> bool {
		use RenderableObject::*;
		match self {
			| LorenzAttractor { .. }
			| RotatingSimplex { .. }
			| RotatingIcosahedron { .. }
			| Kitty { .. }
			| Graph3d { .. }
			=> true,
			| Cube { .. }
			| Monolith { .. }
			=> false,
		}
	}
	fn update(&mut self, delta_time: float) {
		use RenderableObject::*;
		match self {
			| Cube { .. }
			| Monolith { .. }
			=> {}
			LorenzAttractor { la, last_points, max_len, size: _ } => {
				// TODO(optim): use Queue (VecDeque)
				last_points.push(la.get_xyz_as_vec3d());
				if last_points.len() as u32 > *max_len {
					let _ = last_points.remove(0);
				}
				la.step(1e-2);
			}
			RotatingSimplex { initpoints_rotplanes_rotvels_phases } => {
				for (_initpoint, _rotplane, rotation_velocity, phase) in initpoints_rotplanes_rotvels_phases.iter_mut() {
					*phase += *rotation_velocity * delta_time;
					if *phase > TAU {
						*phase -= TAU;
					}
					// debug_assert!(*phase >= 0.);
				}
			}
			RotatingIcosahedron { rotplanes_rotvels_angles, global_rotvel, size: _ } => {
				for (i, (_rotplane, rotation_velocity, angle)) in rotplanes_rotvels_angles.iter_mut().enumerate() {
					*angle += *global_rotvel * *rotation_velocity * delta_time / ((i + 1) as float);
				}
			}
			Kitty { phase, rotvel, .. } => {
				*phase += *rotvel * delta_time;
			}
			Graph3d { initpoints_rotplanes_rotvels_phases, global_rotvel, .. } => {
				for (_initpoint, _rotplane, rotation_velocity, phase) in initpoints_rotplanes_rotvels_phases.iter_mut() {
					*phase += *rotation_velocity * *global_rotvel * delta_time;
					if *phase > TAU {
						*phase -= TAU;
					}
					// debug_assert!(*phase >= 0.);
				}
			}
		}
	}
	fn get_renderable_shapes(&self, camera: &Camera) -> Vec<SdlRenderableShape> {
		use RenderableObject::*;
		use SdlRenderableShape::*;
		match self {
			Cube { size } => {
				let s = size / 2.;
				vec![Lines(vec![
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
				])]
			}
			LorenzAttractor { size, last_points, .. } => {
				vec![Chain(last_points.iter().map(|&p| p * *size).collect())]
			}
			Monolith { sizes } => {
				vec![Lines(sizes.iter().map(|size| {
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
				}).flatten().collect())]
			}
			RotatingSimplex { initpoints_rotplanes_rotvels_phases } => {
				let points: Vec<Vec3f> = initpoints_rotplanes_rotvels_phases.iter()
					.map(|(initpoint, rotplane, _rotvel, phase)| {
						initpoint.rotate_around(*rotplane, *phase)
					})
					.collect();
				let mut lines = vec![];
				for i in 0 .. points.len() {
					for j in i+1 .. points.len() {
						let a = points[i];
						let b = points[j];
						lines.push((a, b));
					}
				}
				vec![Lines(lines)]
			}
			RotatingIcosahedron { size, rotplanes_rotvels_angles, .. } => {
				const PHI: float = GOLDEN_RATIO;
				let mut vertices = [
					// src: https://en.wikipedia.org/wiki/Regular_icosahedron
					vec3![-PHI, 0, -1],
					vec3![-PHI, 0,  1],
					vec3![ PHI, 0, -1],
					vec3![ PHI, 0,  1],
					vec3![-1, -PHI, 0],
					vec3![-1,  PHI, 0],
					vec3![ 1, -PHI, 0],
					vec3![ 1,  PHI, 0],
					vec3![0, -1, -PHI],
					vec3![0, -1,  PHI],
					vec3![0,  1, -PHI],
					vec3![0,  1,  PHI],
				].map(|v| v * *size);
				for (rotplane, _rotvel, angle) in rotplanes_rotvels_angles.iter() {
					for vertex in vertices.iter_mut() {
						*vertex = vertex.rotate_around(*rotplane, *angle);
					}
				}
				const NEARESTS_VERTICES_INDICES: [[u8; 5]; 12] = [
					[ 1, 4, 5, 8, 10, ], // 0
					[ 0, 4, 5, 9, 11, ], // 1
					[ 3, 6, 7, 8, 10, ], // 2
					[ 2, 6, 7, 9, 11, ], // 3
					[ 0, 1, 6, 8, 9, ], // 4
					[ 0, 1, 7, 10, 11, ], // 5
					[ 2, 3, 4, 8, 9, ], // 6
					[ 2, 3, 5, 10, 11, ], // 7
					[ 0, 2, 4, 6, 10, ], // 8
					[ 1, 3, 4, 6, 11, ], // 9
					[ 0, 2, 5, 7, 8, ], // 10
					[ 1, 3, 5, 7, 9, ], // 11
				];
				// const VERTEX_TO_REMOVE: u8 = 5;
				let mut lines = Vec::with_capacity(30);
				for (vertex_index, (&vertex, &nearest_vertices_indices)) in vertices.iter().zip(NEARESTS_VERTICES_INDICES.iter()).enumerate() {
					let vertex_index = vertex_index as u8;
					// if vertex_index == VERTEX_TO_REMOVE { continue }
					for nearest_vertex_index in nearest_vertices_indices {
						if nearest_vertex_index < vertex_index { continue }
						// if nearest_vertex_index == VERTEX_TO_REMOVE { continue }
						lines.push((vertex, vertices[nearest_vertex_index as usize]));
					}
				}
				debug_assert_eq!(30, lines.len());
				vec![Lines(lines)]
			}
			Kitty { size, phase, .. } => {
				let angles_of_points_on_circle_20: Vec<float> = {
					const N: u32 = 20;
					let tau_div_n = TAU / (N as float);
					Vec::from_fn(N as usize, |i| (i as float) * tau_div_n)
				};
				let (cam_r, cam_u, cam_f) = camera.basis();
				let points_outline: Vec<Vec3f> = angles_of_points_on_circle_20.iter()
					.chain(std::iter::once(angles_of_points_on_circle_20.first().unwrap()))
					.enumerate()
					.flat_map(|(i, angle)| {
						// we do some tomfoolery magic here, after all, we love casting spells
						if i == 11 || i == 19 {
							None
						} else {
							let size = size * if i == 12 || i == 18 { 1.5 } else { 1. };
							// Some(get_point_on_circle_in_3d(cam_r, cam_f, *angle, *phase, size))
							Some(
								cam_r.rotate_around(cam_f, *angle) * size
								+ cam_r.rotate_around(cam_f, *phase) * 0.2
							)
						}
					})
					.collect();
				let angles_of_points_on_circle_10: Vec<float> = {
					const N: u32 = 10;
					let tau_div_n = TAU / (N as float);
					Vec::from_fn(N as usize, |i| (i as float) * tau_div_n)
				};
				let points_eye_left: Vec<Vec3f> = angles_of_points_on_circle_10.iter()
					.chain(std::iter::once(angles_of_points_on_circle_10.first().unwrap()))
					.map(|angle| {
						cam_r.rotate_around(cam_f, *angle) * 0.1
						+ cam_r.rotate_around(cam_f, *phase) * 0.2
						+ cam_r * 0.5 + cam_u * 0.2
						+ cam_f * 0.05
					})
					.collect();
				let points_eye_right: Vec<Vec3f> = angles_of_points_on_circle_10.iter()
					.chain(std::iter::once(angles_of_points_on_circle_10.first().unwrap()))
					.map(|angle| {
						cam_r.rotate_around(cam_f, *angle) * 0.1
						+ cam_r.rotate_around(cam_f, *phase) * 0.2
						- cam_r * 0.5 + cam_u * 0.2
						+ cam_f * 0.05
					})
					.collect();
				let points_smile = [
					(1.78, -0.5),
					(1.754, -1.),
					(1.5, -1.41414),
					(1.1, -1.654),
					(0.7, -1.695),
					(0.3, -1.566),
					(0., -1.3),
					(-0.3, -1.566),
					(-0.7, -1.695),
					(-1.1, -1.654),
					(-1.5, -1.41414),
					(-1.754, -1.),
					(-1.78, -0.5),
				];
				let points_smile: Vec<Vec3f> = points_smile.into_iter()
					.map(|(x, y)| {
						// cam_r.rotate_around(cam_f, *angle) * 0.1
						cam_r.rotate_around(cam_f, *phase) * 0.2
						- cam_r * x * 0.2 + cam_u * y * 0.2
						- cam_u * 0.1
					})
					.collect();
				vec![
					Chain(points_outline),
					Chain(points_eye_left),
					Chain(points_eye_right),
					Chain(points_smile),
				]
			}
			Graph3d { connect_n, initpoints_rotplanes_rotvels_phases, .. } => {
				let points: Vec<Vec3f> = initpoints_rotplanes_rotvels_phases.iter()
					.map(|(initpoint, rotplane, _rotvel, phase)| {
						initpoint.rotate_around(*rotplane, *phase)
					})
					.collect();
				let mut neighbors: Vec<Vec<u32>> = Vec::from_fn(points.len(), |_i| Vec::with_capacity(points.len()));
				for i in 0 .. points.len() {
					let mut distances = vec![];
					for j in 0 .. points.len() { // or from i+1 ?
						let dist2 = if i != j { points[i].dist2_to(points[j]) } else { float::MAX };
						distances.push((j as u32, dist2));
					}
					distances.sort_by(|(_j1, d1), (_j2, d2)| d1.partial_cmp(d2).unwrap());
					neighbors[i] = distances[..*connect_n as usize].iter().map(|(j, _d)| *j).collect();
				}
				let mut lines = vec![];
				for i in 0 .. points.len() {
					for j in neighbors[i].iter() {
						if (i as u32) >= *j { continue }
						let a = points[i];
						let b = points[*j as usize];
						lines.push((a, b));
					}
				}
				vec![Lines(lines)]
			}
		}
	}
}





const CHUNK_SIZE: float = 20.;
const CHUNK_SIZE_HALF: float = CHUNK_SIZE / 2.;
struct Chunk {
	color: Color,
	renderable_objects: Vec<(Vec3f, RenderableObject)>,
}
impl Chunk {
	fn new_random(rng: &mut ThreadRng) -> Self {
		Self {
			// color: Color::RGB(255/(CHUNKS_N as u8)*(1 + x as u8), 255/(CHUNKS_N as u8)*(1 + z as u8), 0), // for dbg
			color: Color::RGB(rng.random(), rng.random(), rng.random()),
			renderable_objects: {
				use V8::*;
				// TODO: write macros and use `pr => { ... }`
				match rng.random_variant_weighted([5., 1., 0.3, 1e-2, 0.2, 0.3, 1e-3, 0.1]) {
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
								Vec3f::random_unit_cube(rng) * 0.1
							).set_xyz_(
								Vec3f::random_unit(rng) * rng.random_range(0.1 ..= 0.2)
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
							initpoints_rotplanes_rotvels_phases: {
								macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
								let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
								let n = rng.random_range(4 ..= 10);
								(0..n).map(|_i| (
									Vec3f::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
									Vec3f::random_unit(rng),
									rng.random_range(0.5 ..= 1.4_f32).powi(2),
									rng.random_range(0. ..= TAU),
								)).collect()
							},
						}
					)],
					_6 => vec![(
						Vec3f::from_y(rng.random_range(1. ..= 2.)),
						RenderableObject::RotatingIcosahedron {
							size: rng.random_range(0.5 ..= 2.5),
							global_rotvel: rng.random_range(0.01 ..= 1.),
							rotplanes_rotvels_angles: Vec::from_fn(
								rng.random_range(1 ..= 5),
								|_i| (
									Vec3f::random_unit(rng),
									rng.random_range(0.1 ..= 2.),
									rng.random_range(0. ..= TAU),
								)
							),
						}
					)],
					_7 => vec![(
						Vec3f::from_y(rng.random_range(0.5 ..= 1.)),
						RenderableObject::Kitty {
							size: rng.random_range(1. ..= 1.5),
							rotvel: rng.random_range(5. ..= 15.),
							phase: 0.,
						}
					)],
					_8 => vec![(
						Vec3f::from_y(rng.random_range(2. ..= 5.)),
						RenderableObject::Graph3d {
							connect_n: rng.random_range(1 ..= 6),
							global_rotvel: rng.random_range(0.01 ..= 2.),
							initpoints_rotplanes_rotvels_phases: {
								macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
								let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
								let n = rng.random_range(10 ..= 200);
								(0..n).map(|_i| (
									Vec3f::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
									Vec3f::random_unit(rng),
									rng.random_range(0.5 ..= 1.4_f32).powi(2),
									rng.random_range(0. ..= TAU),
								)).collect()
							},
						}
					)],
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
		let f = 1. / tan(self.fov * 0.5);

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


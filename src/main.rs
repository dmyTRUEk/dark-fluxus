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
	// unused_variables, // FIXME
)]

#![feature(
	default_field_values,
	vec_from_fn,
)]

use std::{any::type_name_of_val, f32::consts::{FRAC_PI_2, GOLDEN_RATIO, PI, TAU}, thread::sleep, time::{Duration, Instant, SystemTime}};

use either::Either;
use rand::{RngExt, rng, rngs::ThreadRng};
use pollster::block_on;
use wgpu::util::DeviceExt;
use winit::{event::{DeviceEvent, ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{Key, NamedKey, SmolStr}, window::Window};
use glam::{Mat4, Vec3, Quat};

mod color_u8;
mod consts;
mod extensions;
mod float_type;
// mod font_rendering;
mod lorenz_attractor;
mod math;
mod math_aliases;
mod typesafe_rng;
mod utils_io;
mod vec2D;
// mod vec2d;
// mod vec3d;
mod vec2_ext;
mod vec3_ext;
mod zqqx_lang;

use color_u8::*;
use consts::*;
use extensions::*;
use float_type::*;
// use font_rendering::*;
use lorenz_attractor::*;
use math::*;
use math_aliases::*;
use typesafe_rng::*;
// use utils_io::*;
use vec2D::*;
// use vec2d::*;
// use vec3d::*;
use vec2_ext::*;
use vec3_ext::*;
// use zqqx_lang::*;


// TODO(refactor): f32 -> float


fn main() {
	let event_loop = winit::event_loop::EventLoop::new().unwrap();
	let mut app = AppMaybeUninit::new();
	event_loop.run_app(&mut app).unwrap();
}



#[allow(clippy::large_enum_variant)]
enum AppMaybeUninit {
	Uninit,
	Init(App),
}
impl AppMaybeUninit {
	fn new() -> Self {
		Self::Uninit
	}
}
impl winit::application::ApplicationHandler for AppMaybeUninit {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) { // aka init
		let window = event_loop
			.create_window(
				winit::window::WindowAttributes::default()
					.with_title(format!("Dark Fluxus v{}", env!("CARGO_PKG_VERSION")))
					.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
			).unwrap();

		// let monitor = window.current_monitor();
		// window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));

		// let monitor = window.current_monitor().unwrap();
		// let video_mode = monitor.video_modes().next().unwrap();
		// window.set_fullscreen(Some(Fullscreen::Exclusive(video_mode)));

		let window_ref: &'static Window = Box::leak(Box::new(window));
		window_ref.set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
		window_ref.set_cursor_visible(false);

		let renderer = Renderer::new(window_ref);

		*self = Self::Init(App::new(window_ref, renderer));
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: winit::window::WindowId, event: WindowEvent) {
		if let Self::Init(app) = self {
			app.window_event(event_loop, id, event);
		}
	}

	fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: winit::event::DeviceId, event: DeviceEvent) {
		if let Self::Init(app) = self {
			app.device_event(event_loop, device_id, event);
		}
	}

	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		if let Self::Init(app) = self {
			app.about_to_wait(event_loop);
		}
	}
}



struct App {
	window: &'static Window,
	renderer: Renderer,
	state: State,
	rng: ThreadRng,
}
impl App {
	fn new(window: &'static Window, renderer: Renderer) -> Self {
		let mut rng = rng();
		let state = State::new(renderer.config.width as f32, renderer.config.height as f32, &mut rng);
		Self {
			window,
			renderer,
			state,
			rng,
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: winit::window::WindowId, event: WindowEvent) {
		use {Key::*, NamedKey::*, WindowEvent::*};
		let is_overlay = self.state.is_overlay();
		// let State { camera, input, last_update_inst, is_redraw_needed, help_lines, is_help_opened, help_line_index, pause_menu_items, is_paused, pause_menu_item_index, is_darkness_at_base, dimension, dim_base_la_for_floor_color, inventory_items, is_inventory_opened, inventory_item_index, surface_world_params, chunks, render_distance, current_chunk_x, current_chunk_z, is_alt_topology, is_x_flipped_global, is_z_flipped_global, movement_type, tick_n, frame_n, is_extra_info_shown } = &mut self.state;
		let char_i: SmolStr = "i".into();
		match event {
			CloseRequested
			// | KeyboardInput { event: KeyEvent { logical_key: Named(Escape), state: ElementState::Pressed, .. }, .. }
			=> {
				event_loop.exit();
			}
			RedrawRequested => {
				self.render();
			}
			Resized(_new_size) => {
				self.reconfigure_surface();
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(Escape), repeat: false, .. }, .. } => {
				if self.state.is_inventory_opened {
					self.state.is_inventory_opened = false;
				}
				else if self.state.is_help_opened {
					self.state.is_help_opened = false;
				}
				else {
					self.state.is_paused = !self.state.is_paused;
				}
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(F1), repeat: false, .. }, .. } if !self.state.is_paused && !self.state.is_inventory_opened => {
				self.state.is_help_opened = !self.state.is_help_opened;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(F3), repeat: false, .. }, .. } if !is_overlay => {
				self.state.is_extra_info_shown = !self.state.is_extra_info_shown;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(F5), repeat: false, .. }, .. } if !is_overlay => {
				self.state.camera.next_movement_type();
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key, repeat: false, .. }, .. } if (logical_key == Character(char_i) || logical_key == Named(Tab)) && !self.state.is_paused && !self.state.is_help_opened => {
				self.state.is_inventory_opened = !self.state.is_inventory_opened;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(ArrowUp), repeat: false, .. }, .. } if is_overlay => {
				if self.state.is_paused {
					self.state.pause_menu_item_index = self.state.pause_menu_item_index.dec_mod(self.state.pause_menu_items.len() as u32);
				}
				else if self.state.is_help_opened {
					self.state.help_line_index = self.state.help_line_index.dec_mod(self.state.help_lines.len() as u32);
				}
				else if self.state.is_inventory_opened {
					self.state.inventory_item_index = self.state.inventory_item_index.dec_mod(self.state.inventory_items.len() as u32);
				}
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(ArrowDown), repeat: false, .. }, .. } if is_overlay => {
				if self.state.is_paused {
					self.state.pause_menu_item_index = self.state.pause_menu_item_index.inc_mod(self.state.pause_menu_items.len() as u32);
				}
				else if self.state.is_help_opened {
					self.state.help_line_index = self.state.help_line_index.inc_mod(self.state.help_lines.len() as u32);
				}
				else if self.state.is_inventory_opened {
					self.state.inventory_item_index = self.state.inventory_item_index.inc_mod(self.state.inventory_items.len() as u32);
				}
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(Enter), repeat: false, .. }, .. } if is_overlay => {
				if self.state.is_paused {
					use PauseMenuItem::*;
					match self.state.pause_menu_items[self.state.pause_menu_item_index as usize] {
						Quit => {
							event_loop.exit();
						}
						Back => {
							self.state.dimension = Dimension::Base;
							self.state.camera.reset_position();
							self.state.current_chunk_x = 0;
							self.state.current_chunk_z = 0;
						}
						GetRandomItem => {
							self.state.inventory_items.push(InventoryItem::new_random(&mut self.rng));
						}
						ToggleTopology => {
							self.state.is_alt_topology = !self.state.is_alt_topology;
						}
						ToggleDarkness => {
							self.state.is_darkness_at_base = !self.state.is_darkness_at_base;
						}
						ToggleUnlimitedFov => {
							self.state.camera.toggle_unlimited_fov();
						}
						ToggleShakyFov => {
							self.state.camera.toggle_shaky_fov();
						}
						Text(_) => {}
					}
					self.state.is_paused = false;
					self.state.is_redraw_needed = true;
				}
				else if self.state.is_inventory_opened {
					use InventoryItem::*;
					let remove_self = true;
					match &self.state.inventory_items[self.state.inventory_item_index as usize] {
						SurfaceWorld => {
							self.state.dimension = Dimension::SurfaceWorld;
							self.state.surface_world_params = gen_surface_world_params(&mut self.rng);
						}
						RenderableObject_(ro) => {
							self.state.chunks[(self.state.current_chunk_x, self.state.current_chunk_z)].renderable_objects.push((
								self.state.camera.position.with_y(self.rng.random_range(0. ..= 5.)),
								ro.clone()
							));
						}
						Text(_) => {}
					}
					if remove_self {
						let _ = self.state.inventory_items.remove(self.state.inventory_item_index as usize);
						if self.state.inventory_item_index >= self.state.inventory_items.len() as u32 {
							self.state.inventory_item_index = (self.state.inventory_items.len() as u32).saturating_sub(1);
						}
					}
					self.state.is_inventory_opened = false;
					self.state.is_redraw_needed = true;
				}
			}
			KeyboardInput { event, .. } => { // handle "continuous" input
				// dbg!(event);
				let is_pressed = event.state == ElementState::Pressed;
				let input = &mut self.state.input;
				match event.logical_key {
					Character(c) if c == "w" || c == "p" => input.forward = is_pressed,
					Character(c) if c == "s" || c == ";" => input.back = is_pressed,
					Character(c) if c == "a" || c == "l" => input.left = is_pressed,
					Character(c) if c == "d" || c == "'" => input.right = is_pressed,
					Character(c) if c == "q" || c == "o" => input.roll_left = is_pressed,
					Character(c) if c == "e" || c == "[" => input.roll_right = is_pressed,
					Character(c) if c == "r"             => input.reset_roll = is_pressed,
					// TODO: space, ctrl, alt => up/down
					Named(Shift) => input.is_fast_move = is_pressed,
					_ => {}
				}
				self.state.is_redraw_needed = true;
			}
			_ => {}
		}

	}

	fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
		let is_overlay = self.state.is_overlay();
		match event {
			DeviceEvent::MouseMotion { delta } if !is_overlay => {
				// dbg!(event);
				let input = &mut self.state.input;
				let (dx, dy) = (delta.0 as f32, delta.1 as f32);
				input.mouse_dx += dx;
				input.mouse_dy += dy;
				self.state.is_redraw_needed = true;
			}
			// TODO: use MouseWheel for FOV? or scrolling?
			_ => {}
		}
	}

	fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
		self.update();
		if self.state.is_redraw_needed {
			self.window.request_redraw();
			self.state.is_redraw_needed = self.state.input.is_redraw_needed(); // "hack" for "smooth" keyboard input (not as "text")
		}
	}

	fn update(&mut self) {
		let now = Instant::now();
		let dt = now.duration_since(self.state.last_update_inst).as_secs_f32();
		self.state.last_update_inst = now;

		let is_overlay = self.state.is_overlay();

		if !is_overlay {
			self.state.camera.update(&mut self.state.input, dt, &mut self.rng);
		}
		// dbg!(&self.camera);

		// let tick_frame_begin_timestamp = SystemTime::now(); // TODO?

		self.state.tick_n += 1;

		// physics update:
		if !self.state.is_paused /* TODO: && exist what needs to be updated */ {
			match self.state.dimension {
				Dimension::Base => {
					self.state.dim_base_la_for_floor_color.step(DIM_BASE_LA_SPEED);
					for (_x, _z, chunk) in self.state.chunks.iter_mut() {
						for (_pos, ro) in chunk.renderable_objects.iter_mut() {
							ro.update(dt);
						}
					}
					if self.state.camera.position.x < -CHUNK_SIZE_HALF {
						self.state.camera.position.x += CHUNK_SIZE;
						if !self.state.is_alt_topology {
							self.state.current_chunk_x = self.state.current_chunk_x.dec_mod(CHUNKS_N);
						} else {
							// camera.position.z = CHUNK_SIZE - camera.position.z; // TODO?
							let ccx: i32 = (self.state.current_chunk_x as i32) + if !self.state.is_x_flipped_global { -1 } else { 1 };
							if ccx < 0 || ccx >= CHUNKS_N as i32 {
								self.state.current_chunk_z = CHUNKS_N - self.state.current_chunk_z - 1;
								self.state.is_z_flipped_global = !self.state.is_z_flipped_global;
							}
							self.state.current_chunk_x = ccx.rem_euclid(CHUNKS_N as i32) as u32;
						}
					}
					else if self.state.camera.position.x > CHUNK_SIZE_HALF {
						self.state.camera.position.x -= CHUNK_SIZE;
						if !self.state.is_alt_topology {
							self.state.current_chunk_x = self.state.current_chunk_x.inc_mod(CHUNKS_N);
						} else {
							// camera.position.z = CHUNK_SIZE - camera.position.z; // TODO?
							let ccx: i32 = (self.state.current_chunk_x as i32) + if !self.state.is_x_flipped_global { 1 } else { -1 };
							if ccx < 0 || ccx >= CHUNKS_N as i32 {
								self.state.current_chunk_z = CHUNKS_N - self.state.current_chunk_z - 1;
								self.state.is_z_flipped_global = !self.state.is_z_flipped_global;
							}
							self.state.current_chunk_x = ccx.rem_euclid(CHUNKS_N as i32) as u32;
						}
					}
					if self.state.camera.position.z < -CHUNK_SIZE_HALF {
						self.state.camera.position.z += CHUNK_SIZE;
						if !self.state.is_alt_topology {
							self.state.current_chunk_z = self.state.current_chunk_z.dec_mod(CHUNKS_N);
						} else {
							// camera.position.x = CHUNK_SIZE - camera.position.x; // TODO?
							let ccz: i32 = (self.state.current_chunk_z as i32) + if !self.state.is_z_flipped_global { -1 } else { 1 };
							if ccz < 0 || ccz >= CHUNKS_N as i32 {
								self.state.current_chunk_x = CHUNKS_N - self.state.current_chunk_x - 1;
								self.state.is_x_flipped_global = !self.state.is_x_flipped_global;
							}
							self.state.current_chunk_z = ccz.rem_euclid(CHUNKS_N as i32) as u32;
						}
					}
					else if self.state.camera.position.z > CHUNK_SIZE_HALF {
						self.state.camera.position.z -= CHUNK_SIZE;
						if !self.state.is_alt_topology {
							self.state.current_chunk_z = self.state.current_chunk_z.inc_mod(CHUNKS_N);
						} else {
							// camera.position.x = CHUNK_SIZE - camera.position.x; // TODO?
							let ccz: i32 = (self.state.current_chunk_z as i32) + if !self.state.is_z_flipped_global { 1 } else { -1 };
							if ccz < 0 || ccz >= CHUNKS_N as i32 {
								self.state.current_chunk_x = CHUNKS_N - self.state.current_chunk_x - 1;
								self.state.is_x_flipped_global = !self.state.is_x_flipped_global;
							}
							self.state.current_chunk_z = ccz.rem_euclid(CHUNKS_N as i32) as u32;
						}
					}
					self.state.is_redraw_needed = true;
				}
				Dimension::SurfaceWorld => {}
			}
		}

		// 3. If anything moved, we need to draw again
		// (If it's a game, you usually just set this to true always)
		self.state.is_redraw_needed = true; // TODO
	}

	fn render(&mut self) {
		let vp = self.state.camera.proj_matrix() * self.state.camera.view_matrix();
		let uniforms = Uniforms { view_proj: vp.to_cols_array_2d() };
		self.renderer.queue.write_buffer(&self.renderer.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

		let frame = match self.renderer.surface.get_current_texture() {
			wgpu::CurrentSurfaceTexture::Success(frame) => frame,
			wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
				drop(frame);
				self.reconfigure_surface();
				return;
			}
			wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
				self.reconfigure_surface();
				return;
			}
			wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
				// skip this frame
				return;
			}
			wgpu::CurrentSurfaceTexture::Validation => {
				eprintln!("surface validation error");
				return;
			}
		};

		self.state.frame_n += 1;

		let mut all_points: Vec<(Vec3, ColorU8)> = vec![];
		let mut all_lines: Vec<((Vec3, ColorU8), (Vec3, ColorU8))> = vec![];
		// let mut all_chains: Vec<Vec<Vec3>> = vec![];
		let mut all_triangles: Vec<((Vec3, ColorU8), (Vec3, ColorU8), (Vec3, ColorU8))> = vec![];

		// canvas.set_draw_color(ColorU8::RGB(((frame_n) % 255) as u8, (((frame_n+64)/2) % 255) as u8, (255 - (frame_n/3) % 255) as u8));
		// canvas.set_draw_color(ColorU8::BLACK);
		// canvas.clear();

		// let (w, h) = canvas.window().size();
		let (w, h) = (self.renderer.config.width, self.renderer.config.height);
		let (wi, hi) = (w as i32, h as i32);
		let (wf, hf) = (w as float, h as float);
		let (wfh, hfh) = (wf / 2., hf / 2.);
		// let wh_ratio = wf / hf;
		// let hw_ratio = hf / wf;

		match self.state.dimension {
			Dimension::Base => {
				let iter = if !self.state.is_alt_topology {
					Either::Left(self.state.chunks.iter_around_wrapping(self.state.current_chunk_x as i32, self.state.current_chunk_z as i32, self.state.render_distance))
				} else {
					Either::Right(self.state.chunks.iter_around_wrapping_alt(self.state.current_chunk_x as i32, self.state.current_chunk_z as i32, self.state.render_distance))
				};
				for (dx, dz, _x, _z, _is_x_flipped_local, _is_z_flipped_local, chunk) in iter {
					const STEP: float = 1.;
					// let is_x_flipped = is_x_flipped_global ^ is_x_flipped_local;
					// let is_z_flipped = is_z_flipped_global ^ is_z_flipped_local;
					let mut x = -CHUNK_SIZE_HALF * (1. - 1e-2);
					while x < CHUNK_SIZE_HALF {
						let mut z = -CHUNK_SIZE_HALF * (1. - 1e-2);
						while z < CHUNK_SIZE_HALF {
							let pos = Vec3::new((dx as float)*CHUNK_SIZE + x, 0., (dz as float)*CHUNK_SIZE + z);
							let pos = pos.flip_x_if(self.state.is_x_flipped_global);
							let pos = pos.flip_z_if(self.state.is_z_flipped_global);
							// canvas.set_draw_color({
							let c = {
								// let mut c = base_color(&dim_base_la_for_floor_color);
								// let pos_rel_to_cam = pos - camera.position;
								// if is_darkness_at_base {
								// 	// TODO: better attenuation curve
								// 	c = ((c as float) / (1. + 2e-3*pos_rel_to_cam.norm2())) as u8;
								// }
								// ColorU8::RGB(c, c, c)
								chunk.color
							};
							let lines = [
								(Vec3::new(pos.x - STEP/3., 0., pos.z - STEP/3.),
								 Vec3::new(pos.x + STEP/3., 0., pos.z + STEP/3.)),
								(Vec3::new(pos.x - STEP/3., 0., pos.z + STEP/3.),
								 Vec3::new(pos.x + STEP/3., 0., pos.z - STEP/3.)),
							];
							for (a, b) in lines.into_iter() {
								// if let Some((a,b)) = self.state.camera.project_line(line, wf, hf) {
								// 	canvas.draw_line(a,b).unwrap();
								// }
								all_lines.push(((a, c), (b, c)));
							}
							z += STEP;
						}
						x += STEP;
					}
				}
				let iter = if !self.state.is_alt_topology {
					Either::Left(self.state.chunks.iter_around_wrapping(self.state.current_chunk_x as i32, self.state.current_chunk_z as i32, self.state.render_distance))
				} else {
					Either::Right(self.state.chunks.iter_around_wrapping_alt(self.state.current_chunk_x as i32, self.state.current_chunk_z as i32, self.state.render_distance))
				};
				for (dx, dz, _x, _z, is_x_flipped_local, is_z_flipped_local, chunk) in iter {
					let is_x_flipped = self.state.is_x_flipped_global ^ is_x_flipped_local;
					let is_z_flipped = self.state.is_z_flipped_global ^ is_z_flipped_local;
					for (pos, ro) in chunk.renderable_objects.iter() {
						use RenderableShape::*;
						let shift: Vec3 = pos.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) +
							Vec3::new((dx as float)*CHUNK_SIZE, 0., (dz as float)*CHUNK_SIZE)
								.flip_x_if(self.state.is_x_flipped_global).flip_z_if(self.state.is_z_flipped_global);
						// canvas.set_draw_color({
						let c = {
							let ColorU8 { mut r, mut g, mut b, .. } = chunk.color;
							if self.state.is_darkness_at_base {
								let pos_rel_to_cam = shift - self.state.camera.position;
								// TODO: better attenuation curve
								r = ((r as float) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
								g = ((g as float) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
								b = ((b as float) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
							}
							ColorU8::new(r, g, b)
						};
						for renderable_shape in ro.get_renderable_shapes(&self.state.camera) {
							// TODO(optim): do these computations on gpu?
							match renderable_shape {
								Points(points) => {
									all_points.extend(
										points.iter()
											.map(|&p| (p.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift, c))
									);
								}
								Lines(lines) => {
									for line in lines.iter() {
										let (a, b) = line;
										let a = a.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped);
										let b = b.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped);
										let a = a + shift;
										let b = b + shift;
										all_lines.push(((a, c), (b, c)));
									}
								}
								Chain(chain) => {
									for [a, b] in chain.array_windows() {
										all_lines.push((
											(a.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift, c),
											(b.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift, c)
										));
									}
								}
							}
						}
					}
				}
			}
			Dimension::SurfaceWorld => {
				todo!();
				// const MESH_SIZE: u32 = 30;
				// const MESH_STEP: float = 0.9;
				// const LODS: &[(u32, ColorU8)] = &[
				// 	(4, ColorU8::new(64, 64, 64)),
				// 	(2, ColorU8::GRAY),
				// 	(1, ColorU8::WHITE),
				// ];
				// let params = &self.state.surface_world_params;
				// fn surface_at(x: float, z: float, params: &[(f32, f32, f32, f32)]) -> float {
				// 	params.iter().map(|(amplitude, phase, cx, cz)| {
				// 		amplitude * sin(phase + cx*x + cz*z) / (params.len() as float)//.powf(*amplitude)
				// 	}).sum()
				// }
				// for (lod_n, lod_color) in LODS {
				// 	canvas.set_draw_color(*lod_color);
				// 	let mesh_step = MESH_STEP * (*lod_n as float);
				// 	let cx = self.state.camera.position.x - (MESH_SIZE as float - 1.) * mesh_step / 2.;
				// 	let cz = self.state.camera.position.z - (MESH_SIZE as float - 1.) * mesh_step / 2.;
				// 	let surface = Vec2D::from_fn(MESH_SIZE, MESH_SIZE, |x, z| {
				// 		let x = (x as float) * mesh_step;
				// 		let z = (z as float) * mesh_step;
				// 		surface_at(x + cx - cx.rem_euclid(mesh_step), z + cz - cz.rem_euclid(mesh_step), params)
				// 	});
				// 	let cx = cx - cx.rem_euclid(mesh_step);
				// 	let cz = cz - cz.rem_euclid(mesh_step);
				// 	// TODO(optim): use draw_lines/chain
				// 	// let mut lines_x = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
				// 	// let mut lines_z = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
				// 	for z in 0..MESH_SIZE-1 {
				// 		let zf = (z as float) * mesh_step;
				// 		for x in 0..MESH_SIZE-1 {
				// 			let xf = (x as float) * mesh_step;
				// 			let line1 = (Vec3::new(xf+cx, surface[(x,z)], zf+cz), Vec3::new(xf+cx+mesh_step, surface[(x+1,z)], zf+cz));
				// 			if let Some((a,b)) = camera.project_line(line1, wf, hf) {
				// 				canvas.draw_line(a,b).unwrap();
				// 			}
				// 			let line2 = (Vec3::new(xf+cx, surface[(x,z)], zf+cz), Vec3::new(xf+cx, surface[(x,z+1)], zf+cz+mesh_step));
				// 			if let Some((a,b)) = camera.project_line(line2, wf, hf) {
				// 				canvas.draw_line(a,b).unwrap();
				// 			}
				// 		}
				// 	}
				// 	for x in 0..MESH_SIZE-1 {
				// 		let z = MESH_SIZE-1;
				// 		let zf = (z as float) * mesh_step;
				// 		let xf = (x as float) * mesh_step;
				// 		let line1 = (Vec3::new(xf+cx, surface[(x,z)], zf+cz), Vec3::new(xf+cx+mesh_step, surface[(x+1,z)], zf+cz));
				// 		if let Some((a,b)) = camera.project_line(line1, wf, hf) {
				// 			canvas.draw_line(a,b).unwrap();
				// 		}
				// 	}
				// 	for z in 0..MESH_SIZE-1 {
				// 		let x = MESH_SIZE-1;
				// 		let xf = (x as float) * mesh_step;
				// 		let zf = (z as float) * mesh_step;
				// 		let line2 = (Vec3::new(xf+cx, surface[(x,z)], zf+cz), Vec3::new(xf+cx, surface[(x,z+1)], zf+cz+mesh_step));
				// 		if let Some((a,b)) = camera.project_line(line2, wf, hf) {
				// 			canvas.draw_line(a,b).unwrap();
				// 		}
				// 	}
				// }
			}
		}

		// if self.state.is_extra_info_shown {
		// 	let text_size = 3;
		// 	canvas.set_draw_color(ColorU8::GRAY);
		// 	let mut lines = vec![
		// 		format!("XYZ: {:.3}, {:.3}, {:.3}", self.state.camera.position.x, self.state.camera.position.y, self.state.camera.position.z),
		// 		format!("CHUNK XZ: {}, {}", self.state.current_chunk_x, self.state.current_chunk_z),
		// 		format!("MOVE TYPE: {}", self.state.movement_type.to_str_uppercase()),
		// 		format!("FOV: {:.3}", self.state.camera.fov_x.to_degrees()),
		// 		format!("TOPOLOGY IS ALT: {}", self.state.is_alt_topology.to_string().to_uppercase()),
		// 	];
		// 	if self.state.is_alt_topology {
		// 		lines.push(format!("is xz flipped global: {}, {}", self.state.is_x_flipped_global, self.state.is_z_flipped_global).to_uppercase());
		// 	}
		// 	for (i, line) in lines.iter().enumerate() {
		// 		canvas.render_text(line, (5, 5 + 35*(i as i32)), text_size);
		// 	}
		//
		// 	// // zqqx lang
		// 	// for char_n in 0..5 {
		// 	// 	let scale: u8 = 5;
		// 	// 	let zqqx_char: [i8; 25] = array::from_fn(|i| {
		// 	// 		let (i, j) = (i % 5, i / 5);
		// 	// 		let cx = char_n as float;
		// 	// 		let cy = ((i+j*5) as float).sqrt();
		// 	// 		// let cz = ((j+i*5) as float).ln_1p();
		// 	// 		let cz = (frame_n as float).ln_1p().ln_1p().ln_1p();
		// 	// 		let coefs = vec3![cx, cy, cz].normed();
		// 	// 		let t = lorenz_attractor.get_linear_combination(coefs.x, coefs.y, coefs.z);
		// 	// 		let t = t.rem_euclid(1.);
		// 	// 		(t * 255. - 128.) as i8
		// 	// 	});
		// 	// 	let bitmap = zqqx_lang.add_or_quantize(ZqqxChar::new(zqqx_char));
		// 	// 	buffer.render_custom_char(
		// 	// 		bitmap.quantize(),
		// 	// 		((buffer.w as i32) - 200 + (((char_n*7)*scale) as i32), 10),
		// 	// 		WHITE,
		// 	// 		scale,
		// 	// 	);
		// 	// }
		//
		// 	// TODO
		// 	// let frame_end_timestamp = SystemTime::now();
		// 	// let frametime = frame_end_timestamp.duration_since(tick_frame_begin_timestamp).unwrap();
		// 	// let fps = 1. / frametime.as_secs_f64();
		// 	// // if fps < 60. { panic!() }
		// 	// let fps_text = format!("\"FPS\": {fps:.1}");
		// 	// canvas.render_text(&fps_text, (wi - 5 - (fps_text.len() as i32) * (text_size as i32) * 6, 5), text_size);
		// }

		// TODO!
		// if self.state.is_help_opened {
		// 	const PADDING: float = 30.;
		// 	const ITEM_Y: float = 30.;
		// 	const ITEMS_N: u32 = 15;
		// 	debug_assert_eq!(1, ITEMS_N % 2);
		// 	const MENU_SIZE_X: float = 1000.;
		// 	const MENU_SIZE_Y: float = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as float);
		// 	const ITEM_X: float = MENU_SIZE_X - 2.*PADDING;
		// 	canvas.set_draw_color(ColorU8::BLACK);
		// 	canvas.fill_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	canvas.set_draw_color(ColorU8::WHITE);
		// 	canvas.draw_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY;
		// 	const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
		// 	// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
		// 	const ITEM_TEXT_SIZE: u8 = 5;
		// 	const ITEM_INNER_PADDING: float = (ITEM_Y - (ITEM_TEXT_SIZE as float)*(FONT_H as float)) / 2.;
		// 	canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 	let i_init: u32 = self.state.help_line_index.saturating_sub((ITEMS_N-1)/2);
		// 	let mut i: u32 = i_init;
		// 	while i - i_init < ITEMS_N && i < self.state.help_lines.len() as u32 {
		// 		let menu_item = &self.state.help_lines[i as usize];
		// 		if i == self.state.help_line_index {
		// 			canvas.set_draw_color(ITEM_SELECTED_COLOR);
		// 		}
		// 		let item_cx = wfh;
		// 		let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
		// 		// canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
		// 		canvas.render_text(menu_item, ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
		// 		if i == self.state.help_line_index {
		// 			canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 		}
		// 		i += 1;
		// 	}
		// }
		//
		// if self.state.is_inventory_opened {
		// 	const PADDING: float = 30.;
		// 	const ITEM_Y: float = 50.;
		// 	const ITEMS_N: u32 = 11;
		// 	debug_assert_eq!(1, ITEMS_N % 2);
		// 	const MENU_SIZE_X: float = 900.;
		// 	const MENU_SIZE_Y: float = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as float);
		// 	const ITEM_X: float = MENU_SIZE_X - 2.*PADDING;
		// 	canvas.set_draw_color(ColorU8::BLACK);
		// 	canvas.fill_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	canvas.set_draw_color(ColorU8::WHITE);
		// 	canvas.draw_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY;
		// 	const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
		// 	// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
		// 	const ITEM_TEXT_SIZE: u8 = 5;
		// 	const ITEM_INNER_PADDING: float = (ITEM_Y - (ITEM_TEXT_SIZE as float)*(FONT_H as float)) / 2.;
		// 	canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 	let i_init: u32 = self.state.inventory_item_index.saturating_sub((ITEMS_N-1)/2);
		// 	let mut i: u32 = i_init;
		// 	while i - i_init < ITEMS_N && i < self.state.inventory_items.len() as u32 {
		// 		let inventory_item = &self.state.inventory_items[i as usize];
		// 		if i == self.state.inventory_item_index {
		// 			canvas.set_draw_color(ITEM_SELECTED_COLOR);
		// 		}
		// 		let item_cx = wfh;
		// 		let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
		// 		canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
		// 		canvas.render_text(&inventory_item.to_string(), ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
		// 		if i == self.state.inventory_item_index {
		// 			canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 		}
		// 		i += 1;
		// 	}
		// }
		//
		// if self.state.is_paused {
		// 	const PADDING: float = 50.;
		// 	const ITEM_Y: float = 80.;
		// 	const ITEMS_N: u32 = 7;
		// 	debug_assert_eq!(1, ITEMS_N % 2);
		// 	const MENU_SIZE_X: float = 800.;
		// 	const MENU_SIZE_Y: float = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as float);
		// 	const ITEM_X: float = MENU_SIZE_X - 2.*PADDING;
		// 	canvas.set_draw_color(ColorU8::BLACK);
		// 	canvas.fill_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	canvas.set_draw_color(ColorU8::WHITE);
		// 	canvas.draw_rect(FRect::from_center_size(wfh, hfh, MENU_SIZE_X, MENU_SIZE_Y)).unwrap();
		// 	const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY;
		// 	const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
		// 	// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
		// 	const ITEM_TEXT_SIZE: u8 = 5;
		// 	const ITEM_INNER_PADDING: float = (ITEM_Y - (ITEM_TEXT_SIZE as float)*(FONT_H as float)) / 2.;
		// 	canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 	let i_init: u32 = self.state.pause_menu_item_index.saturating_sub((ITEMS_N-1)/2);
		// 	let mut i: u32 = i_init;
		// 	while i - i_init < ITEMS_N && i < self.state.pause_menu_items.len() as u32 {
		// 		let menu_item = &self.state.pause_menu_items[i as usize];
		// 		if i == self.state.pause_menu_item_index {
		// 			canvas.set_draw_color(ITEM_SELECTED_COLOR);
		// 		}
		// 		let item_cx = wfh;
		// 		let item_cy = hfh - MENU_SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as float);
		// 		canvas.draw_rect(FRect::from_center_size(item_cx, item_cy, ITEM_X, ITEM_Y)).unwrap();
		// 		canvas.render_text(menu_item.to_str(), ((item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32, (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32), ITEM_TEXT_SIZE);
		// 		if i == self.state.pause_menu_item_index {
		// 			canvas.set_draw_color(ITEM_UNSELECTED_COLOR);
		// 		}
		// 		i += 1;
		// 	}
		// }

		let mut encoder = self.renderer.device.create_command_encoder(&Default::default());
		let view = frame.texture.create_view(&Default::default());
		let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: None,
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				depth_slice: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
			multiview_mask: None,
		});

		let points: Vec<Vertex> = all_points.into_iter()
			.map(|(p, c)| Vertex { position: p.to_array(), color: c.to_array() })
			.collect();
		let lines: Vec<Vertex> = all_lines.into_iter()
			.flat_map(|((p1,c1), (p2,c2))| [
				Vertex { position: p1.to_array(), color: c1.to_array() },
				Vertex { position: p2.to_array(), color: c2.to_array() },
			])
			.collect();
		let triangles: Vec<Vertex> = all_triangles.into_iter()
			.flat_map(|((p1,c1), (p2,c2), (p3,c3))| [
				Vertex { position: p1.to_array(), color: c1.to_array() },
				Vertex { position: p2.to_array(), color: c2.to_array() },
				Vertex { position: p3.to_array(), color: c3.to_array() },
			])
			.collect();

		let counts = [
			points.len() as u32,
			lines.len() as u32,
			triangles.len() as u32,
		];

		let buffers = [
			self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&points),
				usage: wgpu::BufferUsages::VERTEX,
			}),
			self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&lines),
				usage: wgpu::BufferUsages::VERTEX,
			}),
			self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&triangles),
				usage: wgpu::BufferUsages::VERTEX,
			}),
		];

		pass.set_bind_group(0, &self.renderer.bind_group, &[]);
		for i in 0..self.renderer.pipelines.len() {
			if counts[i] > 0 {
				pass.set_pipeline(&self.renderer.pipelines[i]);
				pass.set_vertex_buffer(0, buffers[i].slice(..));
				pass.draw(0..counts[i], 0..1);
			}
		}

		drop(pass);
		let _ = self.renderer.queue.submit([encoder.finish()]);
		frame.present();
	}

	fn reconfigure_surface(&mut self) {
		let size = self.window.inner_size();
		let (w, h) = (size.width, size.height);
		if w == 0 || h == 0 {
			panic!("size = {size:?}");
			// return; // avoid invalid config
		}
		self.renderer.config.width = w;
		self.renderer.config.height = h;
		self.renderer.surface.configure(&self.renderer.device, &self.renderer.config);
		self.state.camera.aspect_ratio = (w as f32) / (h as f32);
		self.state.is_redraw_needed = true;
	}

}





#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}
impl Vertex {
	fn layout() -> wgpu::VertexBufferLayout<'static> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as _,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: 3*4, // 3 * size_of<f32>()
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}

struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	pipelines: [wgpu::RenderPipeline; 3],
	uniform_buffer: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
}
impl Renderer {
	fn new(window: &'static Window) -> Self {
		let instance = wgpu::Instance::default();
		let surface = instance.create_surface(window).unwrap();

		let adapter = block_on(instance.request_adapter(&Default::default())).unwrap();
		let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

		let caps = surface.get_capabilities(&adapter);
		let format = caps.formats[0];

		let size = window.inner_size();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format,
			width: size.width,
			height: size.height,
			present_mode: caps.present_modes[0],
			alpha_mode: caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};

		surface.configure(&device, &config);

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
		});

		let uniforms = Uniforms {
			view_proj: Mat4::IDENTITY.to_cols_array_2d(),
		};
		let uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Uniform Buffer"),
				contents: bytemuck::bytes_of(&uniforms),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			},
		);
		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: None,
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}
			],
		});
		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: uniform_buffer.as_entire_binding(),
				}
			],
			label: None,
		});

		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[Some(&bind_group_layout)],
			immediate_size: 0,
		});

		let make_pipeline = |topology| {
			device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
				label: None,
				layout: Some(&layout),
				vertex: wgpu::VertexState {
					module: &shader,
					entry_point: Some("vs_main"),
					buffers: &[Vertex::layout()],
					compilation_options: Default::default(),
				},
				primitive: wgpu::PrimitiveState {
					topology,
					..Default::default()
				},
				fragment: Some(wgpu::FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					targets: &[Some(wgpu::ColorTargetState {
						format,
						blend: Some(wgpu::BlendState::REPLACE),
						write_mask: wgpu::ColorWrites::ALL,
					})],
					compilation_options: Default::default(),
				}),
				depth_stencil: None,
				multisample: Default::default(),
				multiview_mask: None,
				cache: None,
			})
		};

		let pipelines = [
			make_pipeline(wgpu::PrimitiveTopology::PointList),
			make_pipeline(wgpu::PrimitiveTopology::LineList),
			make_pipeline(wgpu::PrimitiveTopology::TriangleList),
		];

		Self {
			surface,
			device,
			queue,
			config,
			pipelines,
			uniform_buffer,
			bind_group,
		}
	}
}

struct State {
	camera: Camera,
	input: InputState,
	last_update_inst: Instant,
	is_redraw_needed: bool = true,

	// TODO(refactor): extract
	help_lines: Vec<String>,
	is_help_opened: bool = false,
	help_line_index: u32 = 0,

	// TODO(refactor): extract
	pause_menu_items: Vec<PauseMenuItem>, // TODO: static array?
	is_paused: bool = false,
	pause_menu_item_index: u32 = 0,

	is_darkness_at_base: bool = false,

	dimension: Dimension = Dimension::Base,
	dim_base_la_for_floor_color: LorenzAttractor,

	// TODO(refactor): extract
	inventory_items: Vec<InventoryItem>,
	is_inventory_opened: bool = false,
	inventory_item_index: u32 = 0,

	surface_world_params: Vec<(float, float, float, float)>,

	chunks: Vec2D<Chunk>,
	render_distance: u32 = 2,
	current_chunk_x: u32 = 0,
	current_chunk_z: u32 = 0,
	is_alt_topology: bool = true, // FIXME: must be false
	is_x_flipped_global: bool = false, // for alt topology
	is_z_flipped_global: bool = false, // for alt topology

	movement_type: MovementType = MovementType::Grounded,

	tick_n: u64 = 0,
	frame_n: u64 = 0,
	is_extra_info_shown: bool = true,

	// zqqx_lang: ZqqxLang,
}

impl State {
	fn new(w: f32, h: f32, rng: &mut ThreadRng) -> Self {
		let last_update_inst = Instant::now();

		let dim_base_la_for_floor_color = LorenzAttractor::new()
			.offset_params_(Vec3::random_unit_cube(rng) * 0.1)
			.offset_xyz(30., 0., 0.);

		let inventory_items = Vec::with_capacity(100);

		let surface_world_params = gen_surface_world_params(rng);

		let chunks = Vec2D::<Chunk>::from_fn(CHUNKS_N, CHUNKS_N, |_x, _z| {
			Chunk::new_random(rng)
		});
		// println!("chunks.len = {}", chunks.iter().count());

		let camera = Camera::new(w / h);

		Self {
			last_update_inst,
			camera: Camera::new(w / h),
			input: InputState::default(),
			help_lines: [
				"controls:",
				"f1 - show help screen",
				"escape - pause",
				"wasd/arrows/pl;' - move",
				"shift - move fast",
				"space/ctrl/alt - fly up/down",
				"e/q - roll (only in fpv mode)",
				"tab/i - open inventory",
				"+- - change fov",
				"f3 - toggle info overlay",
				"f5 - change movement mode",
				// "f8 - stock market",
			].map(|s| s.to_uppercase()).to_vec(),
			pause_menu_items: { use PauseMenuItem::*; vec![
				Quit,
				Back,
				GetRandomItem,
				ToggleTopology,
				ToggleDarkness,
				ToggleUnlimitedFov,
				ToggleShakyFov,
			]},
			dim_base_la_for_floor_color,
			inventory_items,
			surface_world_params,
			chunks,
			..
		}
	}

	fn is_overlay(&self) -> bool {
		self.is_paused || self.is_inventory_opened || self.is_help_opened
	}
}



#[derive(Debug)]
struct Camera {
	position: Vec3,
	orientation: Quat,
	aspect_ratio: f32,
	fov_x: f32,
	near: f32,
	far: f32,
	movement_type: MovementType,
	is_unlimited_fov: bool,
	is_shaky_fov: bool,
}
impl Camera {
	const GROUNDED_CAMERA_Y: float = 1.5;

	fn new(aspect_ratio: f32) -> Self {
		Self {
			position: Vec3::new(0., Self::GROUNDED_CAMERA_Y, 0.),
			orientation: Quat::IDENTITY,
			aspect_ratio,
			fov_x: 100_f32.to_radians(),
			near: 0.1,
			far: 1000.,
			movement_type: MovementType::Grounded,
			is_unlimited_fov: false,
			is_shaky_fov: false,
		}
	}

	fn toggle_unlimited_fov(&mut self) {
		self.is_unlimited_fov = !self.is_unlimited_fov;
	}
	fn toggle_shaky_fov(&mut self) {
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
	/// returns (right, up, forward) vectors
	fn basis(&self) -> (Vec3, Vec3, Vec3) {
		(self.right(), self.up(), self.forward())
	}

	fn view_matrix(&self) -> Mat4 {
		Mat4::look_to_rh(
			self.position,
			self.forward(),
			self.up(),
		)
	}

	fn proj_matrix(&self) -> Mat4 {
		Mat4::perspective_rh(
			self.fov_y_from_x(),
			self.aspect_ratio,
			self.near,
			self.far,
		)
	}

	// TODO(optim): "cache" the value (store in self)
	fn fov_y_from_x(&self) -> f32 {
		2.0 * ((self.fov_x * 0.5).tan() / self.aspect_ratio).atan()
	}

	fn update(&mut self, input: &mut InputState, dt: f32, rng: &mut ThreadRng) {
		// TODO: first update position or orientation?
		self.update_position(input, dt);
		self.update_orientation(input, dt);
		self.update_fov(input, dt, rng);
	}

	fn update_position(&mut self, input: &InputState, dt: f32) {
		let speed = 0.5 * dt;

		let forward = self.forward();
		let right = forward.cross(Vec3::Y).normalize();

		// if input.forward {
		// 	self.position += forward * speed;
		// }
		// if input.back {
		// 	self.position -= forward * speed;
		// }
		// if input.left {
		// 	self.position -= right * speed;
		// }
		// if input.right {
		// 	self.position += right * speed;
		// }

		let mut move_speed: float = 20.;
		if input.is_fast_move {
			move_speed *= 3.;
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
		const SENSITIVITY: f32 = 0.02; // TODO: must be dependent on fov?
		const ROLL_SPEED: f32 = 1.;

		let yaw = input.mouse_dx * SENSITIVITY; // NOTE: "dt" is in mouse_dx

		const MAX_PITCH: f32 = FRAC_PI_2 * 0.99;
		let pitch_delta = input.mouse_dy * SENSITIVITY; // NOTE: "dt" is in mouse_dy
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
					roll += ROLL_SPEED * dt;
				}
				if input.roll_right {
					roll -= ROLL_SPEED * dt;
				}
				if input.reset_roll {
					roll = 0.;
				}
			}
		}

		let yaw_q = Quat::from_rotation_y(-yaw); // world-space yaw
		let pitch_q = Quat::from_axis_angle(self.right(), -clamped_pitch); // local-space pitch
		let roll_q = Quat::from_axis_angle(self.forward(), roll); // local-space roll
		self.orientation = yaw_q * pitch_q * roll_q * self.orientation;
		self.orientation = self.orientation.normalize();

		input.mouse_dx = 0.;
		input.mouse_dy = 0.;
	}

	fn update_fov(&mut self, input: &InputState, dt: f32, rng: &mut ThreadRng) {
		const FOV_MIN: float = 1e-1_f32.to_radians();
		const FOV_MAX: float = 170_f32.to_radians();
		const FOV_RANGE: float = FOV_MAX - FOV_MIN;
		const FOV_CHANGE_SPEED: float = 3.;

		if self.is_shaky_fov {
			self.fov_x = self.fov_x + rng.random_range(-0.1 ..= 0.1) * dt;
				// .clamp(1_f32.to_radians(), 170_f32.to_radians());
		}
		if !self.is_unlimited_fov {
			self.fov_x = self.fov_x.clamp(FOV_MIN*1.1, FOV_MAX/1.1);
		}

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
	}

	fn reset_roll(&mut self) {
		todo!()
	}

	fn reset_position(&mut self) {
		// const CAMERA_DEFAULT_POSITION: Vec3 = Vec3::ZERO.with_y(GROUNDED_CAMERA_Y);
		// state.camera.pos = CAMERA_DEFAULT_POSITION;
		todo!()
	}

	fn next_movement_type(&mut self) {
		self.movement_type.next();
		match self.movement_type { // #bqooaj
			MovementType::Grounded => {
				self.position.y = Self::GROUNDED_CAMERA_Y;
				self.reset_roll();
			}
			MovementType::FlyingMClike => {}
			MovementType::FlyingGMlike => {}
			MovementType::FpvLike => {}
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
	view_proj: [[f32; 4]; 4],
}

#[derive(Debug, Default)]
struct InputState {
	// "continuous":
	forward: bool,
	back: bool,
	left: bool,
	right: bool,
	up: bool,
	down: bool,
	roll_left: bool,
	roll_right: bool,
	reset_roll: bool,
	mouse_dx: f32,
	mouse_dy: f32,
	zoom_in: bool,
	zoom_out: bool,
	is_fast_move: bool,
	// "discrete":
	escape: bool,
	f1: bool,
}
impl InputState {
	fn is_redraw_needed(&self) -> bool { // TODO: rename?
		self.forward
		|| self.back
		|| self.left
		|| self.right
		|| self.up
		|| self.down
		|| self.roll_left
		|| self.roll_right
		|| self.zoom_in
		|| self.zoom_out
		// TODO: also include discrete inputs?
	}
}

const POINTS: &[Vertex] = &[
	Vertex { position: [0.9, 0., -3.], color: [1., 1., 1.] },
];

const LC: f32 = 0.0;
const HC: f32 = 1.0;
const LINES: &[Vertex] = &[
	Vertex { position: [-0.5, -0.5, -3.], color: [1., 0., 0.] },
	Vertex { position: [ 0.5, -0.5, -3.], color: [0., 1., 0.] },

	Vertex { position: [-5., -5., -5.], color: [LC,LC,LC] },
	Vertex { position: [-5., -5.,  5.], color: [LC,LC,HC] },

	Vertex { position: [-5., -5., -5.], color: [LC,LC,LC] },
	Vertex { position: [-5.,  5., -5.], color: [LC,HC,LC] },

	Vertex { position: [-5., -5., -5.], color: [LC,LC,LC] },
	Vertex { position: [ 5., -5., -5.], color: [HC,LC,LC] },

	Vertex { position: [ 5.,  5.,  5.], color: [HC,HC,HC] },
	Vertex { position: [ 5.,  5., -5.], color: [HC,HC,LC] },

	Vertex { position: [ 5.,  5.,  5.], color: [HC,HC,HC] },
	Vertex { position: [ 5., -5.,  5.], color: [HC,LC,HC] },

	Vertex { position: [ 5.,  5.,  5.], color: [HC,HC,HC] },
	Vertex { position: [-5.,  5.,  5.], color: [LC,HC,HC] },

	Vertex { position: [-5., -5.,  5.], color: [LC,LC,HC] },
	Vertex { position: [-5.,  5.,  5.], color: [LC,HC,HC] },

	Vertex { position: [-5., -5.,  5.], color: [LC,LC,HC] },
	Vertex { position: [ 5., -5.,  5.], color: [HC,LC,HC] },

	Vertex { position: [-5.,  5., -5.], color: [LC,HC,LC] },
	Vertex { position: [-5.,  5.,  5.], color: [LC,HC,HC] },

	Vertex { position: [-5.,  5., -5.], color: [LC,HC,LC] },
	Vertex { position: [ 5.,  5., -5.], color: [HC,HC,LC] },

	Vertex { position: [ 5., -5., -5.], color: [HC,LC,LC] },
	Vertex { position: [ 5., -5.,  5.], color: [HC,LC,HC] },

	Vertex { position: [ 5., -5., -5.], color: [HC,LC,LC] },
	Vertex { position: [ 5.,  5., -5.], color: [HC,HC,LC] },
];

const TRIANGLES: &[Vertex] = &[
	Vertex { position: [ 0.0,  0.5, -3.], color: [1., 0., 0.] },
	Vertex { position: [-0.5, -0.2, -3.], color: [0., 1., 0.] },
	Vertex { position: [ 0.5, -0.2, -3.], color: [0., 0., 1.] },

	Vertex { position: [ 0.0,  0.5,  3.], color: [1., 0., 0.] },
	Vertex { position: [-0.5, -0.2,  3.], color: [0., 1., 0.] },
	Vertex { position: [ 0.5, -0.2,  3.], color: [0., 0., 1.] },

	Vertex { position: [-3.,  0.5,  0.0], color: [1., 0., 0.] },
	Vertex { position: [-3., -0.2, -0.5], color: [0., 1., 0.] },
	Vertex { position: [-3., -0.2,  0.5], color: [0., 0., 1.] },

	Vertex { position: [ 3.,  0.5,  0.0], color: [1., 0., 0.] },
	Vertex { position: [ 3., -0.2, -0.5], color: [0., 1., 0.] },
	Vertex { position: [ 3., -0.2,  0.5], color: [0., 0., 1.] },

	Vertex { position: [ 0.5,  3.,  0.0], color: [1., 0., 0.] },
	Vertex { position: [-0.2,  3., -0.5], color: [0., 1., 0.] },
	Vertex { position: [-0.2,  3.,  0.5], color: [0., 0., 1.] },

	Vertex { position: [ 0.5, -3.,  0.0], color: [1., 0., 0.] },
	Vertex { position: [-0.2, -3., -0.5], color: [0., 1., 0.] },
	Vertex { position: [-0.2, -3.,  0.5], color: [0., 0., 1.] },
];



const DIM_BASE_LA_SPEED: float = 1e-5;

fn base_color(la: &LorenzAttractor) -> u8 {
	let x = la.get_linear_combination(1., 1., 1.);
	let c = x.clamp(1., 80.) as u8;
	c
}

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

fn main_loop_old() {

	// let tick_end_timestamp = SystemTime::now();
	// let ticktime = tick_end_timestamp.duration_since(tick_frame_begin_timestamp).unwrap();
	// let target_fps = 60;
	// if ticktime < Duration::new(0, 1_000_000_000u32 / target_fps) {
	// 	sleep(Duration::new(0, 1_000_000_000u32 / target_fps) - ticktime);
	// }
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
	RenderableObject_(RenderableObject),
	Text(String), // just for test
}
impl InventoryItem {
	fn new_random(rng: &mut ThreadRng) -> Self {
		use InventoryItem::*;
		match_random_weighted! { rng,
			0.1 => SurfaceWorld,
			1. => RenderableObject_(RenderableObject::new_random(rng)),
		}
	}
}
impl ToString for InventoryItem {
	fn to_string(&self) -> String {
		use InventoryItem::*;
		match self {
			SurfaceWorld => "SURFACE WORLD".to_string(),
			RenderableObject_(ro) => ro.to_string(),
			Text(text) => text.clone(),
		}
	}
}





enum PauseMenuItem {
	Quit,
	Back,
	GetRandomItem,
	ToggleTopology,
	ToggleDarkness,
	ToggleUnlimitedFov,
	ToggleShakyFov,
	Text(String), // just for test
}
impl PauseMenuItem {
	fn to_str(&self) -> &str {
		use PauseMenuItem::*;
		match self {
			Quit => "QUIT",
			Back => "BACK",
			GetRandomItem => "GET RANDOM ITEM",
			ToggleTopology => "TOGGLE TOPOLOGY",
			ToggleDarkness => "TOGGLE DARKNESS",
			ToggleUnlimitedFov => "TOGGLE UNLIMITED FOV",
			ToggleShakyFov => "TOGGLE SHAKY FOV",
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





#[derive(Debug, Clone)]
enum RenderableObject {
	Cube { size: float },
	LorenzAttractor { size: float, la: LorenzAttractor, last_points: Vec<Vec3>, max_len: u32 },
	// SpinningText?
	Monolith { sizes: Vec<float> },
	RotatingSimplex { initpoints_rotplanes_rotvels_phases: Vec<(Vec3, Vec3, float, float)> },
	RotatingIcosahedron { size: float, global_rotvel: float, rotplanes_rotvels_angles: Vec<(Vec3, float, float)> },
	Kitty { size: float, rotvel: float, phase: float },
	Graph3d { connect_n: u32, global_rotvel: float, initpoints_rotplanes_rotvels_phases: Vec<(Vec3, Vec3, float, float)> },
	// TravelingSalesmanProblemSolver in realtime
}
impl RenderableObject {
	fn new_random(rng: &mut ThreadRng) -> Self {
		match_random_weighted! { rng,
			0.1 => RenderableObject::Cube {
				size: rng.random_range(0.3 ..= 3.),
			},
			0.3 => RenderableObject::LorenzAttractor {
				size: rng.random_range(0.1 ..= 0.2),
				la: LorenzAttractor::new().offset_params_(
					Vec3::random_unit_cube(rng) * 0.1
				).set_xyz_(
					Vec3::random_unit(rng) * rng.random_range(0.1 ..= 0.2)
				),
				last_points: vec![],
				max_len: 10_f32.powf(rng.random_range(2. ..= 4.)).round() as u32,
			},
			0.1 => RenderableObject::Monolith {
				sizes: Vec::from_fn(
					rng.random_range(5 ..= 20),
					|_i| rng.random_range(0.5 ..= 2.7_f32).powi(2)
				),
			},
			1. => RenderableObject::RotatingSimplex {
				initpoints_rotplanes_rotvels_phases: {
					macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
					let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
					let n = rng.random_range(4 ..= 10);
					(0..n).map(|_i| (
						Vec3::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
						Vec3::random_unit(rng),
						rng.random_range(0.5 ..= 1.4_f32).powi(2),
						rng.random_range(0. ..= TAU),
					)).collect()
				},
			},
			1. => RenderableObject::RotatingIcosahedron {
				size: rng.random_range(0.5 ..= 2.5),
				global_rotvel: rng.random_range(0.01 ..= 1.),
				rotplanes_rotvels_angles: Vec::from_fn(
					rng.random_range(1 ..= 5),
					|_i| (
						Vec3::random_unit(rng),
						rng.random_range(0.1 ..= 2.),
						rng.random_range(0. ..= TAU),
					)
				),
			},
			1e-2 => RenderableObject::Kitty {
				size: rng.random_range(1. ..= 1.5),
				rotvel: rng.random_range(5. ..= 15.),
				phase: 0.,
			},
			0.2 => RenderableObject::Graph3d {
				// connect_n: rng.random_range(1 ..= 6),
				connect_n: 6, // FIXME
				global_rotvel: rng.random_range(0.01 ..= 2.),
				initpoints_rotplanes_rotvels_phases: {
					macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
					let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
					// let n = rng.random_range(10 ..= 200);
					let n = 200; // FIXME
					(0..n).map(|_i| (
						Vec3::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
						Vec3::random_unit(rng),
						rng.random_range(0.5 ..= 1.4_f32).powi(2),
						rng.random_range(0. ..= TAU),
					)).collect()
				},
			},
		}
	}
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
	fn get_renderable_shapes(&self, camera: &Camera) -> Vec<RenderableShape> {
		use RenderableObject::*;
		use RenderableShape::*;
		match self {
			Cube { size } => {
				let s = size / 2.;
				// TODO(optim): use Chain
				vec![Lines(vec![
					(Vec3::new(-s,-s,-s), Vec3::new(-s,-s, s)),
					(Vec3::new(-s,-s,-s), Vec3::new(-s, s,-s)),
					(Vec3::new(-s, s, s), Vec3::new(-s,-s, s)),
					(Vec3::new(-s, s, s), Vec3::new(-s, s,-s)),
					//
					(Vec3::new( s,-s,-s), Vec3::new( s,-s, s)),
					(Vec3::new( s,-s,-s), Vec3::new( s, s,-s)),
					(Vec3::new( s, s, s), Vec3::new( s,-s, s)),
					(Vec3::new( s, s, s), Vec3::new( s, s,-s)),
					//
					(Vec3::new(-s,-s,-s), Vec3::new( s,-s,-s)),
					(Vec3::new( s, s, s), Vec3::new(-s, s, s)),
					(Vec3::new(-s,-s, s), Vec3::new( s,-s, s)),
					(Vec3::new(-s, s,-s), Vec3::new( s, s,-s)),
				])]
			}
			LorenzAttractor { size, last_points, .. } => {
				vec![Chain(last_points.iter().map(|&p| p * *size).collect())]
			}
			Monolith { sizes } => {
				vec![Lines(sizes.iter().map(|size| {
					let s = size / 2.;
					vec![
						(Vec3::new(-s,-s,-s), Vec3::new(-s,-s, s)),
						(Vec3::new(-s,-s,-s), Vec3::new(-s, s,-s)),
						(Vec3::new(-s, s, s), Vec3::new(-s,-s, s)),
						(Vec3::new(-s, s, s), Vec3::new(-s, s,-s)),
						//
						(Vec3::new( s,-s,-s), Vec3::new( s,-s, s)),
						(Vec3::new( s,-s,-s), Vec3::new( s, s,-s)),
						(Vec3::new( s, s, s), Vec3::new( s,-s, s)),
						(Vec3::new( s, s, s), Vec3::new( s, s,-s)),
					]
				}).flatten().collect())]
			}
			RotatingSimplex { initpoints_rotplanes_rotvels_phases } => {
				let points: Vec<Vec3> = initpoints_rotplanes_rotvels_phases.iter()
					.map(|(initpoint, rotplane, _rotvel, phase)| {
						initpoint.rotate_axis(*rotplane, *phase)
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
					Vec3::new(-PHI, 0., -1.),
					Vec3::new(-PHI, 0.,  1.),
					Vec3::new( PHI, 0., -1.),
					Vec3::new( PHI, 0.,  1.),
					Vec3::new(-1., -PHI, 0.),
					Vec3::new(-1.,  PHI, 0.),
					Vec3::new( 1., -PHI, 0.),
					Vec3::new( 1.,  PHI, 0.),
					Vec3::new(0., -1., -PHI),
					Vec3::new(0., -1.,  PHI),
					Vec3::new(0.,  1., -PHI),
					Vec3::new(0.,  1.,  PHI),
				].map(|v| v * *size);
				for (rotplane, _rotvel, angle) in rotplanes_rotvels_angles.iter() {
					for vertex in vertices.iter_mut() {
						*vertex = vertex.rotate_axis(*rotplane, *angle);
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
				// TODO(fix): wrong in alt topology with is_x_flipped/is_z_flipped
				let angles_of_points_on_circle_20: Vec<float> = {
					const N: u32 = 20;
					let tau_div_n = TAU / (N as float);
					Vec::from_fn(N as usize, |i| (i as float) * tau_div_n)
				};
				let (cam_r, cam_u, cam_f) = camera.basis();
				let points_outline: Vec<Vec3> = angles_of_points_on_circle_20.iter()
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
								cam_r.rotate_axis(cam_f, *angle) * size
								+ cam_r.rotate_axis(cam_f, *phase) * 0.2
							)
						}
					})
					.collect();
				let angles_of_points_on_circle_10: Vec<float> = {
					const N: u32 = 10;
					let tau_div_n = TAU / (N as float);
					Vec::from_fn(N as usize, |i| (i as float) * tau_div_n)
				};
				let points_eye_left: Vec<Vec3> = angles_of_points_on_circle_10.iter()
					.chain(std::iter::once(angles_of_points_on_circle_10.first().unwrap()))
					.map(|angle| {
						cam_r.rotate_axis(cam_f, *angle) * 0.1
						+ cam_r.rotate_axis(cam_f, *phase) * 0.2
						+ cam_r * 0.5 + cam_u * 0.2
						+ cam_f * 0.05
					})
					.collect();
				let points_eye_right: Vec<Vec3> = angles_of_points_on_circle_10.iter()
					.chain(std::iter::once(angles_of_points_on_circle_10.first().unwrap()))
					.map(|angle| {
						cam_r.rotate_axis(cam_f, *angle) * 0.1
						+ cam_r.rotate_axis(cam_f, *phase) * 0.2
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
				let points_smile: Vec<Vec3> = points_smile.into_iter()
					.map(|(x, y)| {
						// cam_r.rotate_around(cam_f, *angle) * 0.1
						cam_r.rotate_axis(cam_f, *phase) * 0.2
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
				let points: Vec<Vec3> = initpoints_rotplanes_rotvels_phases.iter()
					.map(|(initpoint, rotplane, _rotvel, phase)| {
						initpoint.rotate_axis(*rotplane, *phase)
					})
					.collect();
				let mut neighbors: Vec<Vec<u32>> = Vec::from_fn(points.len(), |_i| Vec::with_capacity(points.len()));
				for i in 0 .. points.len() {
					let mut distances = vec![];
					for j in 0 .. points.len() { // or from i+1 ?
						let dist2 = if i != j { points[i].distance_squared(points[j]) } else { float::MAX };
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
impl ToString for RenderableObject {
	#[allow(unused_variables)]
	fn to_string(&self) -> String {
		use RenderableObject::*;
		match self {
			Cube { size } => format!("cube (size={size:.2})"),
			LorenzAttractor { size, la, last_points, max_len } => format!("lorenz attractor (size={size:.2})"),
			Monolith { sizes } => format!("monolith"),
			RotatingSimplex { initpoints_rotplanes_rotvels_phases } => format!("rotating simplex ({n} points)", n=initpoints_rotplanes_rotvels_phases.len()),
			RotatingIcosahedron { size, global_rotvel, rotplanes_rotvels_angles } => format!("rotating icosahedron ({n} rotation vectors)", n=rotplanes_rotvels_angles.len()),
			Kitty { size, rotvel, phase } => format!("kitty (size={size:.2})"),
			Graph3d { connect_n, global_rotvel, initpoints_rotplanes_rotvels_phases } => format!("graph 3d ({n} points, {nc} connect)", n=initpoints_rotplanes_rotvels_phases.len(), nc=connect_n),
		}.to_uppercase()
	}
}





#[derive(Debug)]
enum RenderableShape {
	Points(Vec<Vec3>),
	Lines(Vec<(Vec3, Vec3)>),
	Chain(Vec<Vec3>), // TODO(rename): LineStrip
}





const CHUNKS_N: u32 = 17;
const CHUNK_SIZE: float = 20.;
const CHUNK_SIZE_HALF: float = CHUNK_SIZE / 2.;
struct Chunk {
	color: ColorU8,
	renderable_objects: Vec<(Vec3, RenderableObject)>,
}
impl Chunk {
	fn new_random(rng: &mut ThreadRng) -> Self {
		Self {
			// color: Color::RGB(255/(CHUNKS_N as u8)*(1 + x as u8), 255/(CHUNKS_N as u8)*(1 + z as u8), 0), // for dbg
			color: ColorU8::new(rng.random(), rng.random(), rng.random()),
			renderable_objects: {
				match_random_weighted! { rng,
					1. => vec![], // empty / void / nothing
					1. => vec![(
						Vec3::from_y(rng.random_range(1. ..= 5.)),
						RenderableObject::new_random(rng),
					)],
					0.5 => Vec::from_fn(
						rng.random_range(0. ..= 4_f32).powi(2).round() as usize,
						|_i| (
							Vec3::new(
								rng.random_range(-CHUNK_SIZE_HALF ..= CHUNK_SIZE_HALF),
								rng.random_range(1. ..= 9.),
								rng.random_range(-CHUNK_SIZE_HALF ..= CHUNK_SIZE_HALF),
							),
							RenderableObject::Cube { size: rng.random_range(0.3 ..= 3.) }
						)
					),
				}
			}
		}
	}
}





// #[derive(Debug)]
// struct Camera {
// 	pos: Vec3f,
// 	forward: Vec3f,
// 	up: Vec3f,
// 	fov: float, // in radians
// }
// const NEAR: float = 0.1;
// impl Camera {
// 	fn reset_roll(&mut self) {
// 		self.up = vec3y!(1.);
// 	}
//
// 	/// returns (right, up, forward) vectors
// 	fn basis(&self) -> (Vec3f, Vec3f, Vec3f) {
// 		let f = self.forward.normed();
// 		let r = f.cross(self.up).normed();
// 		let u = r.cross(f);
// 		(r, u, f)
// 	}
//
// 	fn project_line(
// 		&self,
// 		line: (Vec3f, Vec3f),
// 		width: float,
// 		height: float,
// 		// near: float,
// 	) -> Option<(Vec2f, Vec2f)> {
// 		let (a, b) = self.clip_line_near(line.0, line.1, NEAR)?;
// 		let pa = self.project_point(a, width, height)?;
// 		let pb = self.project_point(b, width, height)?;
// 		clip_line_viewport(pa, pb, width, height)
// 	}
//
// 	fn clip_line_near(&self, a: Vec3f, b: Vec3f, near: float) -> Option<(Vec3f, Vec3f)> {
// 		let (_, _, forward) = self.basis();
// 		let da = (a - self.pos) * forward;
// 		let db = (b - self.pos) * forward;
// 		if da >= near && db >= near { return Some((a, b)); }
// 		if da < near && db < near { return None; }
// 		let t = (near - da) / (db - da);
// 		let intersect = Vec3f::new(
// 			a.x + (b.x - a.x) * t,
// 			a.y + (b.y - a.y) * t,
// 			a.z + (b.z - a.z) * t,
// 		);
// 		if da < near {
// 			Some((intersect, b))
// 		} else {
// 			Some((a, intersect))
// 		}
// 	}
//
// 	fn project_point(&self, p: Vec3f, width: float, height: float) -> Option<Vec2f> {
// 		let (right, up, forward) = self.basis();
//
// 		// world -> camera space
// 		let rel = p - self.pos;
//
// 		let x = rel * right;
// 		let y = rel * up;
// 		let z = rel * forward;
//
// 		// behind camera
// 		if z <= 0. { return None; }
//
// 		let aspect = width / height;
// 		let f = 1. / tan(self.fov * 0.5);
//
// 		// perspective projection (NDC)
// 		let nx = (x / z) * f / aspect;
// 		let ny = (y / z) * f;
//
// 		// to screen pixels
// 		let sx = (nx + 1.) * 0.5 * width;
// 		let sy = (1. - (ny + 1.) * 0.5) * height;
//
// 		Some(Vec2f { x: sx, y: sy })
// 	}
// }
//
// // TODO: rename
// const _INSIDE: u8 = 0;
// const _LEFT: u8 = 1;
// const _RIGHT: u8 = 2;
// const _BOTTOM: u8 = 4;
// const _TOP: u8 = 8;
//
// fn clip_line_viewport(mut a: Vec2f, mut b: Vec2f, w: float, h: float) -> Option<(Vec2f, Vec2f)> {
// 	let mut out_a = compute_outcode(a, w, h);
// 	let mut out_b = compute_outcode(b, w, h);
// 	loop {
// 		if (out_a | out_b) == 0 { return Some((a, b)); }
// 		if (out_a & out_b) != 0 { return None; }
// 		let out = if out_a != 0 { out_a } else { out_b };
// 		let mut x = 0.;
// 		let mut y = 0.;
// 		if (out & _TOP) != 0 {
// 			x = a.x + (b.x - a.x) * (0. - a.y) / (b.y - a.y);
// 			y = 0.;
// 		} else if (out & _BOTTOM) != 0 {
// 			x = a.x + (b.x - a.x) * (h - a.y) / (b.y - a.y);
// 			y = h;
// 		} else if (out & _RIGHT) != 0 {
// 			y = a.y + (b.y - a.y) * (w - a.x) / (b.x - a.x);
// 			x = w;
// 		} else if (out & _LEFT) != 0 {
// 			y = a.y + (b.y - a.y) * (0. - a.x) / (b.x - a.x);
// 			x = 0.;
// 		}
// 		if out == out_a {
// 			a = Vec2f::new(x, y);
// 			out_a = compute_outcode(a, w, h);
// 		} else {
// 			b = Vec2f::new(x, y);
// 			out_b = compute_outcode(b, w, h);
// 		}
// 	}
// }
//
// fn compute_outcode(p: Vec2f, w: float, h: float) -> u8 {
// 	let mut code = _INSIDE;
// 	if p.x < 0. { code |= _LEFT; } else if p.x > w { code |= _RIGHT; }
// 	if p.y < 0. { code |= _TOP; } else if p.y > h { code |= _BOTTOM; }
// 	code
// }





// trait SdlFRectFromCenterSize {
// 	fn from_center_size(cx: float, cy: float, sx: float, sy: float) -> Self;
// 	fn from_center_size_(c: impl Into<Vec2f>, s: impl Into<Vec2f>) -> Self;
// }
// impl SdlFRectFromCenterSize for FRect {
// 	fn from_center_size(cx: float, cy: float, sx: float, sy: float) -> Self {
// 		Self {
// 			x: cx - sx/2.,
// 			y: cy - sy/2.,
// 			w: sx,
// 			h: sy,
// 		}
// 	}
// 	fn from_center_size_(c: impl Into<Vec2f>, s: impl Into<Vec2f>) -> Self {
// 		let c = c.into();
// 		let s = s.into();
// 		Self::from_center_size(c.x, c.y, s.x, s.y)
// 	}
// }



// trait SdlKeyboardExtIsScancodesPressed {
// 	fn is_scancodes_pressed_any(&self, scancodes: &[Scancode]) -> bool;
// 	fn is_scancodes_pressed_all(&self, scancodes: &[Scancode]) -> bool;
// }
// impl SdlKeyboardExtIsScancodesPressed for KeyboardState<'_> {
// 	fn is_scancodes_pressed_any(&self, scancodes: &[Scancode]) -> bool {
// 		for scancode in scancodes {
// 			if self.is_scancode_pressed(*scancode) {
// 				return true
// 			}
// 		}
// 		false
// 	}
// 	fn is_scancodes_pressed_all(&self, scancodes: &[Scancode]) -> bool {
// 		for scancode in scancodes {
// 			if !self.is_scancode_pressed(*scancode) {
// 				return false
// 			}
// 		}
// 		true
// 	}
// }


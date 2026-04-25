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
	unused_variables, // FIXME: ENABLE ME
)]

#![feature(
	default_field_values,
	generic_const_exprs,
	vec_from_fn,
)]

use std::{f32::consts::{FRAC_PI_2, GOLDEN_RATIO, PI, TAU}, time::Instant};

use either::Either;
use rand::{RngExt, rng, rngs::ThreadRng};
use pollster::block_on;
use wgpu::util::DeviceExt;
use winit::{event::{DeviceEvent, ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, window::Window};
use glam::{Mat4, Quat, Vec2, Vec3};

mod color_u8;
mod consts;
mod extensions;
mod font_rendering;
mod lorenz_attractor;
mod math;
mod math_aliases;
mod renderable_shapes;
mod renderable_shapes_2d;
mod renderable_shapes_3d;
mod typesafe_rng;
mod utils_io;
mod vec2D;
mod vec2_ext;
mod vec3_ext;
mod zqqx_lang;

use color_u8::*;
use consts::*;
use extensions::*;
use font_rendering::*;
use lorenz_attractor::*;
use math::*;
use math_aliases::*;
use renderable_shapes::*;
use renderable_shapes_2d::*;
use renderable_shapes_3d::*;
use typesafe_rng::*;
// use utils_io::*;
use vec2D::*;
use vec2_ext::*;
use vec3_ext::*;
// use zqqx_lang::*;


// TODO(refactor): use .. instead of ..= for floats


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
	state: GameState,
	rng: ThreadRng,
}
impl App {
	fn new(window: &'static Window, renderer: Renderer) -> Self {
		let mut rng = rng();
		let state = GameState::new(renderer.config.width as f32, renderer.config.height as f32, &mut rng);
		Self {
			window,
			renderer,
			state,
			rng,
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: winit::window::WindowId, event: WindowEvent) {
		use winit::keyboard::{Key::*, KeyCode, NamedKey::*, PhysicalKey, SmolStr};
		use WindowEvent::*;
		let is_overlay = self.state.is_overlay();
		fn ss(s: &str) -> SmolStr { s.into() }
		match event {
			CloseRequested => {
				event_loop.exit();
			}
			RedrawRequested => {
				self.render();
			}
			Resized(_new_size) => {
				self.reconfigure_surface();
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(Escape), state: ElementState::Pressed, repeat: false, .. }, .. } => {
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
			KeyboardInput { event: KeyEvent { logical_key: Named(F1), state: ElementState::Pressed, repeat: false, .. }, .. } if !self.state.is_paused && !self.state.is_inventory_opened => {
				self.state.is_help_opened = !self.state.is_help_opened;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(F3), state: ElementState::Pressed, repeat: false, .. }, .. } if !is_overlay => {
				self.state.is_extra_info_shown = !self.state.is_extra_info_shown;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(F5), state: ElementState::Pressed, repeat: false, .. }, .. } if !is_overlay => {
				self.state.camera.next_movement_type();
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key, state: ElementState::Pressed, repeat: false, .. }, .. } if (logical_key == Character(ss("i")) || logical_key == Named(Tab)) && !self.state.is_paused && !self.state.is_help_opened => {
				self.state.is_inventory_opened = !self.state.is_inventory_opened;
				self.state.is_redraw_needed = true;
			}
			KeyboardInput { event: KeyEvent { logical_key: Named(ArrowUp), state: ElementState::Pressed, .. }, .. } if is_overlay => {
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
			KeyboardInput { event: KeyEvent { logical_key: Named(ArrowDown), state: ElementState::Pressed, .. }, .. } if is_overlay => {
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
			KeyboardInput { event: KeyEvent { logical_key: Named(Enter), state: ElementState::Pressed, repeat: false, .. }, .. } if is_overlay => {
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
						GetRandomItems => {
							for _ in 0..100 {
								self.state.inventory_items.push(InventoryItem::new_random(&mut self.rng));
							}
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
						ToggleVsync => {
							use wgpu::PresentMode::*;
							let present_mode = &mut self.renderer.config.present_mode;
							*present_mode = match present_mode {
								AutoVsync | Fifo | FifoRelaxed => { AutoNoVsync }
								AutoNoVsync | Immediate | Mailbox => { AutoVsync }
							};
							self.reconfigure_surface();
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
				// dbg!(&event);
				let is_pressed = event.state == ElementState::Pressed;
				// dbg!(is_pressed);
				let input = &mut self.state.input;
				// TODO: use only one type of keys: physical or logical?
				match event.logical_key {
					Character(c) if c == "w" || c == "W" || c == "p" || c == "P" => input.forward = is_pressed,
					Character(c) if c == "s" || c == "S" || c == ";" || c == ":" => input.back = is_pressed,
					Character(c) if c == "a" || c == "A" || c == "l" || c == "L" => input.left = is_pressed,
					Character(c) if c == "d" || c == "D" || c == "'" || c == "\""=> input.right = is_pressed,
					Character(c) if c == "q" || c == "Q" || c == "o" || c == "O" => input.roll_left = is_pressed,
					Character(c) if c == "e" || c == "E" || c == "[" || c == "{" => input.roll_right = is_pressed,
					Character(c) if c == "r" || c == "R"                         => input.reset_roll = is_pressed,
					Named(Space)         => input.up = is_pressed,
					Named(Control | Alt) => input.down = is_pressed,
					Character(c) if c == "=" || c == "+" => input.zoom_in = is_pressed,
					Character(c) if c == "-" || c == "_" => input.zoom_out = is_pressed,
					_ => {}
				}
				// TODO: dont use physical keys?
				match event.physical_key {
					PhysicalKey::Code(KeyCode::ShiftLeft | KeyCode::ShiftRight) => input.is_fast_move = is_pressed,
					_ => {}
				}
				// dbg!(input);
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
		// TODO!(fix): prevent huge dt (after pause)
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
					self.state.dim_base_la_for_floor_color.step(DIM_BASE_LA_SPEED * dt);
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

		// self.state.is_redraw_needed = true; // TODO: render always?
	}

	// TODO(refactor): split into `render_3d` and `render_2d`?
	fn render(&mut self) {
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

		let mut all_3d_points: Vec<Point3d> = vec![];
		let mut all_3d_lines: Vec<Line3d> = vec![];
		let mut all_3d_lines_oc: Vec<Line3dOC> = vec![];
		let mut all_3d_triangles: Vec<Triangle3d> = vec![];
		let mut all_3d_quads: Vec<Quad3d> = vec![];
		let mut all_3d_quads_oc: Vec<Quad3dOC> = vec![];

		let mut all_2d_points: Vec<Point2d> = vec![];
		let mut all_2d_lines: Vec<Line2d> = vec![];
		let mut all_2d_triangles: Vec<Triangle2d> = vec![];
		let mut all_2d_rect_filled: Vec<Rectangle2dFilledOC> = vec![];
		let mut all_2d_rect_hollow: Vec<Rectangle2dHollowOC> = vec![];

		let (w, h) = (self.renderer.config.width, self.renderer.config.height);
		let wh = (w, h);
		let (wi, _hi) = (w as i32, h as i32);
		let (wf, hf) = (w as f32, h as f32);
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
				for (dx, dz, _x, _z, _is_x_flipped_local, _is_z_flipped_local, _chunk) in iter {
					// let step = 2_f32.powi(max(dx.abs(), dz.abs()) - 1); // TODO: use if render_distance > something?
					let step = 1.;
					let mut x = -CHUNK_SIZE_HALF;
					while x < CHUNK_SIZE_HALF {
						let mut z = -CHUNK_SIZE_HALF;
						while z < CHUNK_SIZE_HALF {
							let pos = Vec3::new((dx as f32)*CHUNK_SIZE + x, 0., (dz as f32)*CHUNK_SIZE + z);
							let pos = pos.flip_x_if(self.state.is_x_flipped_global);
							let pos = pos.flip_z_if(self.state.is_z_flipped_global);
							let color = {
								let mut c = base_color(&self.state.dim_base_la_for_floor_color);
								let pos_rel_to_cam = pos - self.state.camera.position;
								if self.state.is_darkness_at_base {
									// TODO: better attenuation curve
									c = ((c as f32) / (1. + 2e-3*pos_rel_to_cam.length_squared())) as u8;
								}
								ColorU8::new(c, c, c)
								// chunk.color
							};
							// let lines = [
							// 	(Vec3::new(pos.x - step/3., 0., pos.z - step/3.),
							// 	 Vec3::new(pos.x + step/3., 0., pos.z + step/3.)),
							// 	(Vec3::new(pos.x - step/3., 0., pos.z + step/3.),
							// 	 Vec3::new(pos.x + step/3., 0., pos.z - step/3.)),
							// ];
							// for (a, b) in lines.into_iter() {
							// 	all_3d_lines_oc.push(Line3dOC::from(a, b, color));
							// }
							// all_3d_points.push(Point3d::from(pos, chunk.color));
							all_3d_points.push(Point3d::new(pos, color));
							z += step;
						}
						x += step;
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
							Vec3::new((dx as f32)*CHUNK_SIZE, 0., (dz as f32)*CHUNK_SIZE)
								.flip_x_if(self.state.is_x_flipped_global).flip_z_if(self.state.is_z_flipped_global);
						let color = {
							let ColorU8 { mut r, mut g, mut b, .. } = chunk.color;
							if self.state.is_darkness_at_base {
								let pos_rel_to_cam = shift - self.state.camera.position;
								// TODO: better attenuation curve
								r = ((r as f32) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
								g = ((g as f32) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
								b = ((b as f32) / (1. + 1e-2*pos_rel_to_cam.length_squared())) as u8;
							}
							ColorU8::new(r, g, b)
						};
						for renderable_shape in ro.get_renderable_shapes(&self.state.camera) {
							// TODO(optim): do these computations on gpu?
							match renderable_shape {
								Points3dNC_(points) => {
									all_3d_points.extend(
										points.iter().map(|p|
											Point3d::new(
												p.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
												color
											)
										)
									);
								}
								Lines3d_(lines) => {
									all_3d_lines.extend(
										lines.iter().map(|Line3d { a, b }|
											Line3d::new(
												Point3d::new(
													a.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													a.color
												),
												Point3d::new(
													b.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													b.color
												)
											)
										)
									);
								}
								Lines3dNC_(lines) => {
									all_3d_lines_oc.extend(
										lines.iter().map(|Line3dNC { a, b }|
											Line3dOC::from(
												a.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
												b.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
												color
											)
										)
									);
								}
								LineStrip3dNC_(points) => {
									all_3d_lines_oc.extend(
										points.array_windows().map(|[a, b]|
											Line3dOC::from(
												a.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
												b.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
												color
											)
										)
									);
								}
								Triangles3d_(triangles) => {
									all_3d_triangles.extend(
										triangles.iter().map(|Triangle3d { a, b, c }|
											Triangle3d::new(
												Point3d::new(
													a.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													a.color
												),
												Point3d::new(
													b.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													b.color
												),
												Point3d::new(
													c.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													c.color
												)
											)
										)
									);
								}
								Quads3d_(quads) => {
									all_3d_quads.extend(
										quads.iter().map(|Quad3d { a, b, c, d }|
											Quad3d::new(
												Point3d::new(
													a.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													a.color
												),
												Point3d::new(
													b.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													b.color
												),
												Point3d::new(
													c.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													c.color
												),
												Point3d::new(
													d.v.flip_x_if(is_x_flipped).flip_z_if(is_z_flipped) + shift,
													d.color
												)
											)
										)
									);
								}
								_ => unimplemented!("{renderable_shape:?}")
							}
						}
					}
				}
			}
			Dimension::SurfaceWorld => {
				const MESH_SIZE: u32 = 100;
				const MESH_STEP: f32 = 0.2;
				const LODS: &[(u32, ColorU8)] = &[
					(27, ColorU8::gray(4)),
					(9, ColorU8::gray(16)),
					(3, ColorU8::gray(64)),
					(1, ColorU8::WHITE),
				];
				let params = &self.state.surface_world_params;
				fn surface_at(x: f32, z: f32, params: &[(f32, f32, f32, f32)]) -> f32 {
					params.iter().map(|(amplitude, phase, cx, cz)| {
						// TODO(optim): bench if taking /n out of here makes it faster
						amplitude * sin(phase + cx*x + cz*z) / (params.len() as f32)//.powf(*amplitude)
					}).sum()
				}
				for (lod_n, lod_color) in LODS {
					// canvas.set_draw_color(*lod_color);
					let mesh_step = MESH_STEP * (*lod_n as f32);
					let cx = self.state.camera.position.x - (MESH_SIZE as f32 - 1.) * mesh_step / 2.;
					let cz = self.state.camera.position.z - (MESH_SIZE as f32 - 1.) * mesh_step / 2.;
					let surface = Vec2D::from_fn(MESH_SIZE, MESH_SIZE, |x, z| {
						let x = (x as f32) * mesh_step;
						let z = (z as f32) * mesh_step;
						surface_at(x + cx - cx.rem_euclid(mesh_step), z + cz - cz.rem_euclid(mesh_step), params)
					});
					let cx = cx - cx.rem_euclid(mesh_step);
					let cz = cz - cz.rem_euclid(mesh_step);
					// TODO(optim): use draw_lines/chain
					// let mut lines_x = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
					// let mut lines_z = Vec::with_capacity((MESH_SIZE+1) as usize); // TODO: remove +1?
					for z in 0..MESH_SIZE-1 {
						let zf = (z as f32) * mesh_step;
						for x in 0..MESH_SIZE-1 {
							let xf = (x as f32) * mesh_step;
							let a = Vec3::new(xf+cx, surface[(x,z)], zf+cz);
							let b = Vec3::new(xf+cx+mesh_step, surface[(x+1,z)], zf+cz);
							all_3d_lines_oc.push(Line3dOC::from(a, b, *lod_color));
							let a = Vec3::new(xf+cx, surface[(x,z)], zf+cz);
							let b = Vec3::new(xf+cx, surface[(x,z+1)], zf+cz+mesh_step);
							all_3d_lines_oc.push(Line3dOC::from(a, b, *lod_color));
						}
					}
					for x in 0..MESH_SIZE-1 {
						let z = MESH_SIZE-1;
						let zf = (z as f32) * mesh_step;
						let xf = (x as f32) * mesh_step;
						let a = Vec3::new(xf+cx, surface[(x,z)], zf+cz);
						let b = Vec3::new(xf+cx+mesh_step, surface[(x+1,z)], zf+cz);
						all_3d_lines_oc.push(Line3dOC::from(a, b, *lod_color));
					}
					for z in 0..MESH_SIZE-1 {
						let x = MESH_SIZE-1;
						let xf = (x as f32) * mesh_step;
						let zf = (z as f32) * mesh_step;
						let a = Vec3::new(xf+cx, surface[(x,z)], zf+cz);
						let b = Vec3::new(xf+cx, surface[(x,z+1)], zf+cz+mesh_step);
						all_3d_lines_oc.push(Line3dOC::from(a, b, *lod_color));
					}
				}
			}
		}

		// -------------------- UI --------------------

		if self.state.is_extra_info_shown {
			let text_size = 3;
			let color = ColorU8::GRAY_32;
			let mut lines = vec![
				format!("XYZ: {:.3}, {:.3}, {:.3}", self.state.camera.position.x, self.state.camera.position.y, self.state.camera.position.z),
				format!("CHUNK XZ: {}, {}", self.state.current_chunk_x, self.state.current_chunk_z),
				format!("MOVE TYPE: {}", self.state.camera.movement_type.to_str_uppercase()),
				format!("FOV: {:.3}", self.state.camera.fov_x.to_degrees()),
				format!("TOPOLOGY IS ALT: {}", self.state.is_alt_topology.to_string().to_uppercase()),
			];
			if self.state.is_alt_topology {
				lines.push(format!("is xz flipped global: {}, {}", self.state.is_x_flipped_global, self.state.is_z_flipped_global).to_uppercase());
			}
			for (i, line) in lines.iter().enumerate() {
				let pixels: Vec<_> = get_text_pixels(line, (5, 5 + 35*(i as i32)), text_size, wh)
					.iter().map(|(x,y)| Point2d::from(*x, *y, color)).collect();
				all_2d_points.extend(pixels);
			}

			// // zqqx lang
			// for char_n in 0..5 {
			// 	let scale: u8 = 5;
			// 	let zqqx_char: [i8; 25] = array::from_fn(|i| {
			// 		let (i, j) = (i % 5, i / 5);
			// 		let cx = char_n as f32;
			// 		let cy = ((i+j*5) as f32).sqrt();
			// 		// let cz = ((j+i*5) as f32).ln_1p();
			// 		let cz = (frame_n as f32).ln_1p().ln_1p().ln_1p();
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

			// TODO
			let frame_end_timestamp = Instant::now();
			let frametime = frame_end_timestamp.duration_since(self.state.last_update_inst);
			let fps = 1. / frametime.as_secs_f64();
			// if fps < 60. { panic!() }
			let fps_text = format!("FPS?: {fps:.1}");
			let pixels: Vec<_> = get_text_pixels(&fps_text, (wi - 5 - (fps_text.len() as i32) * (text_size as i32) * 6, 5), text_size, wh)
				.iter().map(|(x,y)| Point2d::from(*x, *y, color)).collect();
			all_2d_points.extend(pixels);
		}

		if self.state.is_help_opened {
			const PADDING: f32 = 30.;
			const ITEM_Y: f32 = 30.;
			const ITEMS_N: u32 = 15;
			debug_assert_eq!(1, ITEMS_N % 2);
			const SIZE_X: f32 = 1000.;
			const SIZE_Y: f32 = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as f32);
			const ITEM_X: f32 = SIZE_X - 2.*PADDING;
			all_2d_rect_filled.push(Rectangle2dFilledOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::BLACK });
			all_2d_rect_hollow.push(Rectangle2dHollowOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::WHITE });
			const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY_64;
			const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
			// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
			const ITEM_TEXT_SIZE: u8 = 5;
			const ITEM_INNER_PADDING: f32 = (ITEM_Y - (ITEM_TEXT_SIZE as f32)*(FONT_H as f32)) / 2.;
			let i_init: u32 = self.state.help_line_index.saturating_sub((ITEMS_N-1)/2);
			let mut i: u32 = i_init;
			while i - i_init < ITEMS_N && i < self.state.help_lines.len() as u32 {
				let help_line = &self.state.help_lines[i as usize];
				let color = if i == self.state.help_line_index { ITEM_SELECTED_COLOR } else { ITEM_UNSELECTED_COLOR };
				let item_cx = wfh;
				let item_cy = hfh - SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as f32);
				let text_x = (item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32;
				let text_y = (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32;
				all_2d_points.extend(
					get_text_pixels(help_line, (text_x, text_y), ITEM_TEXT_SIZE, wh)
						.iter().map(|(x,y)| Point2d::from(*x, *y, color))
				);
				i += 1;
			}
		}

		if self.state.is_inventory_opened {
			const PADDING: f32 = 30.;
			const ITEM_Y: f32 = 50.;
			const ITEMS_N: u32 = 11;
			debug_assert_eq!(1, ITEMS_N % 2);
			const SIZE_X: f32 = 900.;
			const SIZE_Y: f32 = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as f32);
			const ITEM_X: f32 = SIZE_X - 2.*PADDING;
			all_2d_rect_filled.push(Rectangle2dFilledOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::BLACK });
			all_2d_rect_hollow.push(Rectangle2dHollowOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::WHITE });
			const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY_64;
			const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
			// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
			const ITEM_TEXT_SIZE: u8 = 5;
			const ITEM_INNER_PADDING: f32 = (ITEM_Y - (ITEM_TEXT_SIZE as f32)*(FONT_H as f32)) / 2.;
			let i_init: u32 = self.state.inventory_item_index.saturating_sub((ITEMS_N-1)/2);
			let mut i: u32 = i_init;
			while i - i_init < ITEMS_N && i < self.state.inventory_items.len() as u32 {
				let inventory_item = &self.state.inventory_items[i as usize];
				let color = if i == self.state.inventory_item_index { ITEM_SELECTED_COLOR } else { ITEM_UNSELECTED_COLOR };
				let item_cx = wfh;
				let item_cy = hfh - SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as f32);
				all_2d_rect_hollow.push(Rectangle2dHollowOC { x: item_cx, y: item_cy, w: ITEM_X, h: ITEM_Y, color });
				let text_x = (item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32;
				let text_y = (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32;
				all_2d_points.extend(
					get_text_pixels(&inventory_item.to_string(), (text_x, text_y), ITEM_TEXT_SIZE, wh)
						.iter().map(|(x,y)| Point2d::from(*x, *y, color))
				);
				i += 1;
			}
		}

		if self.state.is_paused {
			const PADDING: f32 = 50.;
			const ITEM_Y: f32 = 80.;
			const ITEMS_N: u32 = 7;
			debug_assert_eq!(1, ITEMS_N % 2);
			const SIZE_X: f32 = 800.;
			const SIZE_Y: f32 = PADDING + (ITEM_Y+PADDING)*(ITEMS_N as f32);
			const ITEM_X: f32 = SIZE_X - 2.*PADDING;
			all_2d_rect_filled.push(Rectangle2dFilledOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::BLACK });
			all_2d_rect_hollow.push(Rectangle2dHollowOC { x: wfh, y: hfh, w: SIZE_X, h: SIZE_Y, color: ColorU8::WHITE });
			const ITEM_UNSELECTED_COLOR: ColorU8 = ColorU8::GRAY_64;
			const ITEM_SELECTED_COLOR: ColorU8 = ColorU8::WHITE;
			// const ITEM_TEXT_COLOR: ColorU8 = ColorU8::GREEN;
			const ITEM_TEXT_SIZE: u8 = 5;
			const ITEM_INNER_PADDING: f32 = (ITEM_Y - (ITEM_TEXT_SIZE as f32)*(FONT_H as f32)) / 2.;
			let i_init: u32 = self.state.pause_menu_item_index.saturating_sub((ITEMS_N-1)/2);
			let mut i: u32 = i_init;
			while i - i_init < ITEMS_N && i < self.state.pause_menu_items.len() as u32 {
				let menu_item = &self.state.pause_menu_items[i as usize];
				let color = if i == self.state.pause_menu_item_index { ITEM_SELECTED_COLOR } else { ITEM_UNSELECTED_COLOR };
				let item_cx = wfh;
				let item_cy = hfh - SIZE_Y/2. + PADDING + ITEM_Y/2. + (PADDING+ITEM_Y)*((i - i_init) as f32);
				all_2d_rect_hollow.push(Rectangle2dHollowOC { x: item_cx, y: item_cy, w: ITEM_X, h: ITEM_Y, color });
				let text_x = (item_cx-ITEM_X/2.+ITEM_INNER_PADDING) as i32;
				let text_y = (item_cy-ITEM_Y/2.+ITEM_INNER_PADDING) as i32;
				all_2d_points.extend(
					get_text_pixels(menu_item.to_str(), (text_x, text_y), ITEM_TEXT_SIZE, wh)
						.iter().map(|(x,y)| Point2d::from(*x, *y, color))
				);
				i += 1;
			}
		}

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

		{ // -------------------- 3D RENDER --------------------
			let view_proj_3d = self.state.camera.proj_matrix() * self.state.camera.view_matrix();
			let uniforms_3d = Uniforms { view_proj: view_proj_3d.to_cols_array_2d() };
			self.renderer.queue.write_buffer(&self.renderer.uniform_buffer_3d, 0, bytemuck::bytes_of(&uniforms_3d));

			let triangles_3d: Vec<Vertex> = all_3d_triangles.into_iter().flat_map(|t| t.to_vertices())
				.chain(all_3d_quads.into_iter().flat_map(|q| q.to_vertices()))
				.chain(all_3d_quads_oc.into_iter().flat_map(|q| q.to_vertices()))
				.collect();
			let lines_3d: Vec<Vertex> = all_3d_lines.into_iter().flat_map(|l| l.to_vertices())
				.chain(all_3d_lines_oc.into_iter().flat_map(|l| l.to_vertices()))
				.collect();
			let points_3d: Vec<Vertex> = all_3d_points.into_iter().map(|p| p.to_vertex())
				.collect();

			let counts_3d = [
				triangles_3d.len() as u32,
				lines_3d.len() as u32,
				points_3d.len() as u32,
			];

			let buffers_3d = [
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&triangles_3d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&lines_3d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&points_3d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
			];

			pass.set_bind_group(0, &self.renderer.bind_group_3d, &[]);
			for i in 0..self.renderer.pipelines_3d.len() {
				if counts_3d[i] > 0 {
					pass.set_pipeline(&self.renderer.pipelines_3d[i]);
					pass.set_vertex_buffer(0, buffers_3d[i].slice(..));
					pass.draw(0..counts_3d[i], 0..1);
				}
			}
		}

		{ // -------------------- 2D RENDER --------------------
			let view_proj_2d = Mat4::orthographic_rh(0.0, wf, hf, 0.0, -1.0, 1.0); // orthographic projection
			let uniforms_2d = Uniforms { view_proj: view_proj_2d.to_cols_array_2d() };
			self.renderer.queue.write_buffer(&self.renderer.uniform_buffer_2d, 0, bytemuck::bytes_of(&uniforms_2d));

			let triangles_2d: Vec<Vertex> = all_2d_triangles.into_iter().flat_map(|t| t.to_vertices())
				.chain(all_2d_rect_filled.into_iter().flat_map(|r| r.to_vertices()))
				.collect();
			let lines_2d: Vec<Vertex> = all_2d_lines.into_iter().flat_map(|l| l.to_vertices())
				.chain(all_2d_rect_hollow.into_iter().flat_map(|r| r.to_vertices()))
				.collect();
			let points_2d: Vec<Vertex> = all_2d_points.into_iter().map(|p| p.to_vertex())
				.collect();

			let counts_2d = [
				triangles_2d.len() as u32,
				lines_2d.len() as u32,
				points_2d.len() as u32,
			];

			let buffers_2d = [
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&triangles_2d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&lines_2d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
				self.renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&points_2d),
					usage: wgpu::BufferUsages::VERTEX,
				}),
			];

			pass.set_bind_group(0, &self.renderer.bind_group_2d, &[]);
			for i in 0..self.renderer.pipelines_2d.len() {
				if counts_2d[i] > 0 {
					pass.set_pipeline(&self.renderer.pipelines_2d[i]);
					pass.set_vertex_buffer(0, buffers_2d[i].slice(..));
					pass.draw(0..counts_2d[i], 0..1);
				}
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





// TODO(refactor): rename to GpuVertex
// TODO(optim): separate into Vertex3d and Vertex2d (remove one f32 lol)
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}
impl Vertex {
	fn new(position: [f32; 3], color: [f32; 3]) -> Self {
		Self { position, color }
	}
	fn from(position: Vec3, color: ColorU8) -> Self {
		Self::new(position.to_array(), color.to_array())
	}
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
	pipelines_3d: [wgpu::RenderPipeline; 3],
	pipelines_2d: [wgpu::RenderPipeline; 3],
	uniform_buffer_3d: wgpu::Buffer,
	uniform_buffer_2d: wgpu::Buffer,
	bind_group_3d: wgpu::BindGroup,
	bind_group_2d: wgpu::BindGroup,
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
			present_mode: wgpu::PresentMode::AutoVsync,
			alpha_mode: caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};

		surface.configure(&device, &config);

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
		});

		let uniforms_3d = Uniforms { view_proj: Mat4::IDENTITY.to_cols_array_2d() };
		let uniforms_2d = Uniforms { view_proj: Mat4::IDENTITY.to_cols_array_2d() };
		let uniform_buffer_3d = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Uniform Buffer"),
				contents: bytemuck::bytes_of(&uniforms_3d),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			},
		);
		let uniform_buffer_2d = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Uniform Buffer"),
				contents: bytemuck::bytes_of(&uniforms_2d),
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
		let bind_group_3d = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: uniform_buffer_3d.as_entire_binding(),
				}
			],
			label: None,
		});
		let bind_group_2d = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: uniform_buffer_2d.as_entire_binding(),
				}
			],
			label: None,
		});

		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[Some(&bind_group_layout)],
			immediate_size: 0,
		});

		let make_pipeline = |topology, blend| {
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
						blend: Some(blend),
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

		let pipelines_3d = [
			make_pipeline(wgpu::PrimitiveTopology::TriangleList, wgpu::BlendState::REPLACE),
			make_pipeline(wgpu::PrimitiveTopology::LineList, wgpu::BlendState::REPLACE),
			make_pipeline(wgpu::PrimitiveTopology::PointList, wgpu::BlendState::REPLACE),
		];
		let pipelines_2d = [ // TODO?
			make_pipeline(wgpu::PrimitiveTopology::TriangleList, wgpu::BlendState::ALPHA_BLENDING),
			make_pipeline(wgpu::PrimitiveTopology::LineList, wgpu::BlendState::ALPHA_BLENDING),
			make_pipeline(wgpu::PrimitiveTopology::PointList, wgpu::BlendState::ALPHA_BLENDING),
		];

		Self {
			surface,
			device,
			queue,
			config,
			pipelines_3d,
			pipelines_2d,
			uniform_buffer_3d,
			uniform_buffer_2d,
			bind_group_3d,
			bind_group_2d,
		}
	}
}

struct GameState {
	camera: Camera,
	input: InputState,
	last_update_inst: Instant,
	is_redraw_needed: bool = true,

	// TODO(refactor)?: extract
	help_lines: Vec<String>,
	is_help_opened: bool = false,
	help_line_index: u32 = 0,

	// TODO(refactor)?: extract
	pause_menu_items: Vec<PauseMenuItem>, // TODO: static array?
	is_paused: bool = false,
	pause_menu_item_index: u32 = 0,

	is_darkness_at_base: bool = false,

	dimension: Dimension = Dimension::Base,
	dim_base_la_for_floor_color: LorenzAttractor,

	// TODO(refactor)?: extract
	inventory_items: Vec<InventoryItem>,
	is_inventory_opened: bool = false,
	inventory_item_index: u32 = 0,

	// TODO(refactor)?: move into Dimension::SurfaceWorld
	surface_world_params: Vec<(f32, f32, f32, f32)>,

	chunks: Vec2D<Chunk>,
	render_distance: u32 = 2,
	current_chunk_x: u32 = 0,
	current_chunk_z: u32 = 0,
	is_alt_topology: bool = true, // FIXME: must be false by default
	is_x_flipped_global: bool = false, // for alt topology
	is_z_flipped_global: bool = false, // for alt topology

	tick_n: u64 = 0,
	frame_n: u64 = 0,
	is_extra_info_shown: bool = true,

	// zqqx_lang: ZqqxLang,
}

impl GameState {
	fn new(w: f32, h: f32, rng: &mut ThreadRng) -> Self {
		let help_lines = [
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
		].map(|s| s.to_uppercase()).to_vec();

		let pause_menu_items = { use PauseMenuItem::*; vec![
			Quit,
			Back,
			GetRandomItems,
			ToggleTopology,
			ToggleDarkness,
			ToggleUnlimitedFov,
			ToggleShakyFov,
			ToggleVsync,
		]};

		let dim_base_la_for_floor_color = LorenzAttractor::new()
			.offset_params_(Vec3::random_unit_cube(rng) * 0.1)
			.offset_xyz(30., 0., 0.);

		let chunks = Vec2D::<Chunk>::from_fn(CHUNKS_N, CHUNKS_N, |_x, _z| {
			Chunk::new_random(rng)
		});
		// println!("chunks.len = {}", chunks.iter().count());

		Self {
			camera: Camera::new(w / h),
			input: InputState::new(),
			last_update_inst: Instant::now(),
			inventory_items: Vec::with_capacity(100),
			surface_world_params: gen_surface_world_params(rng),
			game_of_life_state: GameOfLifeState::from_seed(""),
			help_lines,
			pause_menu_items,
			dim_base_la_for_floor_color,
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
	const GROUNDED_CAMERA_Y: f32 = 1.5;
	const DEFAULT_POSITION: Vec3 = Vec3::new(0., Self::GROUNDED_CAMERA_Y, 0.);

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
		const SENSITIVITY: f32 = 0.01; // TODO: must be dependent on fov?
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

	fn update_fov(&mut self, input: &InputState, dt: f32, rng: &mut ThreadRng) {
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
		todo!()
	}

	fn reset_position(&mut self) {
		self.position = Self::DEFAULT_POSITION;
	}

	fn next_movement_type(&mut self) {
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
	view_proj: [[f32; 4]; 4],
}

#[derive(Debug)]
struct InputState {
	// "continuous":
	// TODO!: rename into move_<direction>
	forward: bool = false,
	back: bool = false,
	left: bool = false,
	right: bool = false,
	up: bool = false,
	down: bool = false,
	roll_left: bool = false,
	roll_right: bool = false,
	reset_roll: bool = false,
	mouse_dx: f32 = 0.,
	mouse_dy: f32 = 0.,
	zoom_in: bool = false,
	zoom_out: bool = false,
	is_fast_move: bool = false,
	// "discrete":
}
impl InputState {
	fn new() -> Self {
		Self { .. }
	}
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





const DIM_BASE_LA_SPEED: f32 = 1e-2;

fn base_color(la: &LorenzAttractor) -> u8 {
	let x = la.get_linear_combination(1., 1., 1.);
	let c = x.clamp(1., 80.) as u8;
	c
}

fn gen_surface_world_param(rng: &mut ThreadRng) -> (f32, f32, f32, f32) {
	// returns amplitude, phase, cx, cz
	(
		rng.random_range(0. ..= 3_f32).powi(2),
		rng.random_range(0. ..= TAU),
		rng.random_range(-2. ..= 2.),
		rng.random_range(-2. ..= 2.),
	)
}
fn gen_surface_world_params(rng: &mut ThreadRng) -> Vec<(f32, f32, f32, f32)> {
	Vec::from_fn(
		rng.random_range(2. ..= 7_f32).powi(2).round() as usize,
		|_i| gen_surface_world_param(rng)
	)
}

fn main_loop_old() {
	// TODO: FPS
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
	GetRandomItems,
	ToggleTopology,
	ToggleDarkness,
	ToggleUnlimitedFov,
	ToggleShakyFov,
	ToggleVsync,
	// TODO: inc/dec render_distance
	Text(String), // just for test
}
impl PauseMenuItem {
	fn to_str(&self) -> &str {
		use PauseMenuItem::*;
		match self {
			Quit => "QUIT",
			Back => "BACK",
			GetRandomItems => "GET RANDOM ITEMS",
			ToggleTopology => "TOGGLE TOPOLOGY",
			ToggleDarkness => "TOGGLE DARKNESS",
			ToggleUnlimitedFov => "TOGGLE UNLIMITED FOV",
			ToggleShakyFov => "TOGGLE SHAKY FOV",
			ToggleVsync => "TOGGLE VSYNC",
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
	Cube { size: f32 },
	LorenzAttractor { size: f32, la: LorenzAttractor, last_points: Vec<Vec3>, max_len: u32 },
	// SpinningText?
	Monolith { sizes: Vec<f32> },
	Simplex { initpoints_rotplanes_rotvels_phases: Vec<(Vec3, Vec3, f32, f32)> },
	Icosahedron { size: f32, global_rotvel: f32, rotplanes_rotvels_phases: Vec<(Vec3, f32, f32)> },
	Kitty { size: f32, rotvel: f32, phase: f32 },
	Graph3d { connect_n: u32, global_rotvel: f32, initpoints_rotplanes_rotvels_phases: Vec<(Vec3, Vec3, f32, f32)> },
	// TravelingSalesmanProblemSolver in realtime
	RGBCubeHollow { size: f32, global_rotvel: f32, rotplanes_rotvels_phases: Vec<(Vec3, f32, f32)> },
	RGBCube { size: f32, global_rotvel: f32, rotplanes_rotvels_phases: Vec<(Vec3, f32, f32)> },
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
			1. => RenderableObject::Simplex {
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
			1. => RenderableObject::Icosahedron {
				size: rng.random_range(0.5 ..= 2.5),
				global_rotvel: rng.random_range(0.01 ..= 1.),
				rotplanes_rotvels_phases: Vec::from_fn(
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
				connect_n: rng.random_range(1 ..= 6),
				global_rotvel: rng.random_range(0.01 ..= 2.),
				initpoints_rotplanes_rotvels_phases: {
					macro_rules! random_r { () => { rng.random_range(0.8 ..= 2.3_f32).powi(2) }; }
					let equidistant_from_center = rng.random_bool(0.5).then(|| random_r!());
					let n = rng.random_range(10 ..= 200);
					(0..n).map(|_i| (
						Vec3::random_unit(rng) * if let Some(s) = equidistant_from_center { s } else { random_r!() },
						Vec3::random_unit(rng),
						rng.random_range(0.5 ..= 1.4_f32).powi(2),
						rng.random_range(0. ..= TAU),
					)).collect()
				},
			},
			0.1 => RenderableObject::RGBCubeHollow {
				size: rng.random_range(0.5 ..= 1.7_f32).powi(2),
				global_rotvel: rng.random_range(0.01 ..= 1.),
				rotplanes_rotvels_phases: Vec::from_fn(
					rng.random_range(1 ..= 5),
					|_i| (
						Vec3::random_unit(rng),
						rng.random_range(0.1 ..= 2.),
						rng.random_range(0. ..= TAU),
					)
				),
			},
			0.1 => RenderableObject::RGBCube {
				size: rng.random_range(0.5 ..= 1.7_f32).powi(2),
				global_rotvel: rng.random_range(0.01 ..= 1.),
				rotplanes_rotvels_phases: Vec::from_fn(
					rng.random_range(1 ..= 5),
					|_i| (
						Vec3::random_unit(rng),
						rng.random_range(0.1 ..= 2.),
						rng.random_range(0. ..= TAU),
					)
				),
			},
		}
	}
	fn is_time_dependent(&self) -> bool {
		use RenderableObject::*;
		match self {
			| LorenzAttractor { .. }
			| Simplex { .. }
			| Icosahedron { .. }
			| Kitty { .. }
			| Graph3d { .. }
			| RGBCubeHollow { .. }
			| RGBCube { .. }
			=> true,
			| Cube { .. }
			| Monolith { .. }
			=> false,
		}
	}
	fn update(&mut self, delta_time: f32) {
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
				la.step(delta_time);
			}
			Simplex { initpoints_rotplanes_rotvels_phases } => {
				for (_initpoint, _rotplane, rotation_velocity, phase) in initpoints_rotplanes_rotvels_phases.iter_mut() {
					*phase += *rotation_velocity * delta_time;
					if *phase > TAU { *phase -= TAU; }
				}
			}
			Icosahedron { global_rotvel, rotplanes_rotvels_phases, .. }
			| RGBCubeHollow { global_rotvel, rotplanes_rotvels_phases, .. }
			| RGBCube { global_rotvel, rotplanes_rotvels_phases, .. }
			=> {
				for (i, (_rotplane, rotation_velocity, phase)) in rotplanes_rotvels_phases.iter_mut().enumerate() {
					*phase += *global_rotvel * *rotation_velocity * delta_time / ((i + 1) as f32);
					if *phase > TAU { *phase -= TAU; }
				}
			}
			Kitty { phase, rotvel, .. } => {
				*phase += *rotvel * delta_time;
				if *phase > TAU { *phase -= TAU; }
			}
			Graph3d { global_rotvel, initpoints_rotplanes_rotvels_phases, .. } => {
				for (_initpoint, _rotplane, rotation_velocity, phase) in initpoints_rotplanes_rotvels_phases.iter_mut() {
					*phase += *global_rotvel * *rotation_velocity * delta_time;
					if *phase > TAU { *phase -= TAU; }
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
				vec![Lines3dNC_(vec![
					(Vec3::new(-s,-s,-s), Vec3::new(-s,-s, s)).into(),
					(Vec3::new(-s,-s,-s), Vec3::new(-s, s,-s)).into(),
					(Vec3::new(-s, s, s), Vec3::new(-s,-s, s)).into(),
					(Vec3::new(-s, s, s), Vec3::new(-s, s,-s)).into(),
					//
					(Vec3::new( s,-s,-s), Vec3::new( s,-s, s)).into(),
					(Vec3::new( s,-s,-s), Vec3::new( s, s,-s)).into(),
					(Vec3::new( s, s, s), Vec3::new( s,-s, s)).into(),
					(Vec3::new( s, s, s), Vec3::new( s, s,-s)).into(),
					//
					(Vec3::new(-s,-s,-s), Vec3::new( s,-s,-s)).into(),
					(Vec3::new( s, s, s), Vec3::new(-s, s, s)).into(),
					(Vec3::new(-s,-s, s), Vec3::new( s,-s, s)).into(),
					(Vec3::new(-s, s,-s), Vec3::new( s, s,-s)).into(),
				])]
			}
			LorenzAttractor { size, last_points, .. } => {
				vec![LineStrip3dNC_(last_points.iter().map(|&p| p * *size).collect())]
			}
			Monolith { sizes } => {
				vec![Lines3dNC_(sizes.iter().map(|size| {
					let s = size / 2.;
					vec![
						(Vec3::new(-s,-s,-s), Vec3::new(-s,-s, s)).into(),
						(Vec3::new(-s,-s,-s), Vec3::new(-s, s,-s)).into(),
						(Vec3::new(-s, s, s), Vec3::new(-s,-s, s)).into(),
						(Vec3::new(-s, s, s), Vec3::new(-s, s,-s)).into(),
						//
						(Vec3::new( s,-s,-s), Vec3::new( s,-s, s)).into(),
						(Vec3::new( s,-s,-s), Vec3::new( s, s,-s)).into(),
						(Vec3::new( s, s, s), Vec3::new( s,-s, s)).into(),
						(Vec3::new( s, s, s), Vec3::new( s, s,-s)).into(),
					]
				}).flatten().collect())]
			}
			Simplex { initpoints_rotplanes_rotvels_phases } => {
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
						lines.push(Line3dNC::new(a, b));
					}
				}
				vec![Lines3dNC_(lines)]
			}
			Icosahedron { size, rotplanes_rotvels_phases, .. } => {
				const PHI: f32 = GOLDEN_RATIO;
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
				for (rotplane, _rotvel, phase) in rotplanes_rotvels_phases.iter() {
					for vertex in vertices.iter_mut() {
						*vertex = vertex.rotate_axis(*rotplane, *phase);
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
						lines.push(Line3dNC::new(vertex, vertices[nearest_vertex_index as usize]));
					}
				}
				debug_assert_eq!(30, lines.len());
				vec![Lines3dNC_(lines)]
			}
			Kitty { size, phase, .. } => {
				// TODO(fix): wrong in alt topology with is_x_flipped/is_z_flipped
				let angles_of_points_on_circle_20: Vec<f32> = {
					const N: u32 = 20;
					let tau_div_n = TAU / (N as f32);
					Vec::from_fn(N as usize, |i| (i as f32) * tau_div_n)
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
				let angles_of_points_on_circle_10: Vec<f32> = {
					const N: u32 = 10;
					let tau_div_n = TAU / (N as f32);
					Vec::from_fn(N as usize, |i| (i as f32) * tau_div_n)
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
					LineStrip3dNC_(points_outline),
					LineStrip3dNC_(points_eye_left),
					LineStrip3dNC_(points_eye_right),
					LineStrip3dNC_(points_smile),
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
						let dist2 = if i != j { points[i].distance_squared(points[j]) } else { f32::MAX };
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
						lines.push(Line3dNC{a, b});
					}
				}
				vec![Lines3dNC_(lines)]
			}
			RGBCubeHollow { size, rotplanes_rotvels_phases, .. } => {
				let size = *size;
				let mut vertices = [
					Point3d::new(Vec3::new( size,  size,  size), ColorU8::from_int(0x000000)), // 0
					Point3d::new(Vec3::new( size,  size, -size), ColorU8::from_int(0x0000ff)), // 1
					Point3d::new(Vec3::new( size, -size,  size), ColorU8::from_int(0x00ff00)), // 2
					Point3d::new(Vec3::new( size, -size, -size), ColorU8::from_int(0x00ffff)), // 3
					Point3d::new(Vec3::new(-size,  size,  size), ColorU8::from_int(0xff0000)), // 4
					Point3d::new(Vec3::new(-size,  size, -size), ColorU8::from_int(0xff00ff)), // 5
					Point3d::new(Vec3::new(-size, -size,  size), ColorU8::from_int(0xffff00)), // 6
					Point3d::new(Vec3::new(-size, -size, -size), ColorU8::from_int(0xffffff)), // 7
				];
				for (rotplane, _rotvel, phase) in rotplanes_rotvels_phases.iter() {
					for v in vertices.iter_mut() {
						v.v = v.v.rotate_axis(*rotplane, *phase);
					}
				}
				vec![Lines3d_(vec![
					Line3d::new(vertices[0], vertices[1]),
					Line3d::new(vertices[0], vertices[2]),
					Line3d::new(vertices[0], vertices[4]),
					Line3d::new(vertices[1], vertices[3]),
					Line3d::new(vertices[1], vertices[5]),
					Line3d::new(vertices[2], vertices[3]),
					Line3d::new(vertices[2], vertices[6]),
					Line3d::new(vertices[3], vertices[7]),
					Line3d::new(vertices[4], vertices[5]),
					Line3d::new(vertices[4], vertices[6]),
					Line3d::new(vertices[5], vertices[7]),
					Line3d::new(vertices[6], vertices[7]),
				])]
			}
			RGBCube { size, rotplanes_rotvels_phases, .. } => {
				let size = *size;
				let mut vertices = [
					Point3d::new(Vec3::new( size,  size,  size), ColorU8::from_int(0x000000)), // 0 : 1 2 4 : 1 2 4
					Point3d::new(Vec3::new( size,  size, -size), ColorU8::from_int(0x0000ff)), // 1 : 0 3 5 : 3 5
					Point3d::new(Vec3::new( size, -size,  size), ColorU8::from_int(0x00ff00)), // 2 : 0 3 6 : 3 6
					Point3d::new(Vec3::new( size, -size, -size), ColorU8::from_int(0x00ffff)), // 3 : 1 2 7 : 7
					Point3d::new(Vec3::new(-size,  size,  size), ColorU8::from_int(0xff0000)), // 4 : 0 5 6 : 5 6
					Point3d::new(Vec3::new(-size,  size, -size), ColorU8::from_int(0xff00ff)), // 5 : 1 4 7 : 7
					Point3d::new(Vec3::new(-size, -size,  size), ColorU8::from_int(0xffff00)), // 6 : 2 4 7 : 7
					Point3d::new(Vec3::new(-size, -size, -size), ColorU8::from_int(0xffffff)), // 7 : 3 5 6 : -
					// faces: 0123 0145 0246 1357 2367 4567
				];
				for (rotplane, _rotvel, phase) in rotplanes_rotvels_phases.iter() {
					for v in vertices.iter_mut() {
						v.v = v.v.rotate_axis(*rotplane, *phase);
					}
				}
				vec![Quads3d_(vec![
					Quad3d::new(vertices[0], vertices[1], vertices[2], vertices[3]),
					Quad3d::new(vertices[0], vertices[1], vertices[4], vertices[5]),
					Quad3d::new(vertices[0], vertices[2], vertices[4], vertices[6]),
					Quad3d::new(vertices[1], vertices[3], vertices[5], vertices[7]),
					Quad3d::new(vertices[2], vertices[3], vertices[6], vertices[7]),
					Quad3d::new(vertices[4], vertices[5], vertices[6], vertices[7]),
				])]
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
			Simplex { initpoints_rotplanes_rotvels_phases } => format!("simplex ({n} points)", n=initpoints_rotplanes_rotvels_phases.len()),
			Icosahedron { size, global_rotvel, rotplanes_rotvels_phases } => format!("icosahedron ({n} rotation vectors)", n=rotplanes_rotvels_phases.len()),
			Kitty { size, rotvel, phase } => format!("kitty (size={size:.2})"),
			Graph3d { connect_n, global_rotvel, initpoints_rotplanes_rotvels_phases } => format!("graph 3d ({n} points, {nc} connect)", n=initpoints_rotplanes_rotvels_phases.len(), nc=connect_n),
			RGBCubeHollow { size, global_rotvel, rotplanes_rotvels_phases } => format!("color cube hollow (size={size:.2})"),
			RGBCube { size, global_rotvel, rotplanes_rotvels_phases } => format!("color cube hollow (size={size:.2})"),
		}.to_uppercase()
	}
}





const CHUNKS_N: u32 = 17;
const CHUNK_SIZE: f32 = 20.;
const CHUNK_SIZE_HALF: f32 = CHUNK_SIZE / 2.;
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
					4. => vec![], // empty / void / nothing
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


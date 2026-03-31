//! dark fluxus

#![allow(
	clippy::collapsible_if,
	clippy::let_and_return,
)]

#![deny(
	unused_must_use,
	unused_results,
	unused_variables,
)]

use std::{array, f32::consts::PI, time::SystemTime};

// use encoding_rs::UTF_8;
// use llama_cpp_2::{context::params::LlamaContextParams, llama_backend::LlamaBackend, llama_batch::LlamaBatch, model::{AddBos, LlamaModel, params::LlamaModelParams}, sampling::LlamaSampler};
use minifb::{Key, Window, WindowOptions};
use rand::{Rng, rng};

mod colors;
mod consts;
mod extensions;
mod font_rendering;
mod frame_buffer;
mod lorenz_attractor;
mod math_aliases;
// mod teapot;
mod utils_io;
mod vec2d;
mod vec3d;
mod zqqx_lang;

use colors::*;
use consts::*;
use extensions::*;
use font_rendering::*;
use frame_buffer::*;
use lorenz_attractor::*;
use math_aliases::*;
use utils_io::*;
use vec2d::*;
use vec3d::*;
use zqqx_lang::*;





fn main() {
	#[allow(unused_variables)]
	let mut rng = rng();

	// // const MODEL_PATH: &str = "llm_models/tinyllama-1.1b-chat-v1.0.Q8_0.gguf"; // dumb af // src: https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/tree/main
	// // const MODEL_PATH: &str = "llm_models/Llama-3.2-1B-Instruct-IQ3_M.gguf"; // src: https://huggingface.co/bartowski/Llama-3.2-1B-Instruct-GGUF/tree/main
	// const MODEL_PATH: &str = "llm_models/Llama-3.2-1B-Instruct-Q8_0.gguf"; // best
	// // const MODEL_PATH: &str = "llm_models/Llama-3.2-1B-Instruct-f16.gguf";
	// // const MODEL_PATH: &str = "llm_models/machbase-llama3b.Q6_K.gguf"; // src: https://huggingface.co/mradermacher/machbase-llama3b-GGUF/tree/main
	//
	// // 1. Initialize the llama.cpp backend
	// let mut backend = LlamaBackend::init().unwrap();
	// backend.void_logs();
	// let backend = backend;
	//
	// // 2. Load the model from the file
	// let model_params = LlamaModelParams::default();
	// let model = LlamaModel::load_from_file(&backend, MODEL_PATH, &model_params)
	// 	.expect(&format!("Failed to load model. Check if '{MODEL_PATH}' exists."));
	//
	// let mut items: Vec<String> = vec![
	// 	format!("water"),
	// 	format!("air"),
	// 	format!("stone"),
	// 	format!("fire"),
	// 	// format!("ambrosia"),
	// 	// format!("flumoxium"),
	// 	// format!("diminution"),
	// ];
	//
	// loop {
	// 	for (i, item) in items.iter().enumerate() {
	// 		println!("{i}. {item}");
	// 	}
	// 	println!();
	// 	// let Ok(n1) = prompt("Give first item number: ").parse::<usize>() else { continue };
	// 	// let Ok(n2) = prompt("Give second item number: ").parse::<usize>() else { continue };
	// 	// let Some(item_1) = &items.get(n1) else { continue };
	// 	// let Some(item_2) = &items.get(n2) else { continue };
	// 	let item_1 = &items[rng.random_range(0..items.len())];
	// 	let item_2 = &items[rng.random_range(0..items.len())];
	//
	// 	println!("{item_1} + {item_2}:");
	//
	// 	// 3. Create a context for execution
	// 	let ctx_params = LlamaContextParams::default();
	// 	let mut ctx = model.new_context(&backend, ctx_params).unwrap();
	//
	// 	// 4. Tokenize the prompt
	// 	let prompt = format!("You are a master wizard alchemist. Be creative. What is the result of mixing {item_1} with {item_2}? Answer with only one word, the name of new thing, no more text output, question, explanations or anything. One word.");
	// 	let tokens = model.str_to_token(&prompt, AddBos::Always).unwrap();
	//
	// 	// print!("\n\n\n{}", prompt);
	// 	// io::stdout().flush().unwrap();
	//
	// 	// 5. Create a batch and add the prompt tokens
	// 	// We allocate space for 512 tokens, 1 sequence.
	// 	let mut batch = LlamaBatch::new(512, 1);
	// 	// let last_index = tokens.len() - 1;
	//
	// 	for (i, &token) in tokens.iter().enumerate() {
	// 		let is_last = i == tokens.len() - 1;
	// 		batch.add(token, i as i32, &[0], is_last).unwrap(); // Must be true for the very last prompt token
	// 	}
	// 	ctx.decode(&mut batch).unwrap();
	//
	// 	// 5. Initialize the modern Sampler (Greedy)
	// 	let mut sampler = LlamaSampler::greedy();
	//
	// 	// 6. Text Generation Loop
	// 	let max_tokens = 100;
	// 	let mut n_cur = tokens.len();
	//
	// 	let mut decoder = UTF_8.new_decoder();
	//
	// 	// println!();
	//
	// 	let mut output = String::new();
	//
	// 	while n_cur < max_tokens {
	// 		// This sampler call fails if the PREVIOUS decode didn't request logits
	// 		let new_token_id = sampler.sample(&ctx, batch.n_tokens() - 1);
	//
	// 		if new_token_id == model.token_eos() {
	// 			break;
	// 		}
	//
	// 		// Decode to string
	// 		let token_str = model.token_to_piece(new_token_id, &mut decoder, false, None).unwrap();
	// 		output += &token_str;
	// 		// print!("{}", token_str);
	// 		// io::stdout().flush().unwrap();
	//
	// 		// --- THE FIX IS HERE ---
	// 		batch.clear();
	//
	// 		// The last 'true' tells llama.cpp: "I want to sample from THIS token next"
	// 		// Without this 'true', the next iteration's sampler.sample() will crash.
	// 		batch.add(new_token_id, n_cur as i32, &[0], true).unwrap();
	//
	// 		ctx.decode(&mut batch).unwrap();
	// 		n_cur += 1;
	// 	}
	//
	// 	dbg!(&output);
	// 	output = output.trim().chars().filter(|c| c.is_alphanumeric() || " ".contains(*c)).collect();
	// 	output = output.to_lowercase();
	// 	// output = format!("{s}{output}{s}", s=" ".repeat(10));
	// 	dbg!(&output);
	// 	for item in items.iter() {
	// 		output = output.replace(item, "");
	// 		// output = output.replace(&format!(" {item} "), "");
	// 	}
	// 	output = output.trim().to_lowercase();
	// 	dbg!(&output);
	// 	if !output.is_empty() && output.chars().filter(|&c| c == ' ').count() < 3 && output.chars().count() >= 3 {
	// 		items.push(output);
	// 	}
	//
	// 	println!();
	// }
	//
	// return;

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

	#[derive(Debug)]
	struct Camera {
		pos: Vec3f,
		forward: Vec3f,
		up: Vec3f,
		fov: float, // in radians
	}
	impl Camera {
		/// returns (right, up, forward) vectors
		fn basis(&self) -> (Vec3f, Vec3f, Vec3f) {
			let f = self.forward.normed();
			let r = f.cross(self.up).normed();
			let u = r.cross(f);
			(r, u, f)
		}

		fn project_point(&self, p: Vec3f, width: f32, height: f32) -> Option<Vec2f> {
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

		fn clip_line_near(&self, a: Vec3f, b: Vec3f, near: f32) -> Option<(Vec3f, Vec3f)> {
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

		fn project_line(
			&self,
			line: &(Vec3f, Vec3f),
			width: f32,
			height: f32,
			near: f32,
		) -> Option<(Vec2f, Vec2f)> {
			let (a, b) = self.clip_line_near(line.0, line.1, near)?;
			let pa = self.project_point(a, width, height)?;
			let pb = self.project_point(b, width, height)?;
			clip_line_viewport(pa, pb, width, height)
		}
	}

	const INSIDE: u8 = 0;
	const LEFT: u8 = 1;
	const RIGHT: u8 = 2;
	const BOTTOM: u8 = 4;
	const TOP: u8 = 8;

	fn compute_outcode(p: Vec2f, w: f32, h: f32) -> u8 {
		let mut code = INSIDE;
		if p.x < 0. { code |= LEFT; } else if p.x > w { code |= RIGHT; }
		if p.y < 0. { code |= TOP; } else if p.y > h { code |= BOTTOM; }
		code
	}

	fn clip_line_viewport(mut a: Vec2f, mut b: Vec2f, w: f32, h: f32) -> Option<(Vec2f, Vec2f)> {
		let mut out_a = compute_outcode(a, w, h);
		let mut out_b = compute_outcode(b, w, h);
		loop {
			if (out_a | out_b) == 0 { return Some((a, b)); }
			if (out_a & out_b) != 0 { return None; }
			let out = if out_a != 0 { out_a } else { out_b };
			let mut x = 0.;
			let mut y = 0.;
			if (out & TOP) != 0 {
				x = a.x + (b.x - a.x) * (0. - a.y) / (b.y - a.y);
				y = 0.;
			} else if (out & BOTTOM) != 0 {
				x = a.x + (b.x - a.x) * (h - a.y) / (b.y - a.y);
				y = h;
			} else if (out & RIGHT) != 0 {
				y = a.y + (b.y - a.y) * (w - a.x) / (b.x - a.x);
				x = w;
			} else if (out & LEFT) != 0 {
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

	fn draw_line(
		buffer: &mut FrameBuffer,
		width: u32,
		height: u32,
		a: Vec2f,
		b: Vec2f,
	) {
		let mut x0 = a.x as i32;
		let mut y0 = a.y as i32;
		let x1 = b.x as i32;
		let y1 = b.y as i32;
		let dx = (x1 - x0).abs();
		let dy = -(y1 - y0).abs();
		let sx = if x0 < x1 { 1 } else { -1 };
		let sy = if y0 < y1 { 1 } else { -1 };
		let mut err = dx + dy;
		loop {
			if x0 >= 0 && x0 < width as i32 && y0 >= 0 && y0 < height as i32 {
				buffer[(x0 as u32, y0 as u32)] = WHITE.0;
			}
			if x0 == x1 && y0 == y1 { break; }
			let e2 = 2 * err;
			if e2 >= dy {
				err += dy;
				x0 += sx;
			}
			if e2 <= dx {
				err += dx;
				y0 += sy;
			}
		}
	}


	// let lines: Vec<(Vec3f, Vec3f)> = vec![
	// 	(vec3![0,0,0], vec3![1,0,0]),
	// 	(vec3![0,0,0], vec3![0,1,0]),
	// 	(vec3![0,0,0], vec3![0,0,1]),
	// 	(vec3![1,0,0], vec3![0,1,0]),
	// 	(vec3![1,0,0], vec3![0,0,1]),
	// 	(vec3![0,1,0], vec3![0,0,1]),
	// ];

	// let lines: Vec<(Vec3f, Vec3f)> = {
	// 	use teapot::*;
	// 	let lines = VERTICES.chunks(9)
	// 		.flat_map(|coords| {
	// 			let [ax,ay,az, bx,by,bz, cx,cy,cz] = *coords else { unreachable!() };
	// 			let a = Vec3f::new(ax,ay,az);
	// 			let b = Vec3f::new(bx,by,bz);
	// 			let c = Vec3f::new(cx,cy,cz);
	// 			[ (a, b), (b, c), (c, a) ]
	// 		})
	// 		.collect();
	// 	lines
	// };

	let lines: Vec<(Vec3f, Vec3f)> = {
		let mut lines = vec![];
		const N: i32 = 30;
		for x in -N ..= N {
			for z in -N ..= N {
				let x = x as float;
				let z = z as float;
				let a = vec3xz!(x-0.5, z-0.5);
				let b = vec3xz!(x+0.5, z-0.5);
				let c = vec3xz!(x-0.5, z+0.5);
				lines.push((a, b));
				lines.push((b, c));
				lines.push((c, a));
			}
		}
		for x in -N ..= N {
			let z = N;
			let x = x as float;
			let z = z as float;
			let a = vec3xz!(x+0.5, z+0.5);
			let b = vec3xz!(x-0.5, z+0.5);
			lines.push((a, b));
		}
		for z in -N ..= N {
			let x = N;
			let x = x as float;
			let z = z as float;
			let a = vec3xz!(x+0.5, z+0.5);
			let b = vec3xz!(x+0.5, z-0.5);
			lines.push((a, b));
		}
		for line in lines.iter_mut() {
			for p in [&mut line.0, &mut line.1].iter_mut() {
				p.y += 2. * ln(0.2*(p.x*p.x+p.z*p.z));
			}
		}
		lines
	};

	let mut camera = Camera {
		pos: vec3![0., 0.7, -2.],
		forward: vec3![0., 0., 1.],
		up: vec3![0., 1., 0.],
		fov: 90. * DEG_TO_RAD,
	};



	#[allow(unused_variables)]
	let mut frame_n: u64 = 0;
	let mut is_paused: bool = false;
	// let mut scale: u32 = 1;
	// let mut lorenz_attractor = LorenzAttractor::new();//.offset_params(0.01, -0.01, 0.001);
	// let mut last_points: Vec<Vec3f> = vec![];
	// let mut zqqx_lang = ZqqxLang::new();



	while window.is_open() && !window.is_key_down(Key::Escape) {
		let frame_begin_timestamp = SystemTime::now();
		let mut is_redraw_needed: bool = frame_n == 0;

		// handle resizing
		if buffer.resize(window.get_size()) {
			//if verbose { println!("Resized to {w}x{h}") }
			is_redraw_needed = true;
		}

		// handle inputs
		if window.is_key_pressed_once(Key::Space) {
			is_paused = !is_paused;
		}
		// if window.is_key_pressed_repeat(Key::I) {
		// 	if scale > 1 {
		// 		scale -= 1;
		// 		is_redraw_needed = true;
		// 	}
		// }
		// if window.is_key_pressed_repeat(Key::O) {
		// 	scale += 1;
		// 	is_redraw_needed = true;
		// }

		const DELTA: float = 0.01; // TODO
		const MOVE_SPEED: float = 20.;
		const ROTATION_SPEED: float = 3.;
		if window.is_key_down(Key::Up) {
			camera.pos += camera.forward * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::Down) {
			camera.pos -= camera.forward * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::Left) {
			camera.pos -= camera.basis().0 * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::Right) {
			camera.pos += camera.basis().0 * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::Z) {
			camera.pos += vec3y!(1) * DELTA * MOVE_SPEED;
			// camera.pos += camera.basis().1 * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::X) {
			camera.pos -= vec3y!(1) * DELTA * MOVE_SPEED;
			// camera.pos -= camera.basis().1 * DELTA * MOVE_SPEED;
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::W) {
			camera.forward += camera.basis().1 * DELTA * ROTATION_SPEED;
			camera.forward.normlize();
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::S) {
			camera.forward -= camera.basis().1 * DELTA * ROTATION_SPEED;
			camera.forward.normlize();
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::A) {
			camera.forward -= camera.basis().0 * DELTA * ROTATION_SPEED;
			camera.forward.normlize();
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::D) {
			camera.forward += camera.basis().0 * DELTA * ROTATION_SPEED;
			camera.forward.normlize();
			is_redraw_needed = true;
		}
		// if window.is_key_down(Key::Q) {
		// 	camera.up -= camera.basis().0 * DELTA * ROTATION_SPEED;
		// 	camera.up.normlize();
		// 	is_redraw_needed = true;
		// }
		// if window.is_key_down(Key::E) {
		// 	camera.up += camera.basis().0 * DELTA * ROTATION_SPEED;
		// 	camera.up.normlize();
		// 	is_redraw_needed = true;
		// }
		if window.is_key_down(Key::R) {
			camera.up = vec3y!(1.);
			is_redraw_needed = true;
		}
		const MIN_FOV: float = 0.01 * PI;
		const MAX_FOV: float = 0.9 * PI;
		const FOV_RANGE: float = MAX_FOV - MIN_FOV;
		if window.is_key_down(Key::I) {
			// camera.fov -= DELTA;
			camera.fov = MIN_FOV + FOV_RANGE * sigmoid(asigmoid((camera.fov-MIN_FOV)/FOV_RANGE) - 0.03);
			is_redraw_needed = true;
		}
		if window.is_key_down(Key::O) {
			// camera.fov += DELTA;
			camera.fov = MIN_FOV + FOV_RANGE * sigmoid(asigmoid((camera.fov-MIN_FOV)/FOV_RANGE) + 0.03);
			is_redraw_needed = true;
		}

		// if !is_paused {
		// 	lorenz_attractor.step(1e-2);
		// 	is_redraw_needed = true;
		// }

		// render new frame
		if is_redraw_needed {
			frame_n += 1;
			buffer.clear();

			// dbg!(&camera);

			for line in lines.iter() {
				if let Some((a, b)) = camera.project_line(line, buffer.wf(), buffer.hf(), 0.1) {
					let (w, h) = buffer.wh();
					draw_line(&mut buffer, w, h, a, b);
				}
			}

			let text_size = 4;

			buffer.render_text(&format!("XYZ: {:.3}, {:.3}, {:.3}", camera.pos.x, camera.pos.y, camera.pos.z), (5,5), GRAY, text_size);
			buffer.render_text(&format!("FOV: {:.3}", camera.fov * RAD_TO_DEG), (5,40), GRAY, text_size);

			// buffer.render_text(
			// 	&format!("LX: {}", lorenz_attractor.x),
			// 	(0, 10),
			// 	RED,
			// 	3,
			// );
			// buffer.render_text(
			// 	&format!("LY: {}", lorenz_attractor.y),
			// 	(0, 40),
			// 	GREEN,
			// 	3,
			// );
			// buffer.render_text(
			// 	&format!("LZ: {}", lorenz_attractor.z),
			// 	(0, 70),
			// 	BLUE,
			// 	3,
			// );

			// last_points.push(lorenz_attractor.get_xyz_as_vec3d());
			//
			// const SCALE: float = 5.;
			// let (wf, hf) = buffer.whf();
			//
			// // projection 1: drop X
			// for v in last_points.iter() {
			// 	let mut v = *v;
			// 	v *= SCALE;
			// 	let mut v = v.yz();
			// 	v += Vec2d::new(wf, hf) / 4.;
			// 	let Vec2d { x, y } = v;
			// 	let (x, y) = (x as u32, y as u32);
			// 	buffer[(x, y)] = WHITE.0;
			// }
			//
			// // projection 2: drop Y
			// for v in last_points.iter() {
			// 	let mut v = *v;
			// 	v *= SCALE;
			// 	let mut v = v.xz();
			// 	v += Vec2d::new(wf*3., hf) / 4.;
			// 	let Vec2d { x, y } = v;
			// 	let (x, y) = (x as u32, y as u32);
			// 	buffer[(x, y)] = WHITE.0;
			// }
			//
			// // projection 3: drop Z
			// for v in last_points.iter() {
			// 	let mut v = *v;
			// 	v *= SCALE;
			// 	let mut v = v.xy();
			// 	v += Vec2d::new(wf, hf*3.) / 4.;
			// 	let Vec2d { x, y } = v;
			// 	let (x, y) = (x as u32, y as u32);
			// 	buffer[(x, y)] = WHITE.0;
			// }
			//
			// // projection 4
			// for v in last_points.iter() {
			// 	let mut v = *v;
			// 	v *= SCALE;
			// 	let mut v = v.project_2d(vec3![1,1,0].normed(), vec3![0,-1,-1].normed());
			// 	v += Vec2d::new(wf*3., hf*3.) / 4.;
			// 	let Vec2d { x, y } = v;
			// 	let (x, y) = (x as u32, y as u32);
			// 	buffer[(x, y)] = WHITE.0;
			// }
			//
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
			let frametime = frame_end_timestamp.duration_since(frame_begin_timestamp).unwrap();
			let fps_text = format!("\"FPS\": {:.1}", 1. / frametime.as_secs_f64());
			buffer.render_text(&fps_text, (buffer.w as i32 - 5 - (fps_text.len() as i32) * (text_size as i32) * 6, 5), GRAY, text_size);

			window.update_with_my_buffer(&buffer);
		} // end of render
		else {
			window.update();
		}
	} // end of main loop
}

const UNABLE_TO_UPDATE_WINDOW_BUFFER: &str = "unable to update window buffer";













#[allow(non_camel_case_types)]
type float = f32;



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


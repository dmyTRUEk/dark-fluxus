//! game of life

use std::collections::HashSet;

use either::{Either, IntoEither};
use glam::IVec2;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{extensions::BoolSelectEither, math::{lerp, min_max}, misc::{int_circle_spiral, int_square_spiral}, utils::hash_str_to_u64};



#[derive(Debug, Clone)]
pub struct GameOfLifeState {
	pub alive_cells: HashSet<IVec2>,
}
impl GameOfLifeState {
	pub fn from_seed(seed: &str) -> Self {
		let seed: u64 = hash_str_to_u64(seed);
		let mut rng = StdRng::seed_from_u64(seed);

		let ratio: f32 = (seed as f32) / (u64::MAX as f32);
		// debug_assert!(0. <= ratio && ratio <= 1.);
		let density_1 = rng.random_range(0. ..= ratio);
		let density_2 = rng.random_range(ratio ..= 1.);
		let (density_min, density_max) = min_max(density_1, density_2);
		let mut density = lerp(ratio, density_min, density_max);
		// debug_assert!(0. <= density && density <= 1.);

		let radius: u32 = 1 + ((seed.count_ones() as f32) / ratio).round() as u32;
		fn norm((x, y): (i32, i32)) -> u32 { x.unsigned_abs() + y.unsigned_abs() }

		let mut alive_cells = HashSet::new();
		let iter = (rng.random_range(0. ..= 1.) < 0.5).select_either(
			int_square_spiral(),
			int_circle_spiral()
		);
		for p in iter {
			if rng.random_range(0. ..= 1.) < density {
				let _inserted = alive_cells.insert(p.into());
				// debug_assert!(inserted); // works only for square spiral
			}
			let is_out_of_radius = norm(p) > radius;
			if is_out_of_radius {
				let is_rng = rng.random_range(0. ..= 1.) < ratio / (density * (radius.pow(3) as f32));
				if is_rng {
					break
				} else {
					density *= 0.99999;
				}
			}
		}
		Self { alive_cells }
	}

	pub fn update(&mut self) {
		*self = self.updated();
	}

	pub fn updated(&self) -> Self {
		fn around(IVec2 { x, y }: IVec2) -> [IVec2; 8] {
			[
				// IVec2::new(x, y),
				IVec2::new(x, y-1),
				IVec2::new(x-1, y),
				IVec2::new(x-1, y-1),
				IVec2::new(x, y+1),
				IVec2::new(x+1, y),
				IVec2::new(x+1, y+1),
				IVec2::new(x+1, y-1),
				IVec2::new(x-1, y+1),
			]
		}
		fn around_and_self(IVec2 { x, y }: IVec2) -> [IVec2; 9] {
			// around(p).pushed(p)
			[
				IVec2::new(x, y),
				IVec2::new(x, y-1),
				IVec2::new(x-1, y),
				IVec2::new(x-1, y-1),
				IVec2::new(x, y+1),
				IVec2::new(x+1, y),
				IVec2::new(x+1, y+1),
				IVec2::new(x+1, y-1),
				IVec2::new(x-1, y+1),
			]
		}
		let old_alive_cells = &self.alive_cells;
		let queue: HashSet<IVec2> = HashSet::from_iter(
			old_alive_cells.iter().cloned().flat_map(around_and_self)
		);
		let mut new_alive_cells = HashSet::new();
		for p in queue {
			let mut neighbors_n = 0;
			for neighbor in around(p) {
				if old_alive_cells.contains(&neighbor) {
					neighbors_n += 1;
				}
			}
			match neighbors_n {
				0..=1 => {}
				2 if old_alive_cells.contains(&p) => { let _ = new_alive_cells.insert(p); }
				3 => { let _ = new_alive_cells.insert(p); }
				_ => {}
			}
		}
		Self { alive_cells: new_alive_cells }
	}
}


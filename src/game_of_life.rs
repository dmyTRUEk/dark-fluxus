//! game of life

use std::collections::HashSet;

use glam::IVec2;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{math::{lerp, min_max}, math_aliases::log2, misc::ispiral, utils::hash_str_to_u64};



#[derive(Debug, Clone)]
pub struct GameOfLifeState {
	pub alive_cells: HashSet<IVec2>,
}
impl GameOfLifeState {
	pub fn from_seed(seed: &str) -> Self {
		let seed: u64 = hash_str_to_u64(seed);
		let mut rng = StdRng::seed_from_u64(seed);

		let ratio: f32 = (seed as f32) / (u64::MAX as f32);
		let density_1 = rng.random_range(0. ..= ratio);
		let density_2 = rng.random_range(ratio ..= 1.);
		let (density_min, density_max) = min_max(density_1, density_2);
		let density = lerp(ratio, density_min, density_max);

		let mut n: u32 = 1 + (log2(seed as f32) / ratio).round() as u32;

		let mut alive_cells = HashSet::new();
		for p in ispiral() {
			if rng.random_range(0. ..= 1.) < density {
				let inserted = alive_cells.insert(p.into());
				debug_assert!(inserted);
				n -= 1;
			}
			if n == 0 { break }
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


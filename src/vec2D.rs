//! rust vec 2d

use std::ops::{Index, IndexMut};



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2D<T> {
	x_size: u32,
	y_size: u32,
	elems: Vec<T>,
}

impl<T> Vec2D<T> {
	pub fn new() -> Self {
		Self { x_size: 0, y_size: 0, elems: Vec::new() }
	}
	pub fn from_fn(x_size: u32, y_size: u32, mut f: impl FnMut(u32, u32) -> T) -> Self {
		Self {
			x_size, y_size,
			elems: (0..y_size).map(|y| {
				(0..x_size).map(|x| f(x, y)).collect()
			}).collect::<Vec<Vec<T>>>()
			.into_iter().flatten().collect()
		}
	}

	fn xy_to_i(&self, x: u32, y: u32) -> usize {
		xy_to_i(x, y, self.x_size)
	}

	pub fn extend_x(&mut self, _dx: u32) { todo!() }
	pub fn extend_y(&mut self, _dy: u32) { todo!() }

	pub fn iter(&self) -> impl Iterator<Item = (u32, u32, &T)> {
		self.elems.iter()
			.enumerate()
			.map(|(i, elem)| {
				let i = i as u32;
				let x = i % self.x_size;
				let y = i / self.x_size;
				(x, y, elem)
			})
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut T)> {
		self.elems.iter_mut()
			.enumerate()
			.map(|(i, elem)| {
				let i = i as u32;
				let x = i % self.x_size;
				let y = i / self.x_size;
				(x, y, elem)
			})
	}
	pub fn iter_around_wrapping(&self, x: i32, y: i32, r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, bool, bool, &T)> {
		(-(r as i32) ..= (r as i32)).flat_map(move |dy| {
			let y_global = (y + dy).rem_euclid(self.y_size as i32) as u32;
			(-(r as i32) ..= (r as i32)).map(move |dx| {
				let x_global = (x + dx).rem_euclid(self.x_size as i32) as u32;
				(dx, dy, x_global, y_global, false, false, &self[(x_global,y_global)])
			})
		})
	}
	// pub fn iter_mut_around_wrapping(&mut self, x: i32, y: i32, r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, &mut T)> {
	// 	(-(r as i32) ..= (r as i32)).flat_map(move |dy| {
	// 		let y_global = (y + dy).rem_euclid(self.y_size as i32) as u32;
	// 		(-(r as i32) ..= (r as i32)).map(move |dx| {
	// 			let x_global = (x + dx).rem_euclid(self.x_size as i32) as u32;
	// 			(dx, dy, x_global, y_global, &mut self[(x_global,y_global)])
	// 		})
	// 	})
	// }
	pub fn iter_around_wrapping_alt(&self, x: i32, y: i32, r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, bool, bool, &T)> {
		// how it should work:
		// l k j i   d c b a   l k j i
		// h g f e   h g f e   h g f e
		// d c b a   l k j i   d c b a
		//
		// i j k l   A B C D   i j k l
		// e f g h   E F G H   e f g h
		// a b c d   I J K L   a b c d
		//
		// l k j i   d c b a   l k j i
		// h g f e   h g f e   h g f e
		// d c b a   l k j i   d c b a
		(-(r as i32) ..= (r as i32)).flat_map(move |dy| {
			let y_total = y + dy;
			let tile_y = y_total.div_euclid(self.y_size as i32);
			let y_global = y_total.rem_euclid(self.y_size as i32) as u32;
			(-(r as i32) ..= (r as i32)).map(move |dx| {
				let x_total = x + dx;
				let tile_x = x_total.div_euclid(self.x_size as i32);
				let mut x_global = x_total.rem_euclid(self.x_size as i32) as u32;
				let mut y_global = y_global;
				let is_y_flipped = tile_x % 2 != 0;
				if is_y_flipped {
					y_global = self.y_size - y_global - 1;
				}
				let is_x_flipped = tile_y % 2 != 0;
				if is_x_flipped {
					x_global = self.x_size - x_global - 1;
				}
				(dx, dy, x_global, y_global, is_x_flipped, is_y_flipped, &self[(x_global,y_global)])
			})
		})
	}
	// pub fn iter_mut_around_wrapping_alt(&mut self, x: i32, y: i32, r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, &mut T)> { todo!() }
}

fn xy_to_i(x: u32, y: u32, x_size: u32) -> usize {
	(x as usize) + (y as usize) * (x_size as usize)
}

impl<T> Vec2D<T> where T: Clone {
	pub fn from_elem(x_size: u32, y_size: u32, elem: T) -> Self {
		Self { x_size, y_size, elems: vec![elem; (x_size as usize) * (y_size as usize)] }
	}
	pub fn from_vec_of_vecs(_elems: Vec<Vec<T>>) -> Self { todo!() }
}

impl<T> Index<(u32, u32)> for Vec2D<T> {
	type Output = T;
	fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
		&self.elems[self.xy_to_i(x, y)]
	}
}
impl<T> IndexMut<(u32, u32)> for Vec2D<T> {
	fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
		let i = self.xy_to_i(x, y);
		&mut self.elems[i]
	}
}





#[cfg(test)]
mod tests {
	#![allow(non_snake_case)]
	#![deny(dead_code)]

	use super::*;

	mod from_elem {
		use super::*;
		#[test]
		fn _2_3() {
			assert_eq!(
				Vec2D { x_size: 2, y_size: 3, elems: vec!['a','a','a', 'a','a','a'] },
				Vec2D::from_elem(2, 3, 'a')
			)
		}
	}

	mod from_fn {
		use super::*;
		#[test]
		fn _3_2__sum() {
			assert_eq!(
				Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,2,3] },
				Vec2D::from_fn(3, 2, |x, y| x + y)
			)
		}
		#[test]
		fn _3_4__pow() {
			assert_eq!(
				Vec2D { x_size: 3, y_size: 4, elems: vec![1,0,0, 1,1,1, 1,2,4, 1,3,9] },
				Vec2D::from_fn(3, 4, |x, y| y.pow(x))
			)
		}
	}

	mod index {
		use super::*;
		#[test]
		fn _3_2__1_1() {
			assert_eq!(
				2,
				Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,2,3] }[(1,1)]
			)
		}
	}

	mod index_mut {
		use super::*;
		#[test]
		fn _3_2__1_1__42() {
			let mut vec2d = Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,2,3] };
			vec2d[(1,1)] = 42;
			assert_eq!(
				Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,42,3] },
				vec2d
			)
		}
	}

	mod iter_around_wrapping {
		#![allow(non_upper_case_globals)]
		use super::*;
		const a: char = 'a'; const b: char = 'b'; const c: char = 'c'; const d: char = 'd';
		const e: char = 'e'; const f: char = 'f'; const g: char = 'g'; const h: char = 'h';
		const i: char = 'i'; const j: char = 'j'; const k: char = 'k'; const l: char = 'l';
		mod r_0 {
			use super::*;
			const R: u32 = 0;
			#[test]
			fn _0_0() {
				assert_eq!(
					vec![a],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(0, 0, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_2() {
				assert_eq!(
					vec![j],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(1, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_1() {
				assert_eq!(
					vec![h],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(3, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _4_1() {
				assert_eq!(
					vec![e],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(4, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_1() {
				assert_eq!(
					vec![f],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(5, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_2() {
				assert_eq!(
					vec![j],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(5, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_3() {
				assert_eq!(
					vec![b],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(5, 3, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_3() {
				assert_eq!(
					vec![b],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(1, 3, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
		}
		mod r_1 {
			use super::*;
			const R: u32 = 1;
			#[test]
			fn _2_1() {
				assert_eq!(
					vec![
						b,c,d,
						f,g,h,
						j,k,l,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(2, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_1() {
				assert_eq!(
					vec![
						c,d,a,
						g,h,e,
						k,l,i,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(3, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_2() {
				assert_eq!(
					vec![
						e,f,g,
						i,j,k,
						a,b,c,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(1, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _2_2() {
				assert_eq!(
					vec![
						f,g,h,
						j,k,l,
						b,c,d,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(2, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_2() {
				assert_eq!(
					vec![
						g,h,e,
						k,l,i,
						c,d,a,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(3, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _4_2() {
				assert_eq!(
					vec![
						h,e,f,
						l,i,j,
						d,a,b,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(4, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _0_0() {
				assert_eq!(
					vec![
						l,i,j,
						d,a,b,
						h,e,f,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping(0, 0, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
		}
	}

	mod iter_around_wrapping_alt {
		#![allow(non_upper_case_globals)]
		use super::*;
		const a: char = 'a'; const b: char = 'b'; const c: char = 'c'; const d: char = 'd';
		const e: char = 'e'; const f: char = 'f'; const g: char = 'g'; const h: char = 'h';
		const i: char = 'i'; const j: char = 'j'; const k: char = 'k'; const l: char = 'l';
		mod r_0 {
			use super::*;
			const R: u32 = 0;
			#[test]
			fn _0_0() {
				assert_eq!(
					vec![a],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(0, 0, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_2() {
				assert_eq!(
					vec![j],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(1, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_1() {
				assert_eq!(
					vec![h],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(3, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _4_1() {
				assert_eq!(
					vec![e],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(4, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_1() {
				assert_eq!(
					vec![f],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(5, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_2() {
				assert_eq!(
					vec![b],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(5, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _5_3() {
				assert_eq!(
					vec![k],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(5, 3, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_3() {
				assert_eq!(
					vec![c],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(1, 3, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
		}
		mod r_1 {
			use super::*;
			const R: u32 = 1;
			#[test]
			fn _2_1() {
				assert_eq!(
					vec![
						b,c,d,
						f,g,h,
						j,k,l,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(2, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_1() {
				assert_eq!(
					vec![
						c,d,i,
						g,h,e,
						k,l,a,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(3, 1, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _1_2() {
				assert_eq!(
					vec![
						e,f,g,
						i,j,k,
						d,c,b,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(1, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _2_2() {
				assert_eq!(
					vec![
						f,g,h,
						j,k,l,
						c,b,a,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(2, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _3_2() {
				assert_eq!(
					vec![
						g,h,e,
						k,l,a,
						b,a,l,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(3, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _4_2() {
				assert_eq!(
					vec![
						h,e,f,
						l,a,b,
						a,l,k,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(4, 2, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
			#[test]
			fn _0_0() {
				assert_eq!(
					vec![
						a,l,k,
						l,a,b,
						h,e,f,
					],
					Vec2D {
						x_size: 4,
						y_size: 3,
						elems: vec![
							a,b,c,d,
							e,f,g,h,
							i,j,k,l,
						]
					}.iter_around_wrapping_alt(0, 0, R)
					.map(|(.., el)| *el).collect::<Vec<_>>()
				)
			}
		}
	}
}


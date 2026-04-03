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
	pub fn from_fn(x_size: u32, y_size: u32, f: fn(u32, u32) -> T) -> Self {
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
	pub fn iter_around_wrapping(&self, x: i32, y: i32, r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, &T)> {
		(-(r as i32) ..= (r as i32)).map(move |dy| {
			(-(r as i32) ..= (r as i32)).map(move |dx| {
				let x_global = (x + dx).rem_euclid(self.x_size as i32) as u32;
				let y_global = (y + dy).rem_euclid(self.y_size as i32) as u32;
				(dx, dy, x_global, y_global, &self[(x_global,y_global)])
			})
		}).flatten()
	}
	// pub fn iter_around_wrapping_complex(&self, _x: u32, _y: u32, _r: u32) -> impl Iterator<Item = (i32, i32, u32, u32, &T)> { todo!() }
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

	#[test]
	fn from_elem_2_3() {
		assert_eq!(
			Vec2D { x_size: 2, y_size: 3, elems: vec!['a','a','a', 'a','a','a'] },
			Vec2D::from_elem(2, 3, 'a')
		)
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

	#[test]
	fn index_3_2__1_1() {
		assert_eq!(
			2,
			Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,2,3] }[(1,1)]
		)
	}

	#[test]
	fn index_mut_3_2__1_1__42() {
		let mut vec2d = Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,2,3] };
		vec2d[(1,1)] = 42;
		assert_eq!(
			Vec2D { x_size: 3, y_size: 2, elems: vec![0,1,2, 1,42,3] },
			vec2d
		)
	}
}


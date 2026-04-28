//! misc

use std::f32::consts::TAU;

use crate::math_aliases::{cos, sin};



pub fn int_square_spiral() -> impl Iterator<Item=(i32,i32)> {
	let mut x = 0;
	let mut y = 0;
	let mut dx = 1;
	let mut dy = 0;
	let mut segment_len = 1;
	let mut steps_left = 1;
	let mut segments_passed = 0;
	let mut first = true;
	std::iter::from_fn(move || {
		if first {
			first = false;
			return Some((x, y));
		}
		// move
		x += dx;
		y += dy;
		steps_left -= 1;
		// end of segment => rotate
		if steps_left == 0 {
			// rotate 90deg CCW
			let tmp = dx;
			dx = -dy;
			dy = tmp;
			segments_passed += 1;
			// increase length every 2 segments
			if segments_passed % 2 == 0 {
				segment_len += 1;
			}
			steps_left = segment_len;
		}
		Some((x, y))
	})
}

#[test]
fn int_square_spiral_() {
	assert_eq!(
		[(0,0), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1), (2,-1), (2,0), (2,1), (2,2), (1,2), (0,2), (-1,2), (-2,2)].to_vec(),
		int_square_spiral().take(17).collect::<Vec<(i32, i32)>>()
	)
}



pub fn int_circle_spiral() -> impl Iterator<Item=(i32,i32)> {
	let mut radius = 0;
	let mut i = 0;
	fn angle_from_i(i: u32, radius: u32) -> f32 { (i as f32) / (radius as f32) }
	std::iter::from_fn(move || {
		// eprintln!("radius={radius}, i={i:.2}");
		//let circ = TAU * (radius as f32);
		//let num_of_points = circ;
		//let angle = TAU * (i as f32) / num_of_points;
		// equivalent:
		let mut angle = angle_from_i(i, radius);
		if angle > TAU {
			radius += 1;
			i = 0;
			angle = angle_from_i(i, radius);
		}
		i += 1;
		// dbg!(angle);
		let x = (radius as f32) * cos(angle);
		let y = (radius as f32) * sin(angle);
		let x = x.round() as i32;
		let y = y.round() as i32;
		Some((x, y))
	})
}

// #[test]
// fn int_circle_spiral_() {
// 	assert_eq!(
// 		[(0,0), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1), (2,-1), (2,0), (2,1), (2,2), (1,2), (0,2), (-1,2), (-2,2)].to_vec(),
// 		int_circle_spiral().take(17).collect::<Vec<(i32, i32)>>()
// 	)
// }



pub const ALPHABET_STR_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHABET_STR_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABET_LOWERCASE: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
pub const ALPHABET_UPPERCASE: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

pub fn string_from_number_u64(mut n: u64, chars: &[char]) -> String {
	if n == 0 { return "".to_string() }
	let mut res = String::new();
	loop {
		n -= 1;
		res.push(chars[(n as usize) % chars.len()]);
		n /= chars.len() as u64;
		if n == 0 { break }
	}
	res.chars().rev().collect()
}

#[cfg(test)]
mod string_from_number_u64 {
	use super::{ALPHABET_LOWERCASE, string_from_number_u64 as string_from_number};
	mod alphabet_lowercase {
		use super::*;
		const CHARS: &[char] = &ALPHABET_LOWERCASE;
		#[test] fn _0() { assert_eq!("", string_from_number(0, CHARS)) }
		#[test] fn _1() { assert_eq!("a", string_from_number(1, CHARS)) }
		#[test] fn _2() { assert_eq!("b", string_from_number(2, CHARS)) }
		#[test] fn _3() { assert_eq!("c", string_from_number(3, CHARS)) }
		#[test] fn _4() { assert_eq!("d", string_from_number(4, CHARS)) }
		#[test] fn _5() { assert_eq!("e", string_from_number(5, CHARS)) }
		#[test] fn _6() { assert_eq!("f", string_from_number(6, CHARS)) }
		#[test] fn _7() { assert_eq!("g", string_from_number(7, CHARS)) }
		#[test] fn _8() { assert_eq!("h", string_from_number(8, CHARS)) }
		#[test] fn _9() { assert_eq!("i", string_from_number(9, CHARS)) }
		#[test] fn _10() { assert_eq!("j", string_from_number(10, CHARS)) }
		#[test] fn _11() { assert_eq!("k", string_from_number(11, CHARS)) }
		#[test] fn _12() { assert_eq!("l", string_from_number(12, CHARS)) }
		#[test] fn _13() { assert_eq!("m", string_from_number(13, CHARS)) }
		#[test] fn _14() { assert_eq!("n", string_from_number(14, CHARS)) }
		#[test] fn _15() { assert_eq!("o", string_from_number(15, CHARS)) }
		#[test] fn _16() { assert_eq!("p", string_from_number(16, CHARS)) }
		#[test] fn _17() { assert_eq!("q", string_from_number(17, CHARS)) }
		#[test] fn _18() { assert_eq!("r", string_from_number(18, CHARS)) }
		#[test] fn _19() { assert_eq!("s", string_from_number(19, CHARS)) }
		#[test] fn _20() { assert_eq!("t", string_from_number(20, CHARS)) }
		#[test] fn _21() { assert_eq!("u", string_from_number(21, CHARS)) }
		#[test] fn _22() { assert_eq!("v", string_from_number(22, CHARS)) }
		#[test] fn _23() { assert_eq!("w", string_from_number(23, CHARS)) }
		#[test] fn _24() { assert_eq!("x", string_from_number(24, CHARS)) }
		#[test] fn _25() { assert_eq!("y", string_from_number(25, CHARS)) }
		#[test] fn _26() { assert_eq!("z", string_from_number(26, CHARS)) }
		#[test] fn _27() { assert_eq!("aa", string_from_number(27, CHARS)) }
		#[test] fn _28() { assert_eq!("ab", string_from_number(28, CHARS)) }
		#[test] fn _29() { assert_eq!("ac", string_from_number(29, CHARS)) }
		#[test] fn _30() { assert_eq!("ad", string_from_number(30, CHARS)) }
	}
}


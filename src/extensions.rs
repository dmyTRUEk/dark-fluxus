//! extensions

use either::Either;



pub trait IndexOfMaxMin<T> {
	fn index_of_max(&self) -> Option<usize>;
	fn index_of_min(&self) -> Option<usize>;
}
impl<T: PartialOrd> IndexOfMaxMin<T> for Vec<T> {
	fn index_of_max(&self) -> Option<usize> {
		let mut option_index_of_max = None;
		for i in 0..self.len() {
			match option_index_of_max {
				None => {
					option_index_of_max = Some(i);
				}
				Some(index_of_max) if self[i] > self[index_of_max] => {
					option_index_of_max = Some(i);
				}
				_ => {}
			}
		}
		option_index_of_max
	}
	fn index_of_min(&self) -> Option<usize> {
		let mut option_index_of_min = None;
		for i in 0..self.len() {
			match option_index_of_min {
				None => {
					option_index_of_min = Some(i);
				}
				Some(index_of_min) if self[i] < self[index_of_min] => {
					option_index_of_min = Some(i);
				}
				_ => {}
			}
		}
		option_index_of_min
	}
}



pub trait Into_<T> {
	fn into_(self) -> T;
}
impl<T> Into_<T> for T { fn into_(self) -> T { self } }
// impl<T, S: Into<T>> MyInto<T> for S { fn into_(self) -> T { self.into() } }
impl Into_<f32> for i32 { fn into_(self) -> f32 { self as f32 } }



pub trait AddSubModulo {
	fn add_mod(self, n: Self, modulo: Self) -> Self;
	fn sub_mod(self, n: Self, modulo: Self) -> Self;
	fn inc_mod(self, modulo: Self) -> Self;
	fn dec_mod(self, modulo: Self) -> Self;
}
impl AddSubModulo for u32 {
	fn add_mod(self, n: Self, modulo: Self) -> Self {
		(self + n).rem_euclid(modulo)
	}
	fn sub_mod(self, n: Self, modulo: Self) -> Self {
		((self as i32) - (n as i32)).rem_euclid(modulo as i32) as u32
	}
	fn inc_mod(self, modulo: Self) -> Self {
		self.add_mod(1, modulo)
	}
	fn dec_mod(self, modulo: Self) -> Self {
		self.sub_mod(1, modulo)
	}
}



pub trait BoolSelect<T> {
	fn select(self, true_val: T, false_val: T) -> T;
}
impl<T> BoolSelect<T> for bool {
	fn select(self, true_val: T, false_val: T) -> T {
		if self { true_val } else { false_val }
	}
}

pub trait BoolSelectEither<T, U> {
	fn select_either(self, true_val: T, false_val: U) -> Either<T, U>;
}
impl<T, U> BoolSelectEither<T, U> for bool {
	fn select_either(self, true_val: T, false_val: U) -> Either<T, U> {
		if self {
			Either::Left(true_val)
		} else {
			Either::Right(false_val)
		}
	}
}



pub trait Flatten<T, const L: usize> {
	fn flatten_(self) -> [T; L];
}
impl<T, const N: usize, const M: usize> Flatten<T, {N*M}> for [[T; N]; M] where T: Sized {
	fn flatten_(self) -> [T; N * M] {
		// src: https://stackoverflow.com/questions/76573089/is-flattening-arrays-by-memtransmute-safe
		unsafe {
			// std::mem::transmute::<[[T; N]; M], [T; N*M]>(self)
			// src: chatgpt
			let ptr = &self as *const _ as *const [T; N * M];
			ptr.read()
		}
	}
}



// pub trait ArrayPushExtend<T, const N: usize, const M: usize> {
// 	fn pushed(self, el: T) -> [T; N+1];
// 	fn extend(self, other: [T; M]) -> [T; N+M];
// }
// impl<T, const N: usize, const M: usize> ArrayPushExtend<T, N, M> for [T; N] where T: Copy {
// 	fn pushed(self, el: T) -> [T; N+1] {
// 		let mut out = [el; N + 1]; // temporary init
// 		let mut i = 0;
// 		while i < N {
// 			out[i] = self[i];
// 			i += 1;
// 		}
// 		out[N] = el;
// 		out
// 	}
// 	fn extend(self, _other: [T; M]) -> [T; N+M] {
// 		todo!()
// 	}
// }



pub trait Extended<T> {
	fn extended(self, other: impl IntoIterator<Item=T>) -> Self;
}
impl<T> Extended<T> for Vec<T> {
	fn extended(mut self, other: impl IntoIterator<Item=T>) -> Self {
		self.extend(other);
		self
	}
}



pub trait Reversed {
	fn reversed(self) -> Self;
}
impl<T> Reversed for Vec<T> {
	fn reversed(mut self) -> Self {
		self.reverse();
		self
	}
}
impl Reversed for String {
	fn reversed(mut self) -> Self {
		debug_assert!(self.is_ascii());
		unsafe { // SAFETY: is ascii by assert above
			self.as_bytes_mut().reverse();
		}
		self
	}
}



pub trait FloatToStringPretty {
	fn to_string_pretty(self, frac_digits: u8) -> String;
}
impl FloatToStringPretty for f64 {
	fn to_string_pretty(mut self, frac_digits: u8) -> String {
		if !self.is_finite() {
			return if self.is_nan() {
				format!("NAN")
			} else if self > 0. {
				format!("INF")
			} else if self < 0. {
				format!("-INF")
			} else {
				unreachable!()
			}
		}
		let is_neg = self.is_sign_negative(); // TODO(fix)?: -NaN
		self = self.abs();
		let (mut frac_str, exp_str) = if self.abs() < 10e15 {
			(format!("{self:.*}", frac_digits as usize), None)
		} else {
			let s = format!("{self:.*E}", frac_digits as usize);
			let (f, e) = s.split_once('E').unwrap();
			(f.to_string(), Some(e.to_string()))
		};
		if !frac_str.contains('.') {
			frac_str += ".";
		}
		debug_assert_eq!(1, frac_str.bytes().filter(|&c| c == b'.').count());
		debug_assert!(frac_str.is_ascii());
		let (frac_left_before, frac_right_before) = frac_str.split_once('.').unwrap();
		let frac_left_after: String = frac_left_before
			.bytes()
			.rev()
			.collect::<Vec<u8>>()
			.chunks(3)
			.map(|c| str::from_utf8(c).unwrap())
			.intersperse("_")
			.collect::<String>()
			.reversed();
		let frac_right_after: String = frac_right_before
			.as_bytes()
			.chunks(3)
			.map(|c| str::from_utf8(c).unwrap())
			.intersperse("_")
			.collect::<String>();
		let maybe_minus = is_neg.select("-", "");
		if let Some(exp_str) = exp_str {
			format!("{maybe_minus}{frac_left_after}.{frac_right_after}_E{exp_str}")
		} else {
			format!("{maybe_minus}{frac_left_after}.{frac_right_after}")
		}
	}
}

#[cfg(test)]
mod to_string_pretty {
	use super::*;
	use std::f64::consts::PI;

	mod pi {
		use super::*;
		const X: f64 = PI;
		#[test] fn _0() { assert_eq!("3.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("3.1", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("3.14", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("3.142", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("3.141_6", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("3.141_59", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("3.141_593", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("3.141_592_7", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("3.141_592_65", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("3.141_592_654", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("3.141_592_653_6", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("3.141_592_653_59", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("3.141_592_653_590", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("3.141_592_653_589_8", X.to_string_pretty(13)) }
		#[test] fn _14(){ assert_eq!("3.141_592_653_589_79", X.to_string_pretty(14)) }
		// only up to 15 digits
	}
	mod m_pi {
		use super::*;
		const X: f64 = -PI;
		#[test] fn _0() { assert_eq!("-3.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-3.1", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-3.14", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-3.142", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-3.141_6", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-3.141_59", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-3.141_593", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-3.141_592_7", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-3.141_592_65", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-3.141_592_654", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-3.141_592_653_6", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("-3.141_592_653_59", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("-3.141_592_653_590", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("-3.141_592_653_589_8", X.to_string_pretty(13)) }
		#[test] fn _14(){ assert_eq!("-3.141_592_653_589_79", X.to_string_pretty(14)) }
		// only up to 15 digits
	}

	mod _10_pi {
		use super::*;
		const X: f64 = 10. * PI;
		#[test] fn _0() { assert_eq!("31.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("31.4", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("31.42", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("31.416", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("31.415_9", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("31.415_93", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("31.415_927", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("31.415_926_5", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("31.415_926_54", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("31.415_926_536", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("31.415_926_535_9", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("31.415_926_535_90", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("31.415_926_535_898", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("31.415_926_535_897_9", X.to_string_pretty(13)) }
		// only up to 15 digits
	}
	mod _m10_pi {
		use super::*;
		const X: f64 = -10. * PI;
		#[test] fn _0() { assert_eq!("-31.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-31.4", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-31.42", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-31.416", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-31.415_9", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-31.415_93", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-31.415_927", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-31.415_926_5", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-31.415_926_54", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-31.415_926_536", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-31.415_926_535_9", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("-31.415_926_535_90", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("-31.415_926_535_898", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("-31.415_926_535_897_9", X.to_string_pretty(13)) }
		// only up to 15 digits
	}

	mod _100_pi {
		use super::*;
		const X: f64 = 100. * PI;
		#[test] fn _0() { assert_eq!("314.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("314.2", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("314.16", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("314.159", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("314.159_3", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("314.159_27", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("314.159_265", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("314.159_265_4", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("314.159_265_36", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("314.159_265_359", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("314.159_265_359_0", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("314.159_265_358_98", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("314.159_265_358_979", X.to_string_pretty(12)) }
		// only up to 15 digits
	}
	mod _m100_pi {
		use super::*;
		const X: f64 = -100. * PI;
		#[test] fn _0() { assert_eq!("-314.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-314.2", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-314.16", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-314.159", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-314.159_3", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-314.159_27", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-314.159_265", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-314.159_265_4", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-314.159_265_36", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-314.159_265_359", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-314.159_265_359_0", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("-314.159_265_358_98", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("-314.159_265_358_979", X.to_string_pretty(12)) }
		// only up to 15 digits
	}

	mod _1000_pi {
		use super::*;
		const X: f64 = 1000. * PI;
		#[test] fn _0() { assert_eq!("3_142.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("3_141.6", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("3_141.59", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("3_141.593", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("3_141.592_7", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("3_141.592_65", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("3_141.592_654", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("3_141.592_653_6", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("3_141.592_653_59", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("3_141.592_653_590", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("3_141.592_653_589_8", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("3_141.592_653_589_79", X.to_string_pretty(11)) }
		// only up to 15 digits
	}
	mod _m1000_pi {
		use super::*;
		const X: f64 = -1000. * PI;
		#[test] fn _0() { assert_eq!("-3_142.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-3_141.6", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-3_141.59", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-3_141.593", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-3_141.592_7", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-3_141.592_65", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-3_141.592_654", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-3_141.592_653_6", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-3_141.592_653_59", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-3_141.592_653_590", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-3_141.592_653_589_8", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("-3_141.592_653_589_79", X.to_string_pretty(11)) }
		// only up to 15 digits
	}

	mod _10_000_pi {
		use super::*;
		const X: f64 = 10_000. * PI;
		#[test] fn _0() { assert_eq!("31_416.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("31_415.9", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("31_415.93", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("31_415.927", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("31_415.926_5", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("31_415.926_54", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("31_415.926_536", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("31_415.926_535_9", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("31_415.926_535_90", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("31_415.926_535_898", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("31_415.926_535_897_9", X.to_string_pretty(10)) }
		// only up to 15 digits
	}
	mod _m10_000_pi {
		use super::*;
		const X: f64 = -10_000. * PI;
		#[test] fn _0() { assert_eq!("-31_416.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-31_415.9", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-31_415.93", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-31_415.927", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-31_415.926_5", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-31_415.926_54", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-31_415.926_536", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-31_415.926_535_9", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-31_415.926_535_90", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-31_415.926_535_898", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-31_415.926_535_897_9", X.to_string_pretty(10)) }
		// only up to 15 digits
	}

	mod _123_456_789_012_345 {
		use super::*;
		const X: f64 = 123_456_789_012_345.;
		#[test] fn _0() { assert_eq!("123_456_789_012_345.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("123_456_789_012_345.0", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("123_456_789_012_345.00", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("123_456_789_012_345.000", X.to_string_pretty(3)) }
	}
	mod _m123_456_789_012_345 {
		use super::*;
		const X: f64 = -123_456_789_012_345.;
		#[test] fn _0() { assert_eq!("-123_456_789_012_345.", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-123_456_789_012_345.0", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-123_456_789_012_345.00", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-123_456_789_012_345.000", X.to_string_pretty(3)) }
	}

	mod _10_30_pi {
		use super::*;
		const X: f64 = 1e30 * PI;
		#[test] fn _0() { assert_eq!("3._E30", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("3.1_E30", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("3.14_E30", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("3.142_E30", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("3.141_6_E30", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("3.141_59_E30", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("3.141_593_E30", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("3.141_592_7_E30", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("3.141_592_65_E30", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("3.141_592_654_E30", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("3.141_592_653_6_E30", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("3.141_592_653_59_E30", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("3.141_592_653_590_E30", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("3.141_592_653_589_8_E30", X.to_string_pretty(13)) }
		#[test] fn _14(){ assert_eq!("3.141_592_653_589_79_E30", X.to_string_pretty(14)) }
		// only up to 15 digits
	}
	mod _m10_30_pi {
		use super::*;
		const X: f64 = -1e30 * PI;
		#[test] fn _0() { assert_eq!("-3._E30", X.to_string_pretty(0)) }
		#[test] fn _1() { assert_eq!("-3.1_E30", X.to_string_pretty(1)) }
		#[test] fn _2() { assert_eq!("-3.14_E30", X.to_string_pretty(2)) }
		#[test] fn _3() { assert_eq!("-3.142_E30", X.to_string_pretty(3)) }
		#[test] fn _4() { assert_eq!("-3.141_6_E30", X.to_string_pretty(4)) }
		#[test] fn _5() { assert_eq!("-3.141_59_E30", X.to_string_pretty(5)) }
		#[test] fn _6() { assert_eq!("-3.141_593_E30", X.to_string_pretty(6)) }
		#[test] fn _7() { assert_eq!("-3.141_592_7_E30", X.to_string_pretty(7)) }
		#[test] fn _8() { assert_eq!("-3.141_592_65_E30", X.to_string_pretty(8)) }
		#[test] fn _9() { assert_eq!("-3.141_592_654_E30", X.to_string_pretty(9)) }
		#[test] fn _10(){ assert_eq!("-3.141_592_653_6_E30", X.to_string_pretty(10)) }
		#[test] fn _11(){ assert_eq!("-3.141_592_653_59_E30", X.to_string_pretty(11)) }
		#[test] fn _12(){ assert_eq!("-3.141_592_653_590_E30", X.to_string_pretty(12)) }
		#[test] fn _13(){ assert_eq!("-3.141_592_653_589_8_E30", X.to_string_pretty(13)) }
		#[test] fn _14(){ assert_eq!("-3.141_592_653_589_79_E30", X.to_string_pretty(14)) }
		// only up to 15 digits
	}
}


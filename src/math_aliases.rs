//! math aliases

use crate::float_type::float;



pub fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b { a } else { b }
}
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
	if a > b { a } else { b }
}



pub fn sin(x: float) -> float { x.sin() }
pub fn cos(x: float) -> float { x.cos() }
pub fn asin(x: float) -> float { x.asin() }
pub fn acos(x: float) -> float { x.acos() }
pub fn sinh(x: float) -> float { x.sinh() }
pub fn cosh(x: float) -> float { x.cosh() }
pub fn asinh(x: float) -> float { x.asinh() }
pub fn acosh(x: float) -> float { x.acosh() }
pub fn tan(x: float) -> float { x.tan() }
pub fn atan(x: float) -> float { x.atan() }
pub fn tanh(x: float) -> float { x.tanh() }
pub fn atanh(x: float) -> float { x.atanh() }
pub fn sqrt(x: float) -> float { x.sqrt() }
pub fn exp(x: float) -> float { x.exp() }
pub fn ln(x: float) -> float { x.ln() }
// pub fn log(x: float) -> float { x.log() }
pub fn log2(x: float) -> float { x.log2() }
pub fn log10(x: float) -> float { x.log10() }

pub fn sigmoid(x: float) -> float { 1. / (1. + exp(-x)) }
// y = 1/(1+exp(-x)) => y*(1+exp(-x)) = 1 => 1+exp(-x) = 1/y => exp(-x) = 1/y-1 => -x = ln(1/y-1) => x = -ln(1/y-1)
pub fn asigmoid(x: float) -> float { -ln(1./x - 1.) }

pub fn hypot(a: float, b: float) -> float { a.hypot(b) }


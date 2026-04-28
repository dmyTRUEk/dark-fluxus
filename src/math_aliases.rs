//! math aliases



pub fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b { a } else { b }
}
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
	if a > b { a } else { b }
}



pub fn abs(x: f32) -> f32 { x.abs() }
pub fn sin(x: f32) -> f32 { x.sin() }
pub fn cos(x: f32) -> f32 { x.cos() }
pub fn asin(x: f32) -> f32 { x.asin() }
pub fn acos(x: f32) -> f32 { x.acos() }
pub fn sinh(x: f32) -> f32 { x.sinh() }
pub fn cosh(x: f32) -> f32 { x.cosh() }
pub fn asinh(x: f32) -> f32 { x.asinh() }
pub fn acosh(x: f32) -> f32 { x.acosh() }
pub fn tan(x: f32) -> f32 { x.tan() }
pub fn atan(x: f32) -> f32 { x.atan() }
pub fn tanh(x: f32) -> f32 { x.tanh() }
pub fn atanh(x: f32) -> f32 { x.atanh() }
pub fn sqrt(x: f32) -> f32 { x.sqrt() }
pub fn exp(x: f32) -> f32 { x.exp() }
pub fn ln(x: f32) -> f32 { x.ln() }
// pub fn log(x: f32) -> f32 { x.log() }
pub fn log2(x: f32) -> f32 { x.log2() }
pub fn log10(x: f32) -> f32 { x.log10() }

pub fn sigmoid(x: f32) -> f32 { 1. / (1. + exp(-x)) }
// y = 1/(1+exp(-x)) => y*(1+exp(-x)) = 1 => 1+exp(-x) = 1/y => exp(-x) = 1/y-1 => -x = ln(1/y-1) => x = -ln(1/y-1)
pub fn asigmoid(x: f32) -> f32 { -ln(1./x - 1.) }

pub fn hypot(a: f32, b: f32) -> f32 { a.hypot(b) }


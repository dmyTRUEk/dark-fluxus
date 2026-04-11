//! math

use crate::float_type::float;



pub fn lerp(t: float, a: float, b: float) -> float { a + (b - a) * t }
pub fn unlerp(x: float, a: float, b: float) -> float { (x - a) / (b - a) }


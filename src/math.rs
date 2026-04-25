//! math



pub fn lerp(t: f32, a: f32, b: f32) -> f32 { a + (b - a) * t }
pub fn unlerp(x: f32, a: f32, b: f32) -> f32 { (x - a) / (b - a) }


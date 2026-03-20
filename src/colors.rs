//! colors

#[derive(Clone, Copy)]
pub struct Color(pub u32);

pub const BLACK: Color = Color(0x000000);
pub const WHITE: Color = Color(0xffffff);

pub const RED  : Color = Color(0xff0000);
pub const GREEN: Color = Color(0x00ff00);
pub const BLUE : Color = Color(0x0000ff);

pub const CYAN   : Color = Color(0x00ffff);
pub const MAGENTA: Color = Color(0xff00ff);
pub const YELLOW : Color = Color(0xffff00);

pub const GRAY: Color = Color(0x888888);


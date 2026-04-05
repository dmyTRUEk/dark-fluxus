//! simple font rendering

use sdl3::{render::{Canvas, FPoint}, video::Window};

use crate::vec2d::Vec2i;



pub trait CanvasRenderText {
	fn render_char(&mut self, char: char, pos: (i32, i32), scale: u8);
	fn render_text(&mut self, text: &str, pos: (i32, i32), scale: u8);
	fn render_char_unchecked(&mut self, char: char, pos: (i32, i32), scale: u8);
	fn render_text_unchecked(&mut self, text: &str, pos: (i32, i32), scale: u8);
	fn render_custom_char(&mut self, bitmap: [u8; 25], pos: (i32, i32), scale: u8);
	// TODO(feat): render 3d text
}

impl CanvasRenderText for Canvas<Window> {
	// TODO(optim): use `fill_rect` instead of billion points
	fn render_char(&mut self, char: char, pos: (i32, i32), scale: u8) {
		let pixels = calc_char(char, pos, scale, self.window().size());
		let points: Vec<FPoint> = pixels.into_iter().map(|pixel| pixel.into()).collect();
		self.draw_points(points.as_slice()).unwrap();
	}
	fn render_text(&mut self, text: &str, pos: (i32, i32), scale: u8) {
		let pixels = calc_text(text, pos, scale, self.window().size());
		let points: Vec<FPoint> = pixels.into_iter().map(|pixel| pixel.into()).collect();
		self.draw_points(points.as_slice()).unwrap();
	}
	fn render_char_unchecked(&mut self, char: char, pos: (i32, i32), scale: u8) {
		let pixels = calc_char_unchecked(char, pos, scale);
		let points: Vec<FPoint> = pixels.into_iter().map(|pixel| pixel.into()).collect();
		self.draw_points(points.as_slice()).unwrap();
	}
	fn render_text_unchecked(&mut self, text: &str, pos: (i32, i32), scale: u8) {
		let pixels = calc_text_unchecked(text, pos, scale);
		let points: Vec<FPoint> = pixels.into_iter().map(|pixel| pixel.into()).collect();
		self.draw_points(points.as_slice()).unwrap();
	}
	fn render_custom_char(&mut self, bitmap: [u8; 25], pos: (i32, i32), scale: u8) {
		let pixels = calc_custom_char(bitmap, pos, scale, self.window().size());
		let points: Vec<FPoint> = pixels.into_iter().map(|pixel| pixel.into()).collect();
		self.draw_points(points.as_slice()).unwrap();
	}
}



// TODO(refactor)?: RenderText<W, H>
pub const FONT_W: u8 = 5;
pub const FONT_H: u8 = 5;

fn calc_text(text: &str, mut pos: (i32, i32), scale: u8, window_wh: (u32, u32)) -> Vec<Vec2i> {
	assert!(scale > 0);
	let mut pixels: Vec<Vec2i> = vec![];
	for c in text.chars() {
		pixels.extend(calc_char(c, pos, scale, window_wh));
		pos.0 += ((FONT_W as i32) + 1) * (scale as i32);
	}
	pixels
}

fn calc_char(char: char, pos: (i32, i32), scale: u8, window_wh: (u32, u32)) -> Vec<Vec2i> {
	assert!(scale > 0);
	let bitmap: Bitmap = get_bitmap_for(char);
	let mut pixels: Vec<Vec2i> = vec![];
	for y in 0 .. FONT_H as i32 {
		for x in 0 .. FONT_W as i32 {
			if bitmap[(y * (FONT_W as i32) + x) as usize] == 1 {
				// top-left corner of scaled pixel block
				let base_x = pos.0 + x * (scale as i32);
				let base_y = pos.1 + y * (scale as i32);
				// draw scaled block
				for dy in 0 .. scale as i32 {
					let Y = base_y + dy;
					if Y >= window_wh.1 as i32 { break }
					if Y < 0 { continue }
					for dx in 0 .. scale as i32 {
						let X = base_x + dx;
						if X >= window_wh.0 as i32 { break }
						if X < 0 { continue }
						pixels.push(Vec2i::new(X, Y));
					}
				}
			}
		}
	}
	pixels
}

fn calc_text_unchecked(text: &str, mut pos: (i32, i32), scale: u8) -> Vec<Vec2i> {
	assert!(scale > 0);
	let mut pixels: Vec<Vec2i> = vec![];
	for c in text.chars() {
		pixels.extend(calc_char_unchecked(c, pos, scale));
		pos.0 += ((FONT_W as i32) + 1) * (scale as i32);
	}
	pixels
}

fn calc_char_unchecked(char: char, pos: (i32, i32), scale: u8) -> Vec<Vec2i> {
	assert!(scale > 0);
	let bitmap: Bitmap = get_bitmap_for(char);
	let mut pixels: Vec<Vec2i> = vec![];
	for y in 0 .. FONT_H as i32 {
		for x in 0 .. FONT_W as i32 {
			if bitmap[(y * (FONT_W as i32) + x) as usize] == 1 {
				// top-left corner of scaled pixel block
				let base_x = pos.0 + x * (scale as i32);
				let base_y = pos.1 + y * (scale as i32);
				// draw scaled block
				for dy in 0 .. scale as i32 {
					let Y = base_y + dy;
					for dx in 0 .. scale as i32 {
						let X = base_x + dx;
						pixels.push(Vec2i::new(X, Y));
					}
				}
			}
		}
	}
	pixels
}

fn calc_custom_char(bitmap: [u8; 25], pos: (i32, i32), scale: u8, window_wh: (u32, u32)) -> Vec<Vec2i> {
	assert!(scale > 0);
	let mut pixels: Vec<Vec2i> = vec![];
	for y in 0 .. FONT_H as i32 {
		for x in 0 .. FONT_W as i32 {
			if bitmap[(y * (FONT_W as i32) + x) as usize] == 1 {
				// top-left corner of scaled pixel block
				let base_x = pos.0 + x * (scale as i32);
				let base_y = pos.1 + y * (scale as i32);
				// draw scaled block
				for dy in 0 .. scale as i32 {
					let Y = base_y + dy;
					if Y >= window_wh.1 as i32 { break }
					if Y < 0 { continue }
					for dx in 0 .. scale as i32 {
						let X = base_x + dx;
						if X >= window_wh.0 as i32 { break }
						if X < 0 { continue }
						pixels.push(Vec2i::new(X, Y));
					}
				}
			}
		}
	}
	pixels
}



fn get_bitmap_for(c: char) -> Bitmap {
	BITMAPS[c as usize - 32]
}


type Bitmap = [u8; (FONT_W*FONT_H) as usize];

const EMPTY_BITMAP: Bitmap = [0; (FONT_W*FONT_H) as usize];
const UNDEFINED_BITMAP: Bitmap = [1,0,1,0,1, 0,1,0,1,0, 1,0,1,0,1, 0,1,0,1,0, 1,0,1,0,1];

const BITMAPS: [Bitmap; 95] = [ // 95 = 126 - 32 + 1
	[ // 32 space
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	[ // 33 !
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,0,0,0,
		0,0,1,0,0,
	],
	[ // 34 "
		0,1,0,1,0,
		0,1,0,1,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	[ // 35 #
		0,1,0,1,0,
		1,1,1,1,1,
		0,1,0,1,0,
		1,1,1,1,1,
		0,1,0,1,0,
	],
	[ // 36 $
		0,1,1,1,0,
		1,0,1,0,0,
		0,1,1,1,0,
		0,0,1,0,1,
		0,1,1,1,0,
	],
	[ // 37 %
		1,1,0,0,1,
		1,1,0,1,0,
		0,0,1,0,0,
		0,1,0,1,1,
		1,0,0,1,1,
	],
	[ // 38 &
		0,1,1,1,0,
		1,0,0,0,0,
		0,1,1,0,0,
		1,0,0,1,0,
		0,1,1,0,1,
	],
	[ // 39 '
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	[ // 40 (
		0,1,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		0,1,0,0,0,
	],
	[ // 41 )
		0,0,0,1,0,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,1,0,
	],
	[ // 42 *
		0,0,0,0,0,
		0,1,0,1,0,
		0,0,1,0,0,
		0,1,0,1,0,
		0,0,0,0,0,
	],
	[ // 43 +
		0,0,1,0,0,
		0,0,1,0,0,
		1,1,1,1,1,
		0,0,1,0,0,
		0,0,1,0,0,
	],
	[ // 44 ,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,1,0,
		0,0,1,0,0,
	],
	[ // 45 -
		0,0,0,0,0,
		0,0,0,0,0,
		1,1,1,1,1,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	[ // 46 .
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,1,0,0,
	],
	[ // 47 /
		0,0,0,0,1,
		0,0,0,1,0,
		0,0,1,0,0,
		0,1,0,0,0,
		1,0,0,0,0,
	],
	[ // 48 0
		0,1,1,1,0,
		1,0,0,0,1,
		1,0,1,0,1,
		1,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 49 1
		0,0,1,0,0,
		0,1,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		1,1,1,1,1,
	],
	[ // 50 2
		1,1,1,1,1,
		0,0,0,0,1,
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
	],
	[ // 51 3
		1,1,1,1,1,
		0,0,0,0,1,
		1,1,1,1,1,
		0,0,0,0,1,
		1,1,1,1,1,
	],
	[ // 52 4
		1,0,0,0,1,
		1,0,0,0,1,
		1,1,1,1,1,
		0,0,0,0,1,
		0,0,0,0,1,
	],
	[ // 53 5
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
		0,0,0,0,1,
		1,1,1,1,1,
	],
	[ // 54 6
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
		1,0,0,0,1,
		1,1,1,1,1,
	],
	[ // 55 7
		1,1,1,1,1,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,0,1,
	],
	[ // 56 8
		1,1,1,1,1,
		1,0,0,0,1,
		1,1,1,1,1,
		1,0,0,0,1,
		1,1,1,1,1,
	],
	[ // 57 9
		1,1,1,1,1,
		1,0,0,0,1,
		1,1,1,1,1,
		0,0,0,0,1,
		1,1,1,1,1,
	],
	[ // 58 :
		0,0,0,0,0,
		0,0,1,0,0,
		0,0,0,0,0,
		0,0,1,0,0,
		0,0,0,0,0,
	],
	[ // 59 ;
		0,0,0,0,0,
		0,0,1,0,0,
		0,0,0,0,0,
		0,0,1,0,0,
		0,1,0,0,0,
	],
	[ // 60 <
		0,0,1,0,0,
		0,1,0,0,0,
		1,0,0,0,0,
		0,1,0,0,0,
		0,0,1,0,0,
	],
	[ // 61 =
		0,0,0,0,0,
		1,1,1,1,1,
		0,0,0,0,0,
		1,1,1,1,1,
		0,0,0,0,0,
	],
	[ // 62 >
		0,0,1,0,0,
		0,0,0,1,0,
		0,0,0,0,1,
		0,0,0,1,0,
		0,0,1,0,0,
	],
	[ // 63 ?
		0,1,1,1,0,
		0,0,0,1,0,
		0,0,1,1,0,
		0,0,0,0,0,
		0,0,1,0,0,
	],
	[ // 64 @
		0,1,1,1,0,
		1,0,0,0,1,
		1,0,1,0,1,
		1,0,0,1,0,
		0,1,0,0,0,
	],
	[ // 65 A
		0,0,1,0,0,
		0,1,0,1,0,
		1,0,0,0,1,
		1,1,1,1,1,
		1,0,0,0,1,
	],
	[ // 66 B
		1,1,1,1,0,
		1,0,0,0,1,
		1,1,1,1,0,
		1,0,0,0,1,
		1,1,1,1,0,
	],
	[ // 67 C
		0,1,1,1,1,
		1,0,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		0,1,1,1,1,
	],
	[ // 68 D
		1,1,1,1,0,
		1,0,0,0,1,
		1,0,0,0,1,
		1,0,0,0,1,
		1,1,1,1,0,
	],
	[ // 69 E
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
	],
	[ // 70 F
		1,1,1,1,1,
		1,0,0,0,0,
		1,1,1,1,1,
		1,0,0,0,0,
		1,0,0,0,0,
	],
	[ // 71 G
		0,1,1,1,0,
		1,0,0,0,0,
		1,0,1,1,1,
		1,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 72 H
		1,0,0,0,1,
		1,0,0,0,1,
		1,1,1,1,1,
		1,0,0,0,1,
		1,0,0,0,1,
	],
	[ // 73 I
		1,1,1,1,1,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		1,1,1,1,1,
	],
	[ // 74 J
		0,0,1,1,1,
		0,0,0,0,1,
		0,0,0,0,1,
		1,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 75 K
		1,0,0,0,1,
		1,0,0,1,0,
		1,1,1,0,0,
		1,0,0,1,0,
		1,0,0,0,1,
	],
	[ // 76 L
		1,0,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		1,1,1,1,1,
	],
	[ // 77 M
		1,0,0,0,1,
		1,1,0,1,1,
		1,0,1,0,1,
		1,0,0,0,1,
		1,0,0,0,1,
	],
	[ // 78 N
		1,0,0,0,1,
		1,1,0,0,1,
		1,0,1,0,1,
		1,0,0,1,1,
		1,0,0,0,1,
	],
	[ // 79 O
		0,1,1,1,0,
		1,0,0,0,1,
		1,0,0,0,1,
		1,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 80 P
		1,1,1,1,0,
		1,0,0,0,1,
		1,1,1,1,0,
		1,0,0,0,0,
		1,0,0,0,0,
	],
	[ // 81 Q
		0,1,1,1,0,
		1,0,0,0,1,
		1,0,1,0,1,
		1,0,0,1,0,
		0,1,1,0,1,
	],
	[ // 82 P
		1,1,1,1,0,
		1,0,0,0,1,
		1,1,1,1,0,
		1,0,0,1,0,
		1,0,0,0,1,
	],
	[ // 83 S
		0,1,1,1,0,
		1,0,0,0,0,
		0,1,1,1,0,
		0,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 84 T
		1,1,1,1,1,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
	],
	[ // 85 U
		1,0,0,0,1,
		1,0,0,0,1,
		1,0,0,0,1,
		1,0,0,0,1,
		0,1,1,1,0,
	],
	[ // 86 V
		1,0,0,0,1,
		1,0,0,0,1,
		0,1,0,1,0,
		0,1,0,1,0,
		0,0,1,0,0,
	],
	[ // 87 W
		1,0,0,0,1,
		1,0,1,0,1,
		1,0,1,0,1,
		1,0,1,0,1,
		0,1,0,1,0,
	],
	[ // 88 X
		1,0,0,0,1,
		0,1,0,1,0,
		0,0,1,0,0,
		0,1,0,1,0,
		1,0,0,0,1,
	],
	[ // 89 Y
		1,0,0,0,1,
		0,1,0,1,0,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
	],
	[ // 90 Z
		1,1,1,1,1,
		0,0,0,1,0,
		0,0,1,0,0,
		0,1,0,0,0,
		1,1,1,1,1,
	],
	[ // 91 [
		1,1,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		1,0,0,0,0,
		1,1,0,0,0,
	],
	[ // 92 \
		1,0,0,0,0,
		0,1,0,0,0,
		0,0,1,0,0,
		0,0,0,1,0,
		0,0,0,0,1,
	],
	[ // 93 ]
		0,0,0,1,1,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,0,1,
		0,0,0,1,1,
	],
	[ // 94 ^
		0,0,1,0,0,
		0,1,0,1,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	[ // 95 _
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
		1,1,1,1,1,
	],
	[ // 96 `
		0,0,1,0,0,
		0,0,0,1,0,
		0,0,0,0,0,
		0,0,0,0,0,
		0,0,0,0,0,
	],
	UNDEFINED_BITMAP, // 97 a
	UNDEFINED_BITMAP, // 98 b
	UNDEFINED_BITMAP, // 99 c
	UNDEFINED_BITMAP, // 100 d
	UNDEFINED_BITMAP, // 101 e
	UNDEFINED_BITMAP, // 102 f
	UNDEFINED_BITMAP, // 103 g
	UNDEFINED_BITMAP, // 104 h
	UNDEFINED_BITMAP, // 105 i
	UNDEFINED_BITMAP, // 106 j
	UNDEFINED_BITMAP, // 107 k
	UNDEFINED_BITMAP, // 108 l
	UNDEFINED_BITMAP, // 109 m
	UNDEFINED_BITMAP, // 110 n
	UNDEFINED_BITMAP, // 111 o
	UNDEFINED_BITMAP, // 112 p
	UNDEFINED_BITMAP, // 113 q
	UNDEFINED_BITMAP, // 114 r
	UNDEFINED_BITMAP, // 115 s
	UNDEFINED_BITMAP, // 116 t
	UNDEFINED_BITMAP, // 117 u
	UNDEFINED_BITMAP, // 118 v
	UNDEFINED_BITMAP, // 119 w
	UNDEFINED_BITMAP, // 120 x
	UNDEFINED_BITMAP, // 121 y
	UNDEFINED_BITMAP, // 122 z
	[ // 123 {
		0,1,1,0,0,
		0,1,0,0,0,
		1,0,0,0,0,
		0,1,0,0,0,
		0,1,1,0,0,
	],
	[ // 124 |
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
		0,0,1,0,0,
	],
	[ // 125 }
		0,0,1,1,0,
		0,0,0,1,0,
		0,0,0,0,1,
		0,0,0,1,0,
		0,0,1,1,0,
	],
	[ // 126 ~
		0,0,0,0,0,
		0,1,0,0,0,
		1,0,1,0,1,
		0,0,0,1,0,
		0,0,0,0,0,
	],
];


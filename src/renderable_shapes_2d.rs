//! renderable shapes

use glam::Vec2;

use crate::{Vertex, color_u8::ColorU8, extensions::{Flatten, Into_}, renderable_shapes::*, vec2_ext::ExtVec2};



impl ToVertexNC for Vec2 {
	fn to_vertex(self, color: ColorU8) -> Vertex {
		Vertex::from(self.extend(0.), color)
	}
}



/// point 2d, with color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
	pub v: Vec2,
	pub color: ColorU8,
}
impl Point2d {
	pub fn from(x: impl Into_<f32>, y: impl Into_<f32>, color: ColorU8) -> Self {
		Self { v: Vec2::new(x.into_(), y.into_()), color }
	}
}
impl ToVertex for Point2d {
	fn to_vertex(self) -> Vertex {
		Vertex::from(self.v.xy0(), self.color)
	}
}





/// line 2d, two colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2d {
	pub a: Point2d,
	pub b: Point2d,
}
impl ToVertices<2> for Line2d {
	fn to_vertices(self) -> [Vertex; 2] {
		[
			Vertex::from(self.a.v.xy0(), self.a.color),
			Vertex::from(self.b.v.xy0(), self.b.color),
		]
	}
}



/// line 2d, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2dOC {
	pub a: Vec2,
	pub b: Vec2,
	pub color: ColorU8,
}
impl Line2dOC {
	pub fn new(a: Vec2, b: Vec2, color: ColorU8) -> Self {
		Self { a, b, color }
	}
}
impl ToVertices<2> for Line2dOC {
	fn to_vertices(self) -> [Vertex; 2] {
		[
			Vertex::from(self.a.xy0(), self.color),
			Vertex::from(self.b.xy0(), self.color),
		]
	}
}



/// line 2d, no color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2dNC {
	pub a: Vec2,
	pub b: Vec2,
}
impl ToVerticesNC<2> for Line2dNC {
	fn to_vertices(self, color: ColorU8) -> [Vertex; 2] {
		[
			Vertex::from(self.a.xy0(), color),
			Vertex::from(self.b.xy0(), color),
		]
	}
}





/// triangle 2d, three colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2d {
	pub a: Point2d,
	pub b: Point2d,
	pub c: Point2d,
}
impl ToVertices<3> for Triangle2d {
	fn to_vertices(self) -> [Vertex; 3] {
		[
			Vertex::from(self.a.v.xy0(), self.a.color),
			Vertex::from(self.b.v.xy0(), self.b.color),
			Vertex::from(self.c.v.xy0(), self.c.color),
		]
	}
}



/// triangle 2d, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2dOC {
	pub a: Vec2,
	pub b: Vec2,
	pub c: Vec2,
	pub color: ColorU8,
}
impl Triangle2dOC {
	pub fn new(a: Vec2, b: Vec2, c: Vec2, color: ColorU8) -> Self {
		Self { a, b, c, color }
	}
}
impl ToVertices<3> for Triangle2dOC {
	fn to_vertices(self) -> [Vertex; 3] {
		[
			Vertex::from(self.a.xy0(), self.color),
			Vertex::from(self.b.xy0(), self.color),
			Vertex::from(self.c.xy0(), self.color),
		]
	}
}



/// triangle 2d, no color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2dNC {
	pub a: Vec2,
	pub b: Vec2,
	pub c: Vec2,
}
impl ToVerticesNC<3> for Triangle2dNC {
	fn to_vertices(self, color: ColorU8) -> [Vertex; 3] {
		[
			Vertex::from(self.a.xy0(), color),
			Vertex::from(self.b.xy0(), color),
			Vertex::from(self.c.xy0(), color),
		]
	}
}





/// rectangle 2d, filled, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle2dOC {
	pub x: f32, // center
	pub y: f32, // center
	pub w: f32,
	pub h: f32,
	pub color: ColorU8,
}
impl Rectangle2dOC {
	fn to_triangles(self) -> [Triangle2dOC; 2] {
		let Self { x, y, w, h, color } = self;
		let w = w / 2.;
		let h = h / 2.;
		[
			Triangle2dOC::new(Vec2::new(x-w, y-h), Vec2::new(x+w, y-h), Vec2::new(x-w, y+h), color),
			Triangle2dOC::new(Vec2::new(x+w, y+h), Vec2::new(x+w, y-h), Vec2::new(x-w, y+h), color),
		]
	}
	fn to_lines(self) -> [Line2dOC; 4] {
		let Self { x, y, w, h, color } = self;
		let w = w / 2.;
		let h = h / 2.;
		[
			Line2dOC::new(Vec2::new(x-w, y-h), Vec2::new(x-w, y+h), color),
			Line2dOC::new(Vec2::new(x-w, y+h), Vec2::new(x+w, y+h), color),
			Line2dOC::new(Vec2::new(x+w, y+h), Vec2::new(x+w, y-h), color),
			Line2dOC::new(Vec2::new(x+w, y-h), Vec2::new(x-w, y-h), color),
		]
	}
	pub fn to_triangles_vertices(self) -> [Vertex; 6] {
		self.to_triangles().map(|t| t.to_vertices()).flatten_()
	}
	pub fn to_lines_vertices(self) -> [Vertex; 8] {
		self.to_lines().map(|l| l.to_vertices()).flatten_()
	}
}


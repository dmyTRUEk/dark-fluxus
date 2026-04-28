//! renderable shapes

use glam::Vec3;

use crate::{Vertex, color_u8::ColorU8, renderable_shapes::*};



impl ToVertexNC for Vec3 {
	fn to_vertex(self, color: ColorU8) -> Vertex {
		Vertex::from(self, color)
	}
}



/// point 3d, with color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3d {
	pub v: Vec3,
	pub color: ColorU8,
}
impl Point3d {
	pub fn new(v: Vec3, color: ColorU8) -> Self {
		Self { v, color }
	}
}
// impl From<(Vec3, ColorU8)> for Point3d {
// 	fn from((v, color): (Vec3, ColorU8)) -> Self {
// 		Self { v, color }
// 	}
// }
impl ToVertex for Point3d {
	fn to_vertex(self) -> Vertex {
		Vertex::from(self.v, self.color)
	}
}





/// line 3d, two colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3d {
	pub a: Point3d,
	pub b: Point3d,
}
impl Line3d {
	pub fn new(a: Point3d, b: Point3d) -> Self {
		Self { a, b }
	}
}
impl ToVertices<2> for Line3d {
	fn to_vertices(self) -> [Vertex; 2] {
		[
			Vertex::from(self.a.v, self.a.color),
			Vertex::from(self.b.v, self.b.color),
		]
	}
}



/// line 3d, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3dOC {
	pub a: Vec3,
	pub b: Vec3,
	pub color: ColorU8,
}
impl Line3dOC {
	pub fn new(a: Vec3, b: Vec3, color: ColorU8) -> Self {
		Self { a, b, color }
	}
	pub fn from(a: impl Into<Vec3>, b: impl Into<Vec3>, color: ColorU8) -> Self {
		Self::new(a.into(), b.into(), color)
	}
}
impl ToVertices<2> for Line3dOC {
	fn to_vertices(self) -> [Vertex; 2] {
		[
			Vertex::from(self.a, self.color),
			Vertex::from(self.b, self.color),
		]
	}
}



/// line 3d, no color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3dNC {
	pub a: Vec3,
	pub b: Vec3,
}
impl Line3dNC {
	pub fn new(a: Vec3, b: Vec3) -> Self {
		Self { a, b }
	}
}
impl From<(Vec3, Vec3)> for Line3dNC {
	fn from((a, b): (Vec3, Vec3)) -> Self {
		Self { a, b }
	}
}
impl ToVerticesNC<2> for Line3dNC {
	fn to_vertices(self, color: ColorU8) -> [Vertex; 2] {
		[
			Vertex::from(self.a, color),
			Vertex::from(self.b, color),
		]
	}
}





/// triangle 3d, three colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3d {
	pub a: Point3d,
	pub b: Point3d,
	pub c: Point3d,
}
impl Triangle3d {
	pub fn new(a: Point3d, b: Point3d, c: Point3d) -> Self {
		Self { a, b, c }
	}
}
impl ToVertices<3> for Triangle3d {
	fn to_vertices(self) -> [Vertex; 3] {
		[
			Vertex::from(self.a.v, self.a.color),
			Vertex::from(self.b.v, self.b.color),
			Vertex::from(self.c.v, self.c.color),
		]
	}
}



/// triangle 3d, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3dOC {
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub color: ColorU8,
}
impl Triangle3dOC {
	fn new(a: Vec3, b: Vec3, c: Vec3, color: ColorU8) -> Self {
		Self { a, b, c, color }
	}
}
impl ToVertices<3> for Triangle3dOC {
	fn to_vertices(self) -> [Vertex; 3] {
		[
			Vertex::from(self.a, self.color),
			Vertex::from(self.b, self.color),
			Vertex::from(self.c, self.color),
		]
	}
}



/// triangle 3d, no color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3dNC {
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
}
impl Triangle3dNC {
	fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
		Self { a, b, c }
	}
}
impl ToVerticesNC<3> for Triangle3dNC {
	fn to_vertices(self, color: ColorU8) -> [Vertex; 3] {
		[
			Vertex::from(self.a, color),
			Vertex::from(self.b, color),
			Vertex::from(self.c, color),
		]
	}
}





/// quad 3d, three colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad3d {
	pub a: Point3d,
	pub b: Point3d,
	pub c: Point3d,
	pub d: Point3d,
}
impl Quad3d {
	pub fn new(a: Point3d, b: Point3d, c: Point3d, d: Point3d) -> Self {
		Self { a, b, c, d }
	}
	fn to_triangles(self) -> [Triangle3d; 2] {
		[
			Triangle3d::new(self.a, self.b, self.c),
			Triangle3d::new(self.b, self.c, self.d)
		]
	}
}
impl ToVertices<4> for Quad3d {
	fn to_vertices(self) -> [Vertex; 4] {
		[
			Vertex::from(self.a.v, self.a.color),
			Vertex::from(self.b.v, self.b.color),
			Vertex::from(self.c.v, self.c.color),
			Vertex::from(self.d.v, self.d.color),
		]
	}
}



/// quad 3d, one color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad3dOC {
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub d: Vec3,
	pub color: ColorU8,
}
impl Quad3dOC {
	pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: ColorU8) -> Self {
		Self { a, b, c, d, color }
	}
}
impl ToVertices<4> for Quad3dOC {
	fn to_vertices(self) -> [Vertex; 4] {
		[
			Vertex::from(self.a, self.color),
			Vertex::from(self.b, self.color),
			Vertex::from(self.c, self.color),
			Vertex::from(self.d, self.color),
		]
	}
}



/// quad 3d, no color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad3dNC {
	pub a: Vec3,
	pub b: Vec3,
	pub c: Vec3,
	pub d: Vec3,
}
impl Quad3dNC {
	fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Self {
		Self { a, b, c, d }
	}
}
impl ToVerticesNC<4> for Quad3dNC {
	fn to_vertices(self, color: ColorU8) -> [Vertex; 4] {
		[
			Vertex::from(self.a, color),
			Vertex::from(self.b, color),
			Vertex::from(self.c, color),
			Vertex::from(self.d, color),
		]
	}
}


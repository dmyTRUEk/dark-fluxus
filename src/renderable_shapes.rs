//! renderable shapes shared code

use glam::{Vec2, Vec3};

use crate::{Vertex, color_u8::ColorU8, renderable_shapes_2d::*, renderable_shapes_3d::*};



pub trait ToVertex { fn to_vertex(self) -> Vertex; }
pub trait ToVertexNC { fn to_vertex(self, color: ColorU8) -> Vertex; }
pub trait ToVertices<const N: usize> { fn to_vertices(self) -> [Vertex; N]; }
pub trait ToVerticesNC<const N: usize> { fn to_vertices(self, color: ColorU8) -> [Vertex; N]; }
// pub trait ToVerticesVec { fn to_vertices(self) -> Vec<Vertex>; }
// pub trait ToVerticesVecNC { fn to_vertices(self, color: ColorU8) -> Vec<Vertex>; }



#[derive(Debug)]
pub enum RenderableShape {
	Points3d_(Vec<Point3d>),
	Points3dOC_(Vec<Vec3>, ColorU8),
	Points3dNC_(Vec<Vec3>),
	Lines3d_(Vec<Line3d>),
	Lines3dOC_(Vec<Line3dNC>, ColorU8),
	Lines3dNC_(Vec<Line3dNC>),
	LineStrip3d_(Vec<Point3d>),
	LineStrip3dOC_(Vec<Vec3>, ColorU8),
	LineStrip3dNC_(Vec<Vec3>),
	Triangles3d_(Vec<Triangle3d>),
	Triangles3dOC_(Vec<Triangle3dNC>, ColorU8),
	Triangles3dNC_(Vec<Triangle3dNC>),
	// TriangleStrip3d_(Points?),
	// TriangleStrip3dOC_(Points?, ColorU8),
	// TriangleStrip3dNC_(Points?),
	Quads3d_(Vec<Quad3d>),
	Quads3dOC_(Vec<Quad3dOC>, ColorU8),
	Quads3dNC_(Vec<Quad3dNC>),

	Points2d_(Vec<Point2d>),
	Points2dOC_(Vec<Vec2>, ColorU8),
	Points2dNC_(Vec<Vec2>),
	Lines2d_(Vec<Line2d>),
	Lines2dOC_(Vec<Line2dNC>, ColorU8),
	Lines2dNC_(Vec<Line2dNC>),
	LineStrip2d_(Vec<Point2d>),
	LineStrip2dOC_(Vec<Vec2>, ColorU8),
	LineStrip2dNC_(Vec<Vec2>),
	Triangles2d_(Vec<Triangle2d>),
	Triangles2dOC_(Vec<Triangle2dNC>, ColorU8),
	Triangles2dNC_(Vec<Triangle2dNC>),
	// TriangleStrip2d_(Points?),
	// TriangleStrip2dOC_(Points?, ColorU8),
	// TriangleStrip2dNC_(Points?),
}


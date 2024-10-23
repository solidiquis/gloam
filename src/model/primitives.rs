#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
}

impl From<Primitive> for gl::types::GLenum {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Points => gl::POINTS,
            Primitive::Lines => gl::LINES,
            Primitive::LineStrip => gl::LINE_STRIP,
            Primitive::LineLoop => gl::LINE_LOOP,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
            Primitive::TriangleFan => gl::TRIANGLE_FAN,
            Primitive::Quads => gl::QUADS,
        }
    }
}

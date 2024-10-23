use gl::types::GLenum;

#[derive(Copy, Clone)]
pub enum TextureWrapParam {
    Repeat,
    ClampToEdge,
    MirroredRepeat,
    /// rgba
    ClampToBorder(f32, f32, f32, f32),
}

impl From<TextureWrapParam> for GLenum {
    fn from(value: TextureWrapParam) -> Self {
        match value {
            TextureWrapParam::Repeat => gl::REPEAT,
            TextureWrapParam::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureWrapParam::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureWrapParam::ClampToBorder(_, _, _, _) => gl::CLAMP_TO_BORDER,
        }
    }
}

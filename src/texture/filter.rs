use gl::types::GLenum;

#[derive(Debug)]
pub enum TextureFilterParam {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl From<TextureFilterParam> for GLenum {
    fn from(value: TextureFilterParam) -> Self {
        match value {
            TextureFilterParam::Nearest => gl::NEAREST,
            TextureFilterParam::Linear => gl::LINEAR,
            TextureFilterParam::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            TextureFilterParam::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            TextureFilterParam::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            TextureFilterParam::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }
    }
}

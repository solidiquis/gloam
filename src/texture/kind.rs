use gl::types::GLenum;

#[derive(Copy, Clone)]
pub enum TextureType {
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCubeMap,
    Texture1DArray,
    Texture2DArray,
    TextureCubeMapArray,
    TextureRectangle,
}

impl From<TextureType> for GLenum {
    fn from(value: TextureType) -> Self {
        match value {
            TextureType::Texture1D => gl::TEXTURE_1D,
            TextureType::Texture2D => gl::TEXTURE_2D,
            TextureType::Texture3D => gl::TEXTURE_3D,
            TextureType::TextureCubeMap => gl::TEXTURE_CUBE_MAP,
            TextureType::Texture1DArray => gl::TEXTURE_1D_ARRAY,
            TextureType::Texture2DArray => gl::TEXTURE_2D_ARRAY,
            TextureType::TextureCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TextureType::TextureRectangle => gl::TEXTURE_RECTANGLE,
        }
    }
}

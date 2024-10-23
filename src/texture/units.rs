use gl::types::GLenum;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum TextureUnit {
    Texture0,
    Texture1,
    Texture2,
    Texture3,
    Texture4,
    Texture5,
    Texture6,
    Texture7,
    Texture8,
    Texture9,
    Texture10,
    Texture11,
    Texture12,
    Texture13,
    Texture14,
    Texture15,
}

#[derive(Default)]
pub struct TextureUnitIter {
    current_unit: u8,
}

impl TextureUnit {
    pub fn iter() -> TextureUnitIter {
        TextureUnitIter::default()
    }
}

impl Iterator for TextureUnitIter {
    type Item = TextureUnit;

    fn next(&mut self) -> Option<Self::Item> {
        let unit = match self.current_unit {
            0 => Some(TextureUnit::Texture0),
            1 => Some(TextureUnit::Texture1),
            2 => Some(TextureUnit::Texture2),
            3 => Some(TextureUnit::Texture3),
            4 => Some(TextureUnit::Texture4),
            5 => Some(TextureUnit::Texture5),
            6 => Some(TextureUnit::Texture6),
            7 => Some(TextureUnit::Texture7),
            8 => Some(TextureUnit::Texture8),
            9 => Some(TextureUnit::Texture9),
            10 => Some(TextureUnit::Texture10),
            11 => Some(TextureUnit::Texture11),
            12 => Some(TextureUnit::Texture12),
            13 => Some(TextureUnit::Texture13),
            14 => Some(TextureUnit::Texture14),
            15 => Some(TextureUnit::Texture15),
            _ => None,
        };
        self.current_unit += 1;
        unit
    }
}

impl From<TextureUnit> for GLenum {
    fn from(value: TextureUnit) -> Self {
        match value {
            TextureUnit::Texture0 => gl::TEXTURE0,
            TextureUnit::Texture1 => gl::TEXTURE1,
            TextureUnit::Texture2 => gl::TEXTURE2,
            TextureUnit::Texture3 => gl::TEXTURE3,
            TextureUnit::Texture4 => gl::TEXTURE4,
            TextureUnit::Texture5 => gl::TEXTURE5,
            TextureUnit::Texture6 => gl::TEXTURE6,
            TextureUnit::Texture7 => gl::TEXTURE7,
            TextureUnit::Texture8 => gl::TEXTURE8,
            TextureUnit::Texture9 => gl::TEXTURE9,
            TextureUnit::Texture10 => gl::TEXTURE10,
            TextureUnit::Texture11 => gl::TEXTURE11,
            TextureUnit::Texture12 => gl::TEXTURE12,
            TextureUnit::Texture13 => gl::TEXTURE13,
            TextureUnit::Texture14 => gl::TEXTURE14,
            TextureUnit::Texture15 => gl::TEXTURE15,
        }
    }
}

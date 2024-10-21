use crate::internal_utils::try_convert;
use crate::{misc_error, Error, Result};
use gl::types::{GLenum, GLuint};
use std::ffi::c_void;
use std::ops::Drop;
use std::path::Path;

pub mod filter;
pub use filter::TextureFilterParam;

pub mod kind;
pub use kind::TextureType;

pub mod wrap;
pub use wrap::TextureWrapParam;

pub struct Texture {
    gl_object_id: GLuint,
    kind: TextureType,
}

pub struct TextureBuilder {
    kind: TextureType,
    data: Vec<u8>,
    width: u32,
    height: u32,
    s_wrap: Option<TextureWrapParam>,
    t_wrap: Option<TextureWrapParam>,
    r_wrap: Option<TextureWrapParam>,
    min_filter: Option<TextureFilterParam>,
    mag_filter: Option<TextureFilterParam>,
    format: gl::types::GLenum,
}

impl Texture {
    pub(crate) fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.kind.into());
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindTexture(self.kind.into(), self.gl_object_id);
        }
    }

    pub(crate) fn unbind(&self) {
        unsafe {
            gl::BindTexture(self.kind.into(), 0);
        }
    }
}

impl TextureBuilder {
    pub fn new_2d_rgba8<Q: AsRef<Path>>(path: Q) -> Result<Self> {
        let buffer = image::open(path.as_ref())
            .map(|img| img.to_rgba8())
            .map_err(|e| misc_error!("failed to load texture file: {e}"))?;

        let (width, height) = buffer.dimensions();
        let data = buffer.into_raw();

        Ok(TextureBuilder {
            data,
            width,
            height,
            kind: TextureType::Texture2D,
            s_wrap: None,
            t_wrap: None,
            r_wrap: None,
            min_filter: None,
            mag_filter: None,
            format: gl::RGBA,
        })
    }

    pub fn s_wrap(mut self, param: TextureWrapParam) -> Self {
        self.s_wrap = Some(param);
        self
    }

    pub fn t_wrap(mut self, param: TextureWrapParam) -> Self {
        self.t_wrap = Some(param);
        self
    }

    pub fn r_wrap(mut self, param: TextureWrapParam) -> Self {
        self.r_wrap = Some(param);
        self
    }

    pub fn min_filter(mut self, param: TextureFilterParam) -> Self {
        self.min_filter = Some(param);
        self
    }

    pub fn mag_filter(mut self, param: TextureFilterParam) -> Self {
        self.mag_filter = Some(param);
        self
    }

    pub fn build(self) -> Result<Texture> {
        unsafe { self.build_impl() }
    }

    unsafe fn build_impl(self) -> Result<Texture> {
        let TextureBuilder {
            data,
            width,
            height,
            kind,
            s_wrap,
            t_wrap,
            r_wrap,
            min_filter,
            mag_filter,
            format,
        } = self;

        let mut texture = 0;
        let target = GLenum::from(kind);
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(target, texture);

        if let Some(wrap) = s_wrap {
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, try_convert!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(wrap) = t_wrap {
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, try_convert!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(wrap) = r_wrap {
            gl::TexParameteri(target, gl::TEXTURE_WRAP_R, try_convert!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(min) = min_filter {
            gl::TexParameteri(
                target,
                gl::TEXTURE_MIN_FILTER,
                try_convert!(GLenum::from(min)),
            );
        }
        if let Some(mag) = mag_filter {
            gl::TexParameteri(
                target,
                gl::TEXTURE_MAG_FILTER,
                try_convert!(GLenum::from(mag)),
            );
        }

        match kind {
            TextureType::Texture1D => todo!(),
            TextureType::Texture2D => gl::TexImage2D(
                target,
                0,
                try_convert!(format),
                try_convert!(width),
                try_convert!(height),
                0,
                try_convert!(format),
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            ),
            TextureType::Texture3D => todo!(),
            TextureType::TextureCubeMap => todo!(),
            TextureType::Texture1DArray => todo!(),
            TextureType::Texture2DArray => todo!(),
            TextureType::TextureCubeMapArray => todo!(),
            TextureType::TextureRectangle => todo!(),
        }

        gl::BindTexture(target, 0);
        Ok(Texture {
            kind,
            gl_object_id: texture,
        })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.gl_object_id);
        }
        self.gl_object_id = 0;
    }
}

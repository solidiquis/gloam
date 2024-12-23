use crate::{
    context::GLContext,
    error::{misc_error, Error, Result},
    internal_utils::try_into,
    object::{GLObject, GLObjectDescriptor},
};
use gl::types::{GLenum, GLuint};
use image::{DynamicImage, GenericImageView};
use std::{
    ffi::c_void,
    fmt::Debug,
    ops::Drop,
    path::{Path, PathBuf},
};

pub mod filter;
pub use filter::TextureFilterParam;

pub mod kind;
pub use kind::TextureType;

pub mod wrap;
pub use wrap::TextureWrapParam;

pub mod units;
pub use units::TextureUnit;

#[derive(Eq, PartialEq, Hash)]
pub struct Texture {
    pub gl_object_id: GLuint,
    pub kind: TextureType,
    pub src: PathBuf,
}

#[derive(Debug)]
pub struct TextureBuilder {
    kind: TextureType,
    src: PathBuf,
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
        let texture_path = path.as_ref();
        let buffer = image::open(texture_path)
            .map(|img| DynamicImage::ImageRgba8(img.to_rgba8()).flipv())
            .map_err(|e| misc_error!("failed to load texture file: {e}"))?;

        let (width, height) = buffer.dimensions();
        let data = buffer
            .as_rgba8()
            .map(|b| b.as_raw().clone())
            .ok_or(misc_error!("failed to load texture"))?;

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
            src: texture_path.to_path_buf(),
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

    pub fn build(self, ctx: &mut GLContext) -> Result<GLObjectDescriptor> {
        unsafe { self.build_impl(ctx) }
    }

    unsafe fn build_impl(self, ctx: &mut GLContext) -> Result<GLObjectDescriptor> {
        let TextureBuilder {
            data,
            src,
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
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, try_into!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(wrap) = t_wrap {
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, try_into!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(wrap) = r_wrap {
            gl::TexParameteri(target, gl::TEXTURE_WRAP_R, try_into!(GLenum::from(wrap)));

            if let TextureWrapParam::ClampToBorder(r, g, b, a) = wrap {
                let border_color: Vec<f32> = vec![r, g, b, a];
                gl::TexParameterfv(target, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());
            }
        }
        if let Some(min) = min_filter {
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, try_into!(GLenum::from(min)));
        }
        if let Some(mag) = mag_filter {
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, try_into!(GLenum::from(mag)));
        }

        match kind {
            TextureType::Texture1D => todo!(),
            TextureType::Texture2D => gl::TexImage2D(
                target,
                0,
                try_into!(format),
                try_into!(width),
                try_into!(height),
                0,
                try_into!(format),
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
        let texture = Texture {
            kind,
            src,
            gl_object_id: texture,
        };
        let obj_desc = ctx.register_object(GLObject::Texture(texture));
        Ok(obj_desc)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_object_id);
        }
        self.gl_object_id = 0;
    }
}

impl Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Texture {{ gl_object_id={}, texture_type={:?}, src='{}' }}",
            self.gl_object_id,
            self.kind,
            self.src.display(),
        )
    }
}

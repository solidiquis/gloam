use super::GLContext;
use crate::{
    error::{gl_check_err, Error, Result},
    internal_utils::try_into,
    object::GLObjectDescriptor,
    texture::TextureUnit,
};
use gl::types::GLint;

impl GLContext {
    pub fn activate_texture(
        &mut self,
        obj_desc: GLObjectDescriptor,
        generate_mipmap: bool,
    ) -> Result<GLint> {
        // texture already active
        if let Some((idx, _)) = self
            .active_textures
            .iter()
            .enumerate()
            .find(|(_, od)| od.is_some_and(|o| o == obj_desc))
        {
            return TextureUnit::try_from(idx).map(GLint::from);
        }

        let (idx, _) = self
            .active_textures
            .iter()
            .enumerate()
            .find(|(_, od)| od.is_none())
            .ok_or(Error::MaxActiveTextures)?;

        let slot = &mut self.active_textures[idx];
        *slot = Some(obj_desc);

        let texture = self.get_texture(obj_desc)?;
        let texture_unit = TextureUnit::try_from(idx)?;

        unsafe {
            gl::ActiveTexture(texture_unit.into());
        }
        texture.bind();

        log::debug!("bound texture {texture_unit:?}: {texture:?}");

        if generate_mipmap {
            texture.generate_mipmap();
        }
        Ok(GLint::from(texture_unit))
    }

    pub fn deactivate_texture(
        &mut self,
        obj_desc: GLObjectDescriptor,
    ) -> Option<GLObjectDescriptor> {
        let (idx, od) = self
            .active_textures
            .iter_mut()
            .enumerate()
            .find(|(_, od)| od.is_some_and(|o| o == obj_desc))?;

        let obj_desc = od.take().unwrap();
        let texture = self.get_texture(obj_desc).ok()?;
        let texture_unit = TextureUnit::try_from(idx).ok()?;

        unsafe {
            gl::ActiveTexture(texture_unit.into());
        }
        texture.unbind();

        log::debug!("unbound texture {texture_unit:?}: {texture:?}");

        Some(obj_desc)
    }

    pub(super) fn init_texture_units() -> Result<Vec<Option<GLObjectDescriptor>>> {
        let mut max_active_textures = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_active_textures) };
        gl_check_err()?;
        let maximum: usize = try_into!(max_active_textures);
        log::debug!("maximum allowed texture units: {maximum}");
        Ok(vec![None; maximum])
    }
}

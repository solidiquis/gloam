use crate::{
    error::{gl_check_err, Error, Result},
    internal_utils::try_into,
    object::{GLObjectDescriptor, GLObjectRegistry},
    texture::TextureUnit,
};
use gl::types::{GLfloat, GLint, GLsizei};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct GLContext {
    object_registry: GLObjectRegistry,
    active_textures: Vec<Option<GLObjectDescriptor>>,
    bound_vertex_object: Option<GLObjectDescriptor>,
    active_program: Option<GLObjectDescriptor>,
}

impl GLContext {
    pub fn new(object_registry: GLObjectRegistry) -> Result<Self> {
        Ok(Self {
            active_textures: Self::init_texture_units()?,
            object_registry,
            bound_vertex_object: None,
            active_program: None,
        })
    }

    pub fn bind_vertex_object(&mut self, obj_desc: GLObjectDescriptor) -> Result<()> {
        // vertex object already bound
        if self
            .bound_vertex_object
            .as_ref()
            .is_some_and(|vo| *vo == obj_desc)
        {
            return Ok(());
        }
        let obj = self.get_vertex_object(obj_desc)?;
        obj.bind();
        self.bound_vertex_object = Some(obj_desc);
        Ok(())
    }

    pub fn unbind_vertex_object(&mut self) -> Option<GLObjectDescriptor> {
        let obj_desc = self.bound_vertex_object.take()?;
        let vo = self.get_vertex_object(obj_desc).ok()?;
        vo.unbind();
        Some(obj_desc)
    }

    pub fn use_program(&mut self, obj_desc: GLObjectDescriptor) -> Result<()> {
        // program already in use
        if self.active_program.is_some_and(|p| p == obj_desc) {
            return Ok(());
        }
        let program = self.get_program(obj_desc)?;
        program.use_program();
        self.active_program = Some(obj_desc);
        Ok(())
    }

    pub fn detach_program(&mut self) -> Option<GLObjectDescriptor> {
        let obj_desc = self.active_program.take()?;
        let program = self.get_program(obj_desc).ok()?;
        program.detach();
        Some(obj_desc)
    }

    pub fn set_uniform_1i<T: Into<GLint>>(&self, uniform: &str, val: T) -> Result<()> {
        let Some(obj_desc) = self.active_program else {
            return Err(Error::NoActiveProgram);
        };
        self.get_program(obj_desc)
            .and_then(|p| p.set_uniform_i(uniform, val.into()))
    }

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
        Some(obj_desc)
    }

    fn init_texture_units() -> Result<Vec<Option<GLObjectDescriptor>>> {
        let mut max_active_textures = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_active_textures) };
        gl_check_err()?;
        let maximum: usize = try_into!(max_active_textures);
        Ok(vec![None; maximum])
    }

    pub fn try_render(&self) -> Result<()> {
        if self.active_program.is_none() {
            return Err(Error::NoActiveProgram);
        }
        let obj_desc = self.bound_vertex_object.ok_or(Error::NoBoundModel)?;
        let vertex_object = self.get_vertex_object(obj_desc)?;
        vertex_object.render();
        Ok(())
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { gl::Viewport(x, y, width, height) }
    }

    pub fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Deref for GLContext {
    type Target = GLObjectRegistry;

    fn deref(&self) -> &Self::Target {
        &self.object_registry
    }
}

impl DerefMut for GLContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object_registry
    }
}

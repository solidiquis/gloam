use crate::{
    error::{gl_check_err, Result},
    object::{GLObjectDescriptor, GLObjectRegistry},
};
use gl::types::{GLenum, GLint, GLsizei};
use std::ops::{Deref, DerefMut};

pub mod clear;
pub use clear::ClearMask;

pub mod program;
pub mod texture;
pub mod vertex;

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

    pub fn try_enable_depth_test(&self, depth_func: Option<GLenum>) -> Result<()> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            let func = depth_func.unwrap_or(gl::LESS);
            gl::DepthFunc(func);
        }
        gl_check_err()
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { gl::Viewport(x, y, width, height) }
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

use crate::context::GLContext;
use crate::model::Model;
use crate::texture::Texture;
use crate::{Error, Result};
use gl::types::{GLfloat, GLint, GLsizei};
use std::rc::Rc;

#[derive(Default)]
pub struct Frame;

impl Frame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind_model(&mut self, ctx: &mut GLContext, model: Rc<Model>) {
        ctx.set_bound_model(model);
    }

    pub fn activate_texture(
        &mut self,
        ctx: &mut GLContext,
        texture: Rc<Texture>,
        generate_mipmap: bool,
    ) -> Result<()> {
        ctx.activate_texture(texture, generate_mipmap)
    }

    pub fn deactivate_texture(&mut self, ctx: &mut GLContext, texture: Rc<Texture>) {
        ctx.deactivate_texture(texture)
    }

    pub fn unbind_model(&mut self, ctx: &mut GLContext) {
        ctx.unbind_model();
    }

    pub fn render(&mut self, ctx: &mut GLContext) -> Result<()> {
        let model = ctx.bound_model.as_mut().ok_or(Error::NoBoundModel)?;
        model.render();
        Ok(())
    }

    pub fn clear_color(&self, ctx: &mut GLContext, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        ctx.clear_color(r, g, b, a);
    }

    pub fn viewport(&self, ctx: &GLContext, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        ctx.viewport(x, y, width, height);
    }
}

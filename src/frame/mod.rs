use crate::model::Model;
use crate::{Error, Result};
use gl::types::{GLfloat, GLint, GLsizei};

#[derive(Default)]
pub struct Frame<'a> {
    bound_model: Option<&'a Model>,
    close_window: bool,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        Self {
            bound_model: None,
            close_window: false,
        }
    }

    pub fn bind_model(&mut self, model: &'a Model) {
        model.bind();
        model.use_program();
        self.bound_model = Some(model);
    }

    pub fn unbind_model(&mut self) {
        if let Some(model) = self.bound_model.take() {
            model.unbind();
            model.detach_program();
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let model = self.bound_model.as_mut().ok_or(Error::NoBoundModel)?;
        model.render();
        Ok(())
    }

    pub fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { gl::Viewport(x, y, width, height) }
    }

    pub fn close_window(&mut self) {
        self.close_window = true;
    }
}

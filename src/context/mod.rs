use crate::model::Model;
use crate::texture::{Texture, TextureUnit};
use crate::{Error, Result};
use gl::types::{GLenum, GLfloat, GLint, GLsizei};
use glfw::{
    Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint, WindowMode,
};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct GLContext {
    glfw: Glfw,
    window: PWindow,
    events_rx: GlfwReceiver<(f64, WindowEvent)>,
    pub(crate) bound_model: Option<Rc<Model>>,
    pub(crate) active_textures: HashMap<Rc<Texture>, TextureUnit>,
}

pub struct GLContextConfig<'a> {
    pub major_version: u32,
    pub minor_version: u32,
    pub initial_width: u32,
    pub initial_height: u32,
    pub title: &'a str,
    pub window_mode: WindowMode<'a>,
}

impl Default for GLContextConfig<'_> {
    fn default() -> Self {
        Self {
            major_version: 3,
            minor_version: 3,
            initial_width: 800,
            initial_height: 600,
            title: "",
            window_mode: WindowMode::Windowed,
        }
    }
}

impl GLContext {
    pub fn new(config: GLContextConfig) -> Result<Self> {
        let GLContextConfig {
            major_version,
            minor_version,
            initial_height,
            initial_width,
            title,
            window_mode,
        } = config;

        let mut glfw_obj = glfw::init_no_callbacks().map_err(Error::boxed)?;
        glfw_obj.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw_obj.window_hint(WindowHint::ContextVersion(major_version, minor_version));

        #[cfg(target_os = "macos")]
        glfw_obj.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, events_rx) = glfw_obj
            .create_window(initial_width, initial_height, title, window_mode)
            .ok_or(Error::Misc("failed to initialize window".to_string()))?;

        gl::load_with(|sym| window.get_proc_address(sym));
        glfw_obj.make_context_current(Some(&window));

        Ok(Self {
            window,
            events_rx,
            glfw: glfw_obj,
            bound_model: None,
            active_textures: HashMap::new(),
        })
    }

    pub fn bind_model(&mut self, model: Rc<Model>) {
        // model is already bound
        if self.bound_model.as_ref().is_some_and(|m| *m == model) {
            return;
        }
        model.bind_and_use_program();
        self.bound_model = Some(model);
    }

    pub fn unbind_model(&mut self) {
        if let Some(model) = self.bound_model.take() {
            model.unbind_and_detach_program();
        }
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

    pub fn set_uniform_1i<T: Into<GLint>>(&self, uniform: &str, val: T) -> Result<()> {
        let Some(model) = self.bound_model.as_ref() else {
            return Err(Error::NoBoundModel);
        };
        model.program().set_uniform_i(uniform, val.into())
    }

    pub fn activate_texture(
        &mut self,
        texture: Rc<Texture>,
        generate_mipmap: bool,
    ) -> Result<TextureUnit> {
        if let Some(unit) = self.active_textures.get(&texture).cloned() {
            return Ok(unit);
        }

        let unit = if self.active_textures.is_empty() {
            Some(TextureUnit::Texture0)
        } else {
            let active_units = self
                .active_textures
                .values()
                .collect::<HashSet<&TextureUnit>>();
            TextureUnit::iter().find(|u| !active_units.contains(u))
        };

        let Some(unused_unit) = unit else {
            return Err(Error::MaxActiveTextures);
        };

        unsafe { gl::ActiveTexture(GLenum::from(unused_unit)) }
        texture.bind();
        if generate_mipmap {
            texture.generate_mipmap();
        }
        self.active_textures.insert(texture, unused_unit);

        Ok(unused_unit)
    }

    pub fn deactivate_texture(&mut self, texture: Rc<Texture>) {
        let Some(unit) = self.active_textures.remove(&texture) else {
            return;
        };
        unsafe {
            gl::ActiveTexture(GLenum::from(unit));
        }
        texture.unbind();
    }

    pub fn try_render(&self) -> Result<()> {
        let Some(model) = self.bound_model.as_ref() else {
            return Err(Error::NoBoundModel);
        };
        model.render();
        Ok(())
    }

    pub fn draw(&mut self) {
        self.swap_buffers();
    }

    pub fn run_event_loop<F>(&mut self, mut op: F) -> Result<()>
    where
        F: FnMut(&mut Self, Option<WindowEvent>) -> Result<()>,
    {
        while !self.should_close() {
            let event = self.events_rx.receive().map(|(_, ev)| ev);
            op(self, event)?;
            self.glfw.poll_events();
        }
        Ok(())
    }
}

impl Deref for GLContext {
    type Target = PWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for GLContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

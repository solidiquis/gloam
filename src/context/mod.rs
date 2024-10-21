use crate::frame::Frame;
use crate::{Error, Result};
use glfw::{
    Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint, WindowMode,
};
use std::ops::{Deref, DerefMut};

pub struct GLContext {
    glfw: Glfw,
    window: PWindow,
    events_rx: GlfwReceiver<(f64, WindowEvent)>,
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
        })
    }

    pub fn new_frame<'a>(&self) -> Frame<'a> {
        Frame::new()
    }

    pub fn run_event_loop<F>(mut self, mut op: F) -> Result<()>
    where
        F: FnMut(&mut Self, Option<WindowEvent>) -> Result<()>,
    {
        while !self.should_close() {
            let event = self.events_rx.receive().map(|(_, ev)| ev);
            op(&mut self, event)?;
            self.swap_buffers();
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

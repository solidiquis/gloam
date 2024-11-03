use crate::error::{Error, Result};
use glfw::{
    Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint, WindowMode,
};
use std::ops::{Deref, DerefMut};

pub struct Window {
    inner: PWindow,
    glfw: Glfw,
    events_rx: GlfwReceiver<(f64, WindowEvent)>,
}

pub struct WindowConfig<'a> {
    pub gl_major_version: u32,
    pub gl_minor_version: u32,
    pub initial_width: u32,
    pub initial_height: u32,
    pub title: &'a str,
    pub window_mode: WindowMode<'a>,
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            gl_major_version: 3,
            gl_minor_version: 3,
            initial_width: 800,
            initial_height: 600,
            title: "",
            window_mode: WindowMode::Windowed,
        }
    }
}

impl Window {
    pub fn new(config: WindowConfig<'_>) -> Result<Self> {
        let WindowConfig {
            gl_major_version,
            gl_minor_version,
            initial_height,
            initial_width,
            title,
            window_mode,
        } = config;

        let mut glfw_obj = glfw::init_no_callbacks().map_err(Error::boxed)?;
        glfw_obj.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw_obj.window_hint(WindowHint::ContextVersion(
            gl_major_version,
            gl_minor_version,
        ));

        #[cfg(target_os = "macos")]
        glfw_obj.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, events_rx) = glfw_obj
            .create_window(initial_width, initial_height, title, window_mode)
            .ok_or(Error::Misc("failed to initialize window".to_string()))?;

        gl::load_with(|sym| window.get_proc_address(sym));
        glfw_obj.make_context_current(Some(&window));

        Ok(Self {
            events_rx,
            glfw: glfw_obj,
            inner: window,
        })
    }

    pub fn draw(&mut self) {
        self.swap_buffers();
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn run_event_loop<F>(mut self, mut op: F) -> Result<()>
    where
        F: FnMut(&mut Self, Option<WindowEvent>) -> Result<()>,
    {
        while !self.should_close() {
            let event = self.events_rx.receive().map(|(_, ev)| ev);
            op(&mut self, event)?;
            self.glfw.poll_events();
        }
        Ok(())
    }
}

impl Deref for Window {
    type Target = PWindow;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

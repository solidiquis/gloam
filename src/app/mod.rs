use crate::{
    context::GLContext,
    error::Result,
    object::GLObjectRegistry,
    window::{Window, WindowConfig},
};

pub fn init_default_opengl_3_3(title: &str) -> Result<(Window, GLContext)> {
    let window_config = WindowConfig {
        title,
        ..Default::default()
    };
    let window = Window::new(window_config)?;
    let object_registry = GLObjectRegistry::default();
    let ctx = GLContext::new(object_registry)?;
    Ok((window, ctx))
}

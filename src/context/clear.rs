use super::GLContext;
use gl::types::{GLbitfield, GLfloat};

#[derive(Debug, Copy, Clone)]
pub enum ClearMask {
    // RGBA
    Color(GLfloat, GLfloat, GLfloat, GLfloat),
    DepthBuffer,
}

impl From<ClearMask> for GLbitfield {
    fn from(value: ClearMask) -> Self {
        match value {
            ClearMask::Color(_, _, _, _) => gl::COLOR_BUFFER_BIT,
            ClearMask::DepthBuffer => gl::DEPTH_BUFFER_BIT,
        }
    }
}

impl GLContext {
    pub fn clear(&self, masks: &[ClearMask]) {
        if masks.is_empty() {
            return;
        }

        let mut bitmasks = Vec::with_capacity(masks.len());
        for mask in masks {
            bitmasks.push(GLbitfield::from(*mask));
            match *mask {
                ClearMask::Color(r, g, b, a) => unsafe {
                    gl::ClearColor(r, g, b, a);
                },
                _ => continue,
            }
        }
        if let Some(bitmask) = bitmasks.into_iter().reduce(|a, b| a | b) {
            unsafe { gl::Clear(bitmask) }
        }
    }
}

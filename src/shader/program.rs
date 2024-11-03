use super::Shader;
use crate::{
    context::GLContext,
    error::{Error, Result},
    object::{GLObject, GLObjectDescriptor},
};
use std::{mem, ops::Drop, ptr};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub gl_object_id: gl::types::GLuint,
}

pub struct Linker {
    shaders: Vec<Shader>,
    program: gl::types::GLuint,
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}

impl Linker {
    pub fn new() -> Self {
        let shaders = vec![];
        let program = unsafe { gl::CreateProgram() };
        Self { shaders, program }
    }

    pub fn attach_shader(mut self, shader: Shader) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn link(self, ctx: &mut GLContext) -> Result<GLObjectDescriptor> {
        unsafe {
            for Shader(shader) in &self.shaders {
                gl::AttachShader(self.program, *shader);
            }

            gl::LinkProgram(self.program);

            let mut success = 0;
            gl::GetProgramiv(self.program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut log: [u8; 512] = [0; 512];
                gl::GetProgramInfoLog(
                    self.program,
                    (mem::size_of::<u8>() * log.len()).try_into().unwrap(),
                    ptr::null_mut(),
                    log.as_mut_ptr() as *mut i8,
                );
                let reason = String::from_utf8_lossy(&log);

                return Err(Error::ProgramLink(String::from(reason.trim())));
            }

            for Shader(shader) in self.shaders {
                gl::DeleteShader(shader);
            }
            let program = Program {
                gl_object_id: self.program,
            };
            let obj_dec = ctx.register_object(GLObject::Program(program));
            Ok(obj_dec)
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_object_id);
        }
        self.gl_object_id = 0;
    }
}

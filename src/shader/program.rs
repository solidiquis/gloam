use super::Shader;
use crate::{Error, Result};
use gl::types::GLint;
use std::{ffi::CString, mem, ops::Drop, ptr};

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

impl Program {
    pub(crate) fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.gl_object_id);
        }
    }

    pub(crate) fn get_attrib_loc(&self, attrib: &str) -> Result<gl::types::GLuint> {
        let c_attrib = CString::new(attrib).map_err(Error::boxed)?;
        unsafe {
            let loc = gl::GetAttribLocation(self.gl_object_id, c_attrib.as_ptr());
            if loc == -1 {
                return Err(Error::AttributeLocNotFound(attrib.into()));
            }
            Ok(loc as gl::types::GLuint)
        }
    }

    pub(crate) fn get_uniform_loc(&self, uniform: &str) -> Result<gl::types::GLint> {
        let c_uniform = CString::new(uniform).map_err(Error::boxed)?;
        unsafe {
            let loc = gl::GetUniformLocation(self.gl_object_id, c_uniform.as_ptr());
            if loc == -1 {
                return Err(Error::UniformLocNotFound(uniform.into()));
            }
            Ok(loc)
        }
    }

    pub(crate) fn set_uniform_i(&self, uniform: &str, value: GLint) -> Result<()> {
        let loc = self.get_uniform_loc(uniform)?;
        unsafe {
            gl::Uniform1i(loc, value);
        }
        Ok(())
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

    pub fn link(self) -> Result<Program> {
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

            Ok(Program {
                gl_object_id: self.program,
            })
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

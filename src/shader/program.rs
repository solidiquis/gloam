use super::Shader;
use crate::{
    context::GLContext,
    error::{Error, Result},
    object::{GLObject, GLObjectDescriptor},
};
use std::{fmt::Debug, mem, ops::Drop, path::PathBuf, ptr};

#[derive(PartialEq, Eq)]
pub struct Program {
    pub gl_object_id: gl::types::GLuint,
    shader_src_paths: Vec<PathBuf>,
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
            let mut shader_src_paths = Vec::with_capacity(self.shaders.len());

            for Shader { gl_object_id, src } in &self.shaders {
                gl::AttachShader(self.program, *gl_object_id);
                shader_src_paths.push(src.clone());
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

            for Shader { gl_object_id, .. } in self.shaders {
                gl::DeleteShader(gl_object_id);
            }
            let program = Program {
                shader_src_paths,
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

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shader_srcs = self
            .shader_src_paths
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join(",");
        write!(
            f,
            "Program {{ gl_object_id={}, src='{shader_srcs}' }}",
            self.gl_object_id
        )
    }
}

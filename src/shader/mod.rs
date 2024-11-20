use super::try_into;
use crate::{Error, Result};
use std::{
    convert::AsRef,
    ffi::{CString, OsStr},
    fs, mem,
    path::{Path, PathBuf},
    ptr,
};

pub mod program;
pub use program::Program;

pub struct Shader {
    pub gl_object_id: gl::types::GLuint,
    pub src: PathBuf,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

impl From<ShaderType> for gl::types::GLenum {
    fn from(shader_type: ShaderType) -> Self {
        match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl Shader {
    pub fn new<P: AsRef<Path>>(src: P, typ: ShaderType) -> Result<Self> {
        let shader_path = src.as_ref();
        let shader_src = fs::read_to_string(src.as_ref()).map_err(Error::boxed)?;
        let file_name = shader_path
            .file_name()
            .map(OsStr::to_string_lossy)
            .ok_or(Error::Misc(
                "expected shader to have a file name".to_string(),
            ))?;
        let gl_object_id = Self::compile_src(&file_name, &shader_src, typ)?;
        Ok(Self {
            gl_object_id,
            src: shader_path.to_path_buf(),
        })
    }

    fn compile_src(file_name: &str, src: &str, typ: ShaderType) -> Result<gl::types::GLuint> {
        let shader_src = CString::new(src).map_err(Error::boxed)?;

        unsafe {
            let shader = gl::CreateShader(typ.into());
            gl::ShaderSource(shader, 1, &shader_src.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut log: [u8; 512] = [0; 512];
                gl::GetShaderInfoLog(
                    shader,
                    try_into!(mem::size_of::<u8>() * log.len()),
                    ptr::null_mut(),
                    log.as_mut_ptr() as *mut i8,
                );
                let reason = String::from_utf8_lossy(&log);

                return Err(Error::ShaderCompile {
                    shader_name: String::from(file_name),
                    reason: String::from(reason.trim()),
                });
            };
            Ok(shader)
        }
    }
}

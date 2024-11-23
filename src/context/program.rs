use super::GLContext;
use crate::{
    error::{Error, Result},
    internal_utils::as_gl_bool,
    object::GLObjectDescriptor,
    uniform::{Uniform, UniformType},
};
use nalgebra_glm as glm;
use std::ffi::CString;

impl GLContext {
    /// Will return an error if a different program is currently active; if that's the case, it
    /// should be detached before using another program.
    pub fn try_use_program(&mut self, prog_desc: GLObjectDescriptor) -> Result<()> {
        // Program is already in use
        if self.active_program.is_some_and(|desc| desc == prog_desc) {
            return Ok(());
        } else if self.active_program.is_some() {
            return Err(Error::AnotherProgramInUse);
        }
        let program = self.get_program(prog_desc)?;
        unsafe {
            gl::UseProgram(program.gl_object_id);
        }
        log::debug!("currently active program: object_storage_id={prog_desc:?} -> {program:?}");
        self.active_program = Some(prog_desc);
        Ok(())
    }

    /// Detach the program that's currently in use.
    pub fn detach_current_program(&mut self) -> Option<GLObjectDescriptor> {
        let active_program_desc = self.active_program.take()?;
        unsafe {
            gl::UseProgram(0);
        }
        log::debug!("program detached: object_storage_id={active_program_desc:?}");
        Some(active_program_desc)
    }

    pub fn try_get_attrib_loc(
        &self,
        prog_desc: GLObjectDescriptor,
        attrib: &str,
    ) -> Result<gl::types::GLuint> {
        self.ensure_program_active()?;
        let c_attrib = CString::new(attrib).map_err(Error::boxed)?;

        self.get_program(prog_desc).and_then(|prog| unsafe {
            let loc = gl::GetAttribLocation(prog.gl_object_id, c_attrib.as_ptr());
            if loc == -1 {
                return Err(Error::AttributeLocNotFound(attrib.into()));
            }
            Ok(loc as gl::types::GLuint)
        })
    }

    pub fn try_get_uniform_loc(&self, uniform: &str) -> Result<gl::types::GLint> {
        let prog_desc = self.try_get_active_program()?;
        let c_uniform = CString::new(uniform).map_err(Error::boxed)?;

        self.get_program(prog_desc).and_then(|prog| unsafe {
            let loc = gl::GetUniformLocation(prog.gl_object_id, c_uniform.as_ptr());
            if loc == -1 {
                return Err(Error::UniformLocNotFound(uniform.into()));
            }
            Ok(loc)
        })
    }

    pub fn try_set_uniform(&self, uniform: &Uniform) -> Result<()> {
        self.ensure_program_active()?;
        let transpose = as_gl_bool(uniform.transpose);
        self.try_get_uniform_loc(&uniform.name).map(|loc| unsafe {
            match uniform.kind {
                UniformType::D(v) => gl::Uniform1d(loc, v),
                UniformType::F(v) => gl::Uniform1f(loc, v),
                UniformType::I(v) => gl::Uniform1i(loc, v),
                UniformType::D2(v) => gl::Uniform2d(loc, v.x, v.y),
                UniformType::F2(v) => gl::Uniform2f(loc, v.x, v.y),
                UniformType::I2(v) => gl::Uniform2i(loc, v.x, v.y),
                UniformType::D3(v) => gl::Uniform3d(loc, v.x, v.y, v.z),
                UniformType::F3(v) => gl::Uniform3f(loc, v.x, v.y, v.z),
                UniformType::I3(v) => gl::Uniform3i(loc, v.x, v.y, v.z),
                UniformType::D4(v) => gl::Uniform4d(loc, v.x, v.y, v.z, v.w),
                UniformType::F4(v) => gl::Uniform4f(loc, v.x, v.y, v.z, v.w),
                UniformType::I4(v) => gl::Uniform4i(loc, v.x, v.y, v.z, v.w),
                UniformType::DMat2(v) => {
                    gl::UniformMatrix2dv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
                UniformType::FMat2(v) => {
                    gl::UniformMatrix2fv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
                UniformType::DMat3(v) => {
                    gl::UniformMatrix3dv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
                UniformType::FMat3(v) => {
                    gl::UniformMatrix3fv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
                UniformType::DMat4(v) => {
                    gl::UniformMatrix4dv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
                UniformType::FMat4(v) => {
                    gl::UniformMatrix4fv(loc, 1, transpose, glm::value_ptr(&v).as_ptr())
                }
            }
        })
    }

    pub fn try_get_active_program(&self) -> Result<GLObjectDescriptor> {
        self.active_program.ok_or(Error::NoActiveProgram)
    }

    pub fn ensure_program_active(&self) -> Result<()> {
        if self.active_program.is_none() {
            return Err(Error::NoActiveProgram);
        }
        Ok(())
    }
}

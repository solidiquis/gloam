use super::GLContext;
use crate::{
    error::{Error, Result},
    internal_utils::as_gl_bool,
    object::GLObjectDescriptor,
};
use gl::types::{GLfloat, GLint};
use std::ffi::CString;

impl GLContext {
    pub fn try_use_program(&mut self, prog_desc: GLObjectDescriptor) -> Result<()> {
        if self.active_program.is_some_and(|desc| desc == prog_desc) {
            return Ok(());
        }
        let program = self.get_program(prog_desc)?;
        unsafe {
            gl::UseProgram(program.gl_object_id);
        }
        log::debug!("currently active program: object_storage_id={prog_desc:?} -> {program:?}");
        self.active_program = Some(prog_desc);
        Ok(())
    }

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

    pub fn try_set_uniform_1i(&self, uniform: &str, value: GLint) -> Result<()> {
        self.ensure_program_active()?;
        self.try_get_uniform_loc(uniform).map(|loc| unsafe {
            gl::Uniform1i(loc, value);
        })
    }

    pub fn try_set_uniform_1f(&self, uniform: &str, value: GLfloat) -> Result<()> {
        self.ensure_program_active()?;
        self.try_get_uniform_loc(uniform).map(|loc| unsafe {
            gl::Uniform1f(loc, value);
        })
    }

    pub fn try_set_uniform_matrix_3fv(
        &self,
        uniform: &str,
        data: &[GLfloat],
        transpose: bool,
    ) -> Result<()> {
        self.ensure_program_active()?;
        self.try_get_uniform_loc(uniform).map(|loc| unsafe {
            gl::UniformMatrix3fv(loc, 1, as_gl_bool(transpose), data.as_ptr());
        })
    }

    pub fn try_set_uniform_matrix_4fv(
        &self,
        uniform: &str,
        data: &[GLfloat],
        transpose: bool,
    ) -> Result<()> {
        self.ensure_program_active()?;
        self.try_get_uniform_loc(uniform).map(|loc| unsafe {
            gl::UniformMatrix4fv(loc, 1, as_gl_bool(transpose), data.as_ptr());
        })
    }

    pub fn try_set_uniform_3f(
        &self,
        uniform: &str,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) -> Result<()> {
        self.ensure_program_active()?;
        self.try_get_uniform_loc(uniform).map(|loc| unsafe {
            gl::Uniform3f(loc, x, y, z);
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

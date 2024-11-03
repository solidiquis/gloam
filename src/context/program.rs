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
        // program already in use
        if self.program_active(prog_desc) {
            return Ok(());
        }
        let program = self.get_program(prog_desc)?;
        unsafe {
            gl::UseProgram(program.gl_object_id);
        }
        self.active_program = Some(prog_desc);
        Ok(())
    }

    pub fn detach_current_program(&mut self) -> Option<GLObjectDescriptor> {
        let active_program = self.active_program.take()?;
        unsafe {
            gl::UseProgram(0);
        }
        Some(active_program)
    }

    pub fn try_get_attrib_loc(
        &self,
        prog_desc: GLObjectDescriptor,
        attrib: &str,
    ) -> Result<gl::types::GLuint> {
        self.ensure_program_active(prog_desc)?;

        let prog = self.get_program(prog_desc)?;

        let c_attrib = CString::new(attrib).map_err(Error::boxed)?;
        unsafe {
            let loc = gl::GetAttribLocation(prog.gl_object_id, c_attrib.as_ptr());
            if loc == -1 {
                return Err(Error::AttributeLocNotFound(attrib.into()));
            }
            Ok(loc as gl::types::GLuint)
        }
    }

    pub fn try_get_uniform_loc(
        &self,
        prog_desc: GLObjectDescriptor,
        uniform: &str,
    ) -> Result<gl::types::GLint> {
        self.ensure_program_active(prog_desc)?;
        let prog = self.get_program(prog_desc)?;

        let c_uniform = CString::new(uniform).map_err(Error::boxed)?;
        unsafe {
            let loc = gl::GetUniformLocation(prog.gl_object_id, c_uniform.as_ptr());
            if loc == -1 {
                return Err(Error::UniformLocNotFound(uniform.into()));
            }
            Ok(loc)
        }
    }

    pub fn try_set_uniform_1i(
        &self,
        prog_desc: GLObjectDescriptor,
        uniform: &str,
        value: GLint,
    ) -> Result<()> {
        self.ensure_program_active(prog_desc)?;
        let loc = self.try_get_uniform_loc(prog_desc, uniform)?;
        unsafe {
            gl::Uniform1i(loc, value);
        }
        Ok(())
    }

    pub fn try_set_uniform_matrix_4fv(
        &self,
        prog_desc: GLObjectDescriptor,
        uniform: &str,
        data: &[GLfloat],
        transpose: bool,
    ) -> Result<()> {
        self.ensure_program_active(prog_desc)?;
        let loc = self.try_get_uniform_loc(prog_desc, uniform)?;
        unsafe {
            gl::UniformMatrix4fv(loc, 1, as_gl_bool(transpose), data.as_ptr());
        }
        Ok(())
    }

    pub fn program_active(&self, prog_desc: GLObjectDescriptor) -> bool {
        self.active_program.is_some_and(|p| p == prog_desc)
    }

    fn ensure_program_active(&self, prog_desc: GLObjectDescriptor) -> Result<()> {
        if !self.program_active(prog_desc) {
            return Err(Error::ProgramNotInUse);
        }
        Ok(())
    }
}

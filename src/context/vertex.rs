use super::GLContext;
use crate::{
    error::{Error, Result},
    internal_utils::try_into,
    object::GLObjectDescriptor,
};
use gl::types::GLenum;
use std::ptr;

impl GLContext {
    pub fn try_render(&self) -> Result<()> {
        self.ensure_program_active()?;
        let obj_desc = self.bound_vertex_object.ok_or(Error::NoBoundVertexObject)?;
        let vo = self.get_vertex_object(obj_desc)?;

        unsafe {
            let primitive = GLenum::from(vo.primitive);

            if let Some(ibo) = vo.index_buffer_object.as_ref() {
                gl::DrawElements(
                    primitive,
                    try_into!(ibo.indexes.len()),
                    gl::UNSIGNED_INT,
                    ptr::null(),
                )
            } else {
                gl::DrawArrays(primitive, 0, try_into!(vo.num_vertices));
            }
        }

        Ok(())
    }

    /// Will return if a different vertex object (i.e. VAO) is currently bound. Detach the current before
    /// binding a new one.
    pub fn try_bind_vertex_object(&mut self, vo_desc: GLObjectDescriptor) -> Result<()> {
        if self.bound_vertex_object.is_some_and(|vo| vo == vo_desc) {
            return Ok(());
        } else if self.bound_vertex_object.is_some() {
            return Err(Error::AnotherVertexObjectBound);
        }
        let vo = self.get_vertex_object(vo_desc)?;
        unsafe {
            gl::BindVertexArray(vo.vertex_array_object);
            if let Some(ibo) = vo.index_buffer_object.as_ref() {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo.gl_object_id);
            }
        }
        log::debug!("currently bound vertex object: object_storage_id={vo_desc:?} -> {vo:?}");
        self.bound_vertex_object = Some(vo_desc);
        Ok(())
    }

    /// Unbinds the current vertex object i.e. VAO.
    pub fn unbind_current_vertex_object(&mut self) -> Option<GLObjectDescriptor> {
        let obj_desc = self.bound_vertex_object.take()?;
        let vo = self.get_vertex_object(obj_desc).ok()?;
        unsafe {
            gl::BindVertexArray(0);
            if vo.index_buffer_object.is_some() {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
        }
        log::debug!("vertex object unbound: object_storage_id={obj_desc:?}");
        Some(obj_desc)
    }

    pub fn vertex_object_bound(&self, vo_desc: GLObjectDescriptor) -> bool {
        self.bound_vertex_object.is_some_and(|od| od == vo_desc)
    }

    pub fn get_current_bound_vertex_object(&self) -> Option<GLObjectDescriptor> {
        self.bound_vertex_object
    }
}

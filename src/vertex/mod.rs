#![allow(dead_code)]
use crate::{
    context::GLContext,
    error::{Error, Result},
    internal_utils::{as_gl_bool, try_into},
    object::{GLObject, GLObjectDescriptor},
};
use gl::types::{GLenum, GLint, GLuint};
use std::{ffi::c_void, marker::PhantomData, mem, ops::Drop};

pub mod primitives;
pub use primitives::Primitive;

pub mod usage;
pub use usage::Usage;

#[derive(Debug)]
pub struct VertexObject {
    pub(crate) vertex_array_object: GLuint,
    pub(crate) vertex_buffer_object: GLuint,
    pub(crate) index_buffer_object: Option<IndexObject>,
    pub(crate) attributes: Vec<VertexAttribute>,
    pub(crate) num_vertices: GLint,
    pub(crate) primitive: Primitive,
    pub(crate) usage: Usage,
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct VertexAttribute {
    pub(crate) name: String,
    pub(crate) component_size: GLint,
    #[derivative(Debug = "ignore")]
    pub(crate) data: Vec<f32>,
    pub(crate) normalized: bool,
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct IndexObject {
    pub(crate) gl_object_id: GLuint,
    #[derivative(Debug = "ignore")]
    pub(crate) indexes: Vec<u32>,
}

pub struct VertexObjectBuilder<T> {
    pub(crate) num_vertices: GLint,
    pub(crate) attributes: Vec<VertexAttribute>,
    pub(crate) indexes: Option<Vec<u32>>,
    pub(crate) usage: Usage,
    pub(crate) primitive: Primitive,
    pub(crate) state: PhantomData<T>,
}

pub type VOBuilder = VertexObjectBuilder<VOBInit>;

pub struct VOBInit;
pub struct VOBAttr;

impl<T> VertexObjectBuilder<T> {
    pub fn new(primitive: Primitive, usage: Usage) -> VertexObjectBuilder<VOBInit> {
        VertexObjectBuilder {
            usage,
            primitive,
            attributes: Vec::new(),
            indexes: None,
            num_vertices: 0,
            state: PhantomData {},
        }
    }
}

impl VertexObjectBuilder<VOBInit> {
    pub fn attribute(
        mut self,
        name: &str,
        component_size: GLint,
        data: &[f32],
    ) -> Result<VertexObjectBuilder<VOBAttr>> {
        if data.is_empty() {
            return Err(Error::InvalidVertexObject(
                "vertex attribute must have data".to_string(),
            ));
        }
        let num_values: GLint = try_into!(data.len());
        if num_values % component_size != 0 {
            return Err(Error::InvalidVertexObject(format!(
                "length of data ({num_values}) must be evenly divisible by component size ({component_size})"
            )));
        }
        let num_vertices = num_values / component_size;

        self.attributes.push(VertexAttribute {
            component_size,
            name: name.to_string(),
            data: data.to_vec(),
            normalized: false,
        });
        Ok(VertexObjectBuilder {
            num_vertices,
            attributes: self.attributes,
            indexes: self.indexes,
            usage: self.usage,
            primitive: self.primitive,
            state: PhantomData {},
        })
    }
}

impl VertexObjectBuilder<VOBAttr> {
    pub fn attribute(mut self, name: &str, component_size: GLint, data: &[f32]) -> Result<Self> {
        if data.is_empty() {
            return Err(Error::InvalidVertexObject(
                "vertex attribute must have data".to_string(),
            ));
        }

        let num_values: GLint = try_into!(data.len());
        let new_attr_num_vertices = num_values / component_size;

        if self.num_vertices != new_attr_num_vertices {
            return Err(Error::InvalidVertexObject(format!(
                "number of vertices for attribute '{name}' doesn't match number of vertices for previous attributes"
            )));
        }
        self.attributes.push(VertexAttribute {
            component_size,
            name: name.to_string(),
            data: data.to_vec(),
            normalized: false,
        });
        Ok(self)
    }

    pub fn indexes(mut self, indexes: &[u32]) -> Result<Self> {
        let first_attr = self.attributes.first().unwrap();
        let data_len: GLint = try_into!(first_attr.data.len());
        let num_vertices = data_len / first_attr.component_size;

        let max_index = indexes.iter().max().copied().unwrap_or_default();
        if max_index > try_into!(num_vertices - 1) {
            return Err(Error::InvalidVertexObject(format!(
                "index value {max_index} out of bounds with number of vertices, {num_vertices}"
            )));
        }
        self.indexes = Some(indexes.to_vec());
        Ok(self)
    }

    pub fn build(
        self,
        ctx: &mut GLContext,
        program: GLObjectDescriptor,
    ) -> Result<GLObjectDescriptor> {
        unsafe { self.build_impl(ctx, program) }
    }

    unsafe fn build_impl(
        self,
        ctx: &mut GLContext,
        program: GLObjectDescriptor,
    ) -> Result<GLObjectDescriptor> {
        let VertexObjectBuilder {
            attributes,
            indexes,
            usage,
            primitive,
            num_vertices,
            ..
        } = self;

        if attributes.is_empty() {
            return Err(Error::Misc(
                "can't create an empty vertex object".to_string(),
            ));
        }

        let mut buffer = Vec::with_capacity(attributes.iter().map(|a| a.data.len()).sum::<usize>());
        let size_of_f32: GLint = try_into!(mem::size_of::<f32>());
        let stride = size_of_f32 * attributes.iter().map(|a| a.component_size).sum::<GLint>();
        let usage_enum = GLenum::from(usage);

        let mut chunk_iters = attributes
            .iter()
            .map(|attr| attr.data.chunks(try_into!(attr.component_size)))
            .collect::<Vec<_>>();

        for i in (0..chunk_iters.len()).cycle() {
            let Some(chunk) = chunk_iters.get_mut(i).and_then(|c| c.next()) else {
                break;
            };
            buffer.extend_from_slice(chunk);
        }

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            try_into!(mem::size_of_val(buffer.as_slice())),
            buffer.as_ptr() as *const c_void,
            usage_enum,
        );

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut byte_offset = 0;

        ctx.try_use_program(program)?;

        for attribute in &attributes {
            let attr_loc = ctx.try_get_attrib_loc(program, &attribute.name)?;
            gl::VertexAttribPointer(
                attr_loc,
                attribute.component_size,
                gl::FLOAT,
                as_gl_bool(attribute.normalized),
                stride,
                byte_offset as *const c_void,
            );
            gl::EnableVertexAttribArray(attr_loc);
            byte_offset += size_of_f32 * attribute.component_size;
        }

        let index_buffer_object = indexes.map(|indexes| {
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                try_into!(mem::size_of_val(indexes.as_slice())),
                indexes.as_ptr() as *const c_void,
                usage_enum,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            IndexObject {
                indexes,
                gl_object_id: ebo,
            }
        });

        ctx.detach_current_program();
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        let vertex_object = VertexObject {
            num_vertices,
            attributes,
            primitive,
            usage,
            index_buffer_object,
            vertex_array_object: vao,
            vertex_buffer_object: vbo,
        };
        let obj_desc = ctx.register_object(GLObject::VertexObject(vertex_object));
        Ok(obj_desc)
    }
}

impl Drop for VertexObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array_object);
            self.vertex_array_object = 0;
            gl::DeleteBuffers(1, &self.vertex_buffer_object);
            self.vertex_buffer_object = 0;
        }
    }
}

impl Drop for IndexObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_object_id);
            self.gl_object_id = 0;
        }
    }
}

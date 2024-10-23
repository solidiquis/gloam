use crate::internal_utils::{as_gl_bool, try_into};
use crate::shader::program::Program;
use crate::{Error, Result};
use std::{ffi::c_void, mem, ops::Drop, ptr};

pub mod usage;
use usage::Usage;

pub mod primitives;
use primitives::Primitive;

#[derive(Debug, PartialEq, Eq)]
pub struct Model {
    vertex_array_object: gl::types::GLuint,
    element_buffer_object: Option<gl::types::GLuint>,
    program: Program,
    primitive: Primitive,
    num_vertices: gl::types::GLsizei,
    num_indices: gl::types::GLsizei,
}

pub struct ModelBuilder {
    position_attributes: VertexAttribute,
    color_attributes: Option<VertexAttribute>,
    texture_attributes: Option<VertexAttribute>,
    indices: Option<Vec<u32>>,
    usage: Usage,
    program: Program,
    primitive: Primitive,

    vbo_num_elements: gl::types::GLsizei,
    stride: gl::types::GLsizei,
}

pub struct VertexAttribute {
    name: String,
    values: Vec<f32>,
    component_size: gl::types::GLint,
    normalized: bool,
}

impl VertexAttribute {
    pub fn new(
        attr: &str,
        values: Vec<f32>,
        component_size: gl::types::GLint,
        normalized: bool,
    ) -> Self {
        Self {
            name: attr.to_string(),
            values,
            component_size,
            normalized,
        }
    }
}

impl Model {
    pub(crate) fn render(&self) {
        unsafe {
            let primitive: gl::types::GLenum = self.primitive.into();

            if self.element_buffer_object.is_some() {
                gl::DrawElements(primitive, self.num_indices, gl::UNSIGNED_INT, ptr::null())
            } else {
                gl::DrawArrays(primitive, 0, try_into!(self.num_vertices));
            }
        }
    }

    pub(crate) fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program.gl_object_id);
        }
    }

    pub(crate) fn detach_program(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
            if let Some(ebo) = self.element_buffer_object {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            }
        }
    }

    pub(crate) fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
            if self.element_buffer_object.is_some() {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
        }
    }

    pub(crate) fn unbind_and_detach_program(&self) {
        self.unbind();
        self.detach_program();
    }

    pub(crate) fn bind_and_use_program(&self) {
        self.bind();
        self.use_program();
    }
}

impl ModelBuilder {
    pub fn new(
        program: Program,
        usage: Usage,
        primitive: Primitive,
        position_attributes: VertexAttribute,
    ) -> Result<Self> {
        let num_values = gl::types::GLint::try_from(position_attributes.values.len()).unwrap();
        if num_values % position_attributes.component_size != 0 {
            return Err(Error::ComponentSizeValuesMistmatch("position"));
        }
        let stride = gl::types::GLint::try_from(mem::size_of::<f32>()).unwrap()
            * position_attributes.component_size;

        Ok(Self {
            program,
            primitive,
            usage,
            position_attributes,
            stride,
            indices: None,
            vbo_num_elements: num_values,
            color_attributes: None,
            texture_attributes: None,
        })
    }

    pub fn indices(mut self, indices: Vec<u32>) -> Result<Self> {
        let Some(max_index) = indices.iter().max().copied() else {
            return Ok(self);
        };

        let num_vertices = gl::types::GLsizei::try_from(self.position_attributes.values.len())
            .unwrap()
            / self.position_attributes.component_size;

        if max_index > u32::try_from(num_vertices - 1).unwrap() {
            return Err(Error::InvalidIndex {
                index: max_index,
                num_vertices: num_vertices - 1,
            });
        }

        self.indices = Some(indices);
        Ok(self)
    }

    pub fn color_attributes(mut self, vertices: VertexAttribute) -> Result<Self> {
        let num_values = gl::types::GLsizei::try_from(vertices.values.len()).unwrap();
        if num_values % vertices.component_size != 0 {
            return Err(Error::ComponentSizeValuesMistmatch("color"));
        }
        self.vbo_num_elements += num_values;
        self.stride +=
            gl::types::GLsizei::try_from(mem::size_of::<f32>()).unwrap() * vertices.component_size;
        self.color_attributes = Some(vertices);
        Ok(self)
    }

    pub fn texture_attributes(mut self, vertices: VertexAttribute) -> Result<Self> {
        let num_values = gl::types::GLsizei::try_from(vertices.values.len()).unwrap();
        if num_values % vertices.component_size != 0 {
            return Err(Error::ComponentSizeValuesMistmatch("texture"));
        }
        self.vbo_num_elements += num_values;
        self.stride +=
            gl::types::GLsizei::try_from(mem::size_of::<f32>()).unwrap() * vertices.component_size;
        self.texture_attributes = Some(vertices);
        Ok(self)
    }

    pub fn build(self) -> Result<Model> {
        unsafe { self.build_impl() }
    }

    unsafe fn build_impl(self) -> Result<Model> {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let pos_component_size = usize::try_from(self.position_attributes.component_size).unwrap();
        let num_vertices = self.position_attributes.values.len() / pos_component_size;

        let mut attribute_data_iters =
            vec![self.position_attributes.values.chunks(pos_component_size)];

        if let Some(color_attrs) = self.color_attributes.as_ref() {
            let col_component_size = usize::try_from(color_attrs.component_size).unwrap();
            if color_attrs.values.len() / col_component_size != num_vertices {
                return Err(Error::AttributeValuesMistmatch {
                    attr_name_a: "color",
                    attr_name_b: "position",
                });
            }
            attribute_data_iters.push(color_attrs.values.chunks(col_component_size));
        }

        if let Some(tex_attrs) = self.texture_attributes.as_ref() {
            let tex_component_size = usize::try_from(tex_attrs.component_size).unwrap();
            if tex_attrs.values.len() / tex_component_size != num_vertices {
                return Err(Error::AttributeValuesMistmatch {
                    attr_name_a: "texture",
                    attr_name_b: "position",
                });
            }
            attribute_data_iters.push(tex_attrs.values.chunks(tex_component_size));
        }

        let vbo_num_elements = usize::try_from(self.vbo_num_elements).unwrap();
        let mut buffer = Vec::with_capacity(vbo_num_elements);

        for i in (0..attribute_data_iters.len()).cycle() {
            let Some(chunk) = attribute_data_iters.get_mut(i).and_then(|a| a.next()) else {
                break;
            };
            buffer.extend_from_slice(chunk);
        }

        gl::BufferData(
            gl::ARRAY_BUFFER,
            try_into!(mem::size_of_val(buffer.as_slice())),
            buffer.as_ptr() as *const c_void,
            self.usage.into(),
        );

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut byte_offset = mem::size_of::<f32>() * pos_component_size;

        /*
         * Position attribute
         */
        let VertexAttribute {
            name,
            component_size,
            normalized,
            ..
        } = &self.position_attributes;
        let pos_attr_loc = self.program.get_attrib_loc(name)?;
        gl::VertexAttribPointer(
            pos_attr_loc,
            *component_size,
            gl::FLOAT,
            as_gl_bool(*normalized),
            try_into!(self.stride),
            ptr::null(),
        );
        gl::EnableVertexAttribArray(pos_attr_loc);

        /*
         * Color attribute
         */
        if let Some(VertexAttribute {
            name,
            component_size,
            normalized,
            ..
        }) = self.color_attributes.as_ref()
        {
            let col_attr_loc = self.program.get_attrib_loc(name)?;
            gl::VertexAttribPointer(
                col_attr_loc,
                *component_size,
                gl::FLOAT,
                as_gl_bool(*normalized),
                self.stride,
                byte_offset as *const c_void,
            );
            gl::EnableVertexAttribArray(col_attr_loc);
            byte_offset += mem::size_of::<f32>() * usize::try_from(*component_size).unwrap();
        }

        /*
         * Texture attribute
         */
        if let Some(VertexAttribute {
            name,
            component_size,
            normalized,
            ..
        }) = self.texture_attributes.as_ref()
        {
            let tex_attr_loc = self.program.get_attrib_loc(name)?;
            gl::VertexAttribPointer(
                tex_attr_loc,
                *component_size,
                gl::FLOAT,
                as_gl_bool(*normalized),
                self.stride,
                byte_offset as *const c_void,
            );
            gl::EnableVertexAttribArray(tex_attr_loc);
        }

        /*
         * Element array buffer
         */
        let mut num_indices = 0;
        let element_buffer_object = self.indices.as_ref().map(|indices| {
            let mut ebo = 0;
            num_indices = indices.len();
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                try_into!(mem::size_of_val(indices.as_slice())),
                indices.as_ptr() as *const c_void,
                self.usage.into(),
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            ebo
        });

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        Ok(Model {
            program: self.program,
            vertex_array_object: vao,
            num_vertices: try_into!(num_vertices),
            num_indices: try_into!(num_indices),
            element_buffer_object,
            primitive: self.primitive,
        })
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array_object);
            self.vertex_array_object = 0;

            if let Some(ebo) = self.element_buffer_object.as_mut() {
                gl::DeleteBuffers(1, ebo);
                *ebo = 0;
            }
        }
    }
}

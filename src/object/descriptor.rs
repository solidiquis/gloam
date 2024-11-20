use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GLObjectDescriptor {
    pub(super) internal_id: usize,
    pub(super) kind: GLObjectDescriptorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLObjectDescriptorKind {
    VertexObject(usize),
    Program(usize),
    Texture(usize),
}

impl GLObjectDescriptor {
    pub fn new_vertex_object_descriptor(internal_id: usize, id: usize) -> Self {
        Self {
            internal_id,
            kind: GLObjectDescriptorKind::VertexObject(id),
        }
    }

    pub fn new_program_descriptor(internal_id: usize, id: usize) -> Self {
        Self {
            internal_id,
            kind: GLObjectDescriptorKind::Program(id),
        }
    }

    pub fn new_texture_descriptor(internal_id: usize, id: usize) -> Self {
        Self {
            internal_id,
            kind: GLObjectDescriptorKind::Texture(id),
        }
    }
}

impl Debug for GLObjectDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <GLObjectDescriptorKind as Debug>::fmt(&self.kind, f)
    }
}

use crate::{
    error::{Error, Result},
    shader::program::Program,
    texture::Texture,
    vertex::VertexObject,
};
use std::{collections::HashSet, fmt::Debug};

pub mod descriptor;
pub use descriptor::{GLObjectDescriptor, GLObjectDescriptorKind};

#[derive(Debug)]
pub struct GLObjectRegistry {
    objects: Vec<Option<GLObject>>,
    capacity_increment: usize,
    current_internal_id: usize,
    invalidated_ids: HashSet<usize>,
}

pub enum GLObject {
    VertexObject(VertexObject),
    Program(Program),
    Texture(Texture),
}

impl Default for GLObjectRegistry {
    fn default() -> Self {
        Self::new(1024, 50)
    }
}

impl GLObjectRegistry {
    pub fn new(object_capacity: usize, capacity_increment: usize) -> Self {
        Self {
            capacity_increment,
            current_internal_id: 0,
            objects: Vec::with_capacity(object_capacity),
            invalidated_ids: HashSet::new(),
        }
    }

    pub fn register_object(&mut self, obj: GLObject) -> GLObjectDescriptor {
        let desc = {
            if self.objects.len() < self.objects.capacity() {
                let obj_desc = self.make_descriptor(&obj, self.objects.len());
                self.objects.push(Some(obj));
                obj_desc
            } else {
                match self
                    .objects
                    .iter()
                    .enumerate()
                    .find(|(_, obj)| obj.is_none())
                {
                    Some((idx, _)) => {
                        let obj_desc = self.make_descriptor(&obj, idx);
                        let slot = &mut self.objects[idx];
                        *slot = Some(obj);
                        obj_desc
                    }
                    None => {
                        let obj_desc = self.make_descriptor(&obj, self.objects.len());
                        self.objects.reserve(self.capacity_increment);
                        self.objects.push(Some(obj));
                        obj_desc
                    }
                }
            }
        };
        let _ = self.get_object(desc).map(|ob| {
            log::debug!("registered new object: object_storage_id={desc:?} -> {ob:?}");
        });
        desc
    }

    pub fn remove_object(&mut self, obj_desc: GLObjectDescriptor) -> Option<GLObject> {
        if !self.descriptor_is_valid(obj_desc) {
            return None;
        }
        let idx = Self::idx_from_descriptor(obj_desc);
        let obj = &mut self.objects[idx];
        obj.take()
    }

    pub fn get_object(&self, obj_desc: GLObjectDescriptor) -> Result<&GLObject> {
        if !self.descriptor_is_valid(obj_desc) {
            return Err(Error::InvalidObjectDescriptor);
        }
        let idx = Self::idx_from_descriptor(obj_desc);
        Ok(self.objects[idx].as_ref().unwrap())
    }

    pub fn get_vertex_object(&self, obj_desc: GLObjectDescriptor) -> Result<&VertexObject> {
        let GLObject::VertexObject(vertex_object) = self.get_object(obj_desc)? else {
            return Err(Error::UnexpectedObjectDescriptorKind);
        };
        Ok(vertex_object)
    }

    pub fn get_program(&self, obj_desc: GLObjectDescriptor) -> Result<&Program> {
        let GLObject::Program(program) = self.get_object(obj_desc)? else {
            return Err(Error::UnexpectedObjectDescriptorKind);
        };
        Ok(program)
    }

    pub fn get_texture(&self, obj_desc: GLObjectDescriptor) -> Result<&Texture> {
        let GLObject::Texture(texture) = self.get_object(obj_desc)? else {
            return Err(Error::UnexpectedObjectDescriptorKind);
        };
        Ok(texture)
    }

    fn make_descriptor(&mut self, obj: &GLObject, idx: usize) -> GLObjectDescriptor {
        let desc = match obj {
            GLObject::VertexObject(_) => {
                GLObjectDescriptor::new_vertex_object_descriptor(self.current_internal_id, idx)
            }
            GLObject::Program(_) => {
                GLObjectDescriptor::new_program_descriptor(self.current_internal_id, idx)
            }
            GLObject::Texture(_) => {
                GLObjectDescriptor::new_texture_descriptor(self.current_internal_id, idx)
            }
        };
        self.current_internal_id += 1;
        desc
    }

    fn idx_from_descriptor(obj_desc: GLObjectDescriptor) -> usize {
        match obj_desc.kind {
            GLObjectDescriptorKind::VertexObject(idx) => idx,
            GLObjectDescriptorKind::Program(idx) => idx,
            GLObjectDescriptorKind::Texture(idx) => idx,
        }
    }

    fn descriptor_is_valid(&self, obj_desc: GLObjectDescriptor) -> bool {
        !self.invalidated_ids.contains(&obj_desc.internal_id)
    }
}

impl Debug for GLObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VertexObject(o) => <VertexObject as Debug>::fmt(o, f),
            Self::Texture(o) => <Texture as Debug>::fmt(o, f),
            Self::Program(o) => <Program as Debug>::fmt(o, f),
        }
    }
}

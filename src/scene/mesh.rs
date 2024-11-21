use crate::{
    context::GLContext,
    error::{Error, Result},
    object::GLObjectDescriptor,
    uniform::Uniform,
};
use std::marker::PhantomData;

/// Theoretically, if the [GLContext] does what it's supposed to, there will only
/// ever be one active mesh at a time. Additionally, even if you do not plan to set
/// your uniforms at the time you're initializing your [Mesh], you should still register
/// all the uniforms you plan to use. Make use of methods such as [Uniform::default_1d]
/// to initialize zero value uniforms.
pub struct Mesh<const N: usize, T> {
    vertex_object: GLObjectDescriptor,
    program: GLObjectDescriptor,
    uniforms: [Uniform; N],
    state: PhantomData<T>
}

/// Vertex array object is bound and program is in use.
pub struct MeshActive;

/// Vertex array object is not bound and program is not in use.
pub struct MeshInactive;

impl<const N: usize, T> Mesh<N, T> {
    /// Initialize a new inactive [Mesh].
    pub fn new(
        ctx: &GLContext,
        vo_desc: GLObjectDescriptor,
        program_desc: GLObjectDescriptor,
        uniforms: [Uniform; N],
    ) -> Result<Mesh<N, MeshInactive>> {
        // Make sure objects are in registry
        let _vo = ctx.get_vertex_object(vo_desc)?;
        let _p = ctx.get_program(program_desc)?;

        Ok(Mesh {
            uniforms,
            vertex_object: vo_desc,
            program: program_desc,
            state: PhantomData,
        })
    }
}

impl<const N: usize> Mesh<N, MeshInactive> {
    /// Bind vertex array object and use program
    pub fn try_activate(self, ctx: &mut GLContext) -> Result<Mesh<N, MeshActive>> {
        let Mesh { vertex_object, program, uniforms, .. } = self;
        ctx.try_use_program(program)?;
        ctx.try_bind_vertex_object(vertex_object)?;

        Ok(Mesh {
            vertex_object,
            program,
            uniforms,
            state: PhantomData,
        })
    }
}

impl<const N: usize> Mesh<N, MeshActive> {
    /// This will replace the uniform of the same name. If a uniform is not found
    /// with the given name, then an error will be returned. Note that this only updates
    /// the uniform CPU-side and will not send the uniforms to the GPU. Use [Mesh::try_set_uniforms]
    /// to send the uniforms to OpenGL.
    pub fn update_uniform(&mut self, update: Uniform) -> Result<()> {
        for uniform in self.uniforms.iter_mut() {
            if &uniform.name == &update.name {
                *uniform = update;
                return Ok(())
            }
        }
        Err(Error::UniformDoesNotExist(update.name))
    }

    /// Sets the uniforms in OpenGL.
    pub fn try_set_uniforms(&self, ctx: &GLContext) -> Result<()> {
        for uniform in &self.uniforms {
            ctx.try_set_uniform(uniform)?;
        }
        Ok(())
    }

    /// Sets a single uniform given by `name` in OpenGL. If that uniform does not exist
    /// in the program assocaited with this [Mesh] then an error will be returned.
    pub fn try_set_uniform(&self, ctx: &GLContext, name: &str) -> Result<()> {
        let Some(uniform) = self.uniforms.iter().find(|u| u.name.as_str() == name) else {
            return Err(Error::UniformDoesNotExist(name.to_string()));
        };
        ctx.try_set_uniform(uniform)
    }

    /// Sets the uniforms in OpenGL except for ones whose names are in `exceptions`.
    pub fn try_set_uniforms_except(&self, ctx: &GLContext, exceptions: &[&str]) -> Result<()> {
        for uniform in &self.uniforms {
            if exceptions.iter().any(|e| e == &uniform.name.as_str()) {
                continue;
            }
            ctx.try_set_uniform(uniform)?;
        }
        Ok(())
    }

    /// Will unbind the vertex array object and detach the program.
    pub fn deactivate(self, ctx: &mut GLContext) -> Result<Mesh<N, MeshInactive>> {
        let Mesh { vertex_object, program, uniforms, .. } = self;
        ctx.detach_current_program();
        ctx.detach_current_program();

        Ok(Mesh {
            vertex_object,
            program,
            uniforms,
            state: PhantomData,
        })
    }
}

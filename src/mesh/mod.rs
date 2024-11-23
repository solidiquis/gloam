use crate::{
    context::GLContext,
    error::{Error, Result},
    object::GLObjectDescriptor,
    uniform::Uniform,
};

/// An object containing all necessary information to render a complete surface.
/// Even if you do not plan to set your uniforms at the time you're initializing
/// your [Mesh], you should still register all the uniforms you plan to use.
/// Make use of methods such as [Uniform::default_1d] to initialize zero value uniforms.
#[derive(Debug)]
pub struct Mesh {
    vertex_object: GLObjectDescriptor,
    program: GLObjectDescriptor,
    uniforms: Vec<Uniform>,
}

impl Mesh {
    /// Initialize a new [Mesh]. This does not garauntee that the program passed to it is not in
    /// use and that the vertex object is not bound; that depends on what happened to those before
    /// this function was called.
    pub fn new(
        ctx: &GLContext,
        vo_desc: GLObjectDescriptor,
        program_desc: GLObjectDescriptor,
        uniforms: Vec<Uniform>,
    ) -> Result<Self> {
        // Make sure objects are in registry
        let _vo = ctx.get_vertex_object(vo_desc)?;
        let _p = ctx.get_program(program_desc)?;

        Ok(Mesh {
            uniforms,
            vertex_object: vo_desc,
            program: program_desc,
        })
    }

    /// Bind vertex array object, use program, set uniforms on program, and render.
    pub fn try_render(&self, ctx: &mut GLContext) -> Result<()> {
        ctx.try_use_program(self.program)?;
        ctx.try_bind_vertex_object(self.vertex_object)?;
        self.try_set_uniforms_on_gpu(ctx)?;
        ctx.try_render()
    }

    /// Will attempt to render [Mesh] using the current OpenGL state AS IS i.e. the currently bound
    /// vertex array object, the active program, and all the current uniforms set on that program.
    pub fn try_render_with_current_ogl_state(&self, ctx: &mut GLContext) -> Result<()> {
        ctx.try_render()
    }

    /// Will attempt to render the [Mesh] using the current OpenGL state after attempting
    /// to set uniforms in specified in the `updates` argument.
    pub fn try_set_uniforms_and_render_with_current_ogl_state(
        &mut self,
        ctx: &mut GLContext,
        updates: Vec<Uniform>,
    ) -> Result<()> {
        for uniform in updates {
            self.try_set_uniform_on_gpu(ctx, &uniform.name)?;
            self.set_uniform_on_cpu(uniform)?;
        }
        self.try_render_with_current_ogl_state(ctx)
    }

    /// Updates uniforms on the CPU, binds, vertex array object, uses program, sends all uniforms
    /// of this [Mesh] to the GPU, and renders.
    pub fn try_set_uniforms_and_render(
        &mut self,
        ctx: &mut GLContext,
        updates: Vec<Uniform>,
    ) -> Result<()> {
        for uniform in updates {
            self.set_uniform_on_cpu(uniform)?;
        }
        self.try_render(ctx)
    }

    /// This will replace the uniform of the same name. If a uniform is not found
    /// with the given name, then an error will be returned. Note that this only updates
    /// the uniform CPU-side and will not send the uniforms to the GPU. Use [Mesh::try_set_uniforms]
    /// to send the uniforms to the GPU.
    pub fn set_uniform_on_cpu(&mut self, update: Uniform) -> Result<()> {
        for uniform in self.uniforms.iter_mut() {
            if uniform.name == update.name {
                *uniform = update;
                return Ok(());
            }
        }
        Err(Error::UniformDoesNotExist(update.name))
    }

    /// Send all uniforms of this mesh to GPU.
    pub fn try_set_uniforms_on_gpu(&self, ctx: &GLContext) -> Result<()> {
        for uniform in &self.uniforms {
            ctx.try_set_uniform(uniform)?;
        }
        Ok(())
    }

    /// Sets a single uniform given by `name` in the GPU. If that uniform does not exist
    /// in the program assocaited with this [Mesh] then an error will be returned.
    pub fn try_set_uniform_on_gpu(&self, ctx: &GLContext, name: &str) -> Result<()> {
        let Some(uniform) = self.uniforms.iter().find(|u| u.name.as_str() == name) else {
            return Err(Error::UniformDoesNotExist(name.to_string()));
        };
        ctx.try_set_uniform(uniform)
    }

    pub fn deactivate(&self, ctx: &mut GLContext) {
        if ctx
            .try_get_active_program()
            .is_ok_and(|desc| desc == self.program)
        {
            ctx.detach_current_program();
        }
        if ctx
            .get_current_bound_vertex_object()
            .is_some_and(|desc| desc == self.vertex_object)
        {
            ctx.unbind_current_vertex_object();
        }
    }
}

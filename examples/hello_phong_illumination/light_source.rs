use gloam::{
    context::GLContext,
    object::GLObjectDescriptor,
    polygons::cube,
    error::Result,
    shader::{
        program::Linker,
        Shader,
        ShaderType,
    },
    vertex::{
        Primitive,
        Usage,
        VOBInit,
        VertexObjectBuilder,
    },
};
use std::path::PathBuf;
use nalgebra_glm as glm;

pub struct LightSource {
    vertices: GLObjectDescriptor,
    program: GLObjectDescriptor,
    initial_position: glm::Vec3,
    // 0.0 to 1.0
    rgb: (f32, f32, f32),

    model_matrix: glm::Mat4,
    view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4,
}

impl LightSource {
    pub fn new(ctx: &mut GLContext, initial_position: glm::Vec3) -> Result<Self> {
        let vertex_shader = {
            let vs_src = PathBuf::from("examples")
                .join("hello_phong_illumination")
                .join("light_source_vs.glsl");
            Shader::new(vs_src, ShaderType::Vertex)?
        };

        let fragment_shader = {
            let fs_src = PathBuf::from("examples")
                .join("hello_phong_illumination")
                .join("light_source_fs.glsl");
            Shader::new(fs_src, ShaderType::Fragment)?
        };

        let program = Linker::new()
            .attach_shader(vertex_shader)
            .attach_shader(fragment_shader)
            .link(ctx)?;

        let vertices = {
            VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
                .attribute("position", 3, &cube::POSITION_ATTR)?
                .build(ctx, program)?
        };

        let model_matrix = glm::translate(&glm::identity(), &initial_position);
        let view_matrix = glm::identity();
        let projection_matrix = glm::identity();

        Ok(Self {
            program,
            vertices,
            initial_position,
            model_matrix,
            view_matrix,
            projection_matrix,
            rgb: (1.0, 1.0, 1.0),
        })
    }

    pub fn scale(&mut self, factor: f32) {
        self.model_matrix = glm::scale(
            &self.model_matrix, &glm::vec3(factor, factor, factor)
        );
    }

    pub fn set_projection_matrix(&mut self, matrix: &glm::Mat4) {
        if &self.projection_matrix != matrix {
            self.projection_matrix = matrix.clone();
        }
    }

    pub fn set_view_matrix(&mut self, matrix: &glm::Mat4) {
        if &self.view_matrix != matrix {
            self.view_matrix = matrix.clone();
        }
    }

    pub fn try_render(&self, ctx: &mut GLContext) -> Result<()> {
        ctx.try_bind_vertex_object(self.vertices)?;
        ctx.try_use_program(self.program)?;


        //ctx.try_set_uniform_matrix_4fv("modelMatrix", &glm::value_ptr(&self.model_matrix), false)?;
        //ctx.try_set_uniform_matrix_4fv("viewMatrix", &glm::value_ptr(&self.view_matrix), false)?;
        //ctx.try_set_uniform_matrix_4fv("projectionMatrix", &glm::value_ptr(&self.projection_matrix), false)?;
        //ctx.try_set_uniform_1f("red", self.rgb.0)?;
        //ctx.try_set_uniform_1f("green", self.rgb.1)?;
        //ctx.try_set_uniform_1f("blue", self.rgb.2)?;
        ctx.try_render()?;

        Ok(())
    }
}

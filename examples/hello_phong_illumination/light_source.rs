use gloam::{
    context::GLContext,
    error::Result,
    mesh::Mesh,
    polygons::cube,
    shader::{program::Linker, Shader, ShaderType},
    uniform::Uniform,
    vertex::{Primitive, Usage, VOBuilder},
};
use nalgebra_glm as glm;
use std::path::PathBuf;

pub fn init_light_source(ctx: &mut GLContext, rgb: glm::Vec3) -> Result<Mesh> {
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

    let vertex_object = VOBuilder::new(Primitive::Triangles, Usage::Static)
        .attribute("position", 3, &cube::POSITION_ATTR)?
        .build(ctx, program)?;

    Mesh::new(
        ctx,
        vertex_object,
        program,
        vec![
            Uniform::new_1f("red", rgb.x.min(1.0)),
            Uniform::new_1f("green", rgb.y.min(1.0)),
            Uniform::new_1f("blue", rgb.z.min(1.0)),
            Uniform::default_mat4fv("modelMatrix", false),
            Uniform::default_mat4fv("viewMatrix", false),
            Uniform::default_mat4fv("projectionMatrix", false),
        ],
    )
}

use gloam::{
    context::GLContext,
    error::Result,
    mesh::Mesh,
    shader::{program::Linker, Shader, ShaderType},
    uniform::Uniform,
    vertex::{Primitive, Usage, VOBuilder},
};
use nalgebra_glm as glm;
use std::path::PathBuf;

pub fn init_enclosure(ctx: &mut GLContext, rgb: glm::Vec3) -> Result<Mesh> {
    let vertex_shader = {
        let vs_src = PathBuf::from("examples")
            .join("hello_phong_illumination")
            .join("enclosure_vs.glsl");
        Shader::new(vs_src, ShaderType::Vertex)?
    };

    let fragment_shader = {
        let fs_src = PathBuf::from("examples")
            .join("hello_phong_illumination")
            .join("enclosure_fs.glsl");
        Shader::new(fs_src, ShaderType::Fragment)?
    };

    let program = Linker::new()
        .attach_shader(vertex_shader)
        .attach_shader(fragment_shader)
        .link(ctx)?;

    let color_attr = rgb.as_slice().repeat(POSITION_ATTR.len() / 3);

    let vertex_object = VOBuilder::new(Primitive::Triangles, Usage::Static)
        .attribute("position", 3, &POSITION_ATTR)?
        .attribute("normal", 3, &NORMAL_ATTR)?
        .attribute("color", 3, &color_attr)?
        .build(ctx, program)?;

    Mesh::new(
        ctx,
        vertex_object,
        program,
        vec![
            Uniform::default_mat4fv("modelMatrix", false),
            Uniform::default_mat3fv("normalMatrix", false),
            Uniform::default_mat4fv("viewMatrix", false),
            Uniform::default_mat4fv("projectionMatrix", false),
            Uniform::default_3f("lightPosition"),
            Uniform::default_3f("lightColor"),
            Uniform::default_3f("cameraPosition"),
            Uniform::default_1f("ambientLightIntensity"),
            Uniform::default_1f("specularLightIntensity"),
        ],
    )
}

const POSITION_ATTR: [f32; 108] = [
    // Back face
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5,
    -0.5, // Front face
    -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5,
    // Left face
    -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5,
    0.5, // Right face
    0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,
    // Bottom face
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
    -0.5, // Top face
    -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
];

const NORMAL_ATTR: [f32; 108] = [
    // Back face
    0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
    // Front face
    0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
    // Left Face
    1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    // Right face
    -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
    // Bottom face
    0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    // Top face
    0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0,
];

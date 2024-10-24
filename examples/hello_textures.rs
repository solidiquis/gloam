use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    context::{GLContext, GLContextConfig},
    model::{primitives::Primitive, usage::Usage, ModelBuilder, VertexAttribute},
    shader::{program::Linker, Shader, ShaderType},
    texture::{TextureBuilder, TextureFilterParam, TextureWrapParam},
    Result,
};
use std::{path::PathBuf, rc::Rc};

const POSITION_ATTR: [f32; 12] = [
    0.5, 0.5, 0.0, // top right
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

const TEXTURE_ATTR: [f32; 8] = [
    1.0, 1.0, // top right
    1.0, 0.0, // bottom right
    0.0, 0.0, // bottom left
    0.0, 1.0, // bottom right
];

const COLOR_ATTR: [f32; 12] = [
    1.0, 0.0, 0.0, // top right
    0.0, 1.0, 0.0, // bottom right
    0.0, 0.0, 1.0, // bottom left
    1.0, 1.0, 0.0, // bottom right
];

const INDICES: [u32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];

fn main() -> Result<()> {
    let mut gl_context = GLContext::new(GLContextConfig {
        title: "HelloTextures",
        ..Default::default()
    })?;
    gl_context.set_key_polling(true);
    gl_context.set_framebuffer_size_polling(true);

    let vertex_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_textures_vertex.glsl");
    let fragment_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_textures_fragment.glsl");

    let vertex_shader = Shader::new(vertex_shader_src, ShaderType::Vertex)?;
    let fragment_shader = Shader::new(fragment_shader_src, ShaderType::Fragment)?;

    let program = Linker::new()
        .attach_shader(vertex_shader)
        .attach_shader(fragment_shader)
        .link()?;

    let texture_wood_src = PathBuf::new().join("examples").join("hello_textures.jpg");
    let texture_wood = TextureBuilder::new_2d_rgba8(texture_wood_src)?
        .s_wrap(TextureWrapParam::Repeat)
        .t_wrap(TextureWrapParam::Repeat)
        .min_filter(TextureFilterParam::LinearMipmapLinear)
        .mag_filter(TextureFilterParam::Linear)
        .build()
        .map(Rc::new)?;

    let texture_wood_smiley = PathBuf::new()
        .join("examples")
        .join("hello_textures_smiley.png");
    let texture_smiley = TextureBuilder::new_2d_rgba8(texture_wood_smiley)?
        .s_wrap(TextureWrapParam::Repeat)
        .t_wrap(TextureWrapParam::Repeat)
        .min_filter(TextureFilterParam::LinearMipmapLinear)
        .mag_filter(TextureFilterParam::Linear)
        .build()
        .map(Rc::new)?;

    let position_attr = VertexAttribute::new("apos", POSITION_ATTR.to_vec(), 3, false);
    let color_attr = VertexAttribute::new("acol", COLOR_ATTR.to_vec(), 3, false);
    let texture_attr = VertexAttribute::new("atex", TEXTURE_ATTR.to_vec(), 2, false);

    let surface = ModelBuilder::new(program, Usage::Static, Primitive::Triangles, position_attr)
        .and_then(|b| b.color_attributes(color_attr))
        .and_then(|b| b.texture_attributes(texture_attr))
        .and_then(|b| b.indices(INDICES.to_vec()))
        .and_then(|b| b.build())
        .map(Rc::new)?;

    gl_context.bind_model(surface.clone());
    let texture_unit_wood = gl_context.activate_texture(texture_wood.clone(), true)?;
    let texture_unit_smiley = gl_context.activate_texture(texture_smiley.clone(), true)?;

    gl_context.set_uniform_1i("texture1", texture_unit_wood)?;
    gl_context.set_uniform_1i("texture2", texture_unit_smiley)?;

    gl_context.run_event_loop(|ctx, event| {
        match event {
            None => (),
            Some(win_event) => match win_event {
                WindowEvent::Key(key, _, _, modifier) => match (modifier, key) {
                    (Modifiers::Super, Key::W) | (_, Key::Escape) => ctx.set_should_close(true),
                    _ => (),
                },
                WindowEvent::FramebufferSize(width, height) => ctx.viewport(0, 0, width, height),
                _ => (),
            },
        }
        ctx.try_render()?;
        ctx.draw();

        Ok(())
    })
}

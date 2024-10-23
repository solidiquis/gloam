use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    context::{GLContext, GLContextConfig},
    model::{primitives::Primitive, usage::Usage, ModelBuilder, VertexAttribute},
    shader::{program::Linker, Shader, ShaderType},
    texture::{TextureBuilder, TextureFilterParam, TextureWrapParam},
    Result,
};
use std::path::PathBuf;

const POSITION_ATTR: [f32; 12] = [
    0.5, 0.5, 0.0, // top right
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

const TEXTURE_ATTR: [f32; 8] = [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

const COLOR_ATTR: [f32; 12] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];

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

    let texture_src = PathBuf::new().join("examples").join("hello_textures.jpg");

    let mut texture = TextureBuilder::new_2d_rgba8(texture_src)
        .map(|b| b.s_wrap(TextureWrapParam::Repeat))
        .map(|b| b.t_wrap(TextureWrapParam::Repeat))
        .map(|b| b.min_filter(TextureFilterParam::LinearMipmapLinear))
        .map(|b| b.mag_filter(TextureFilterParam::Linear))
        .and_then(|b| b.build())?;

    let position_attr = VertexAttribute::new("apos", POSITION_ATTR.to_vec(), 3, false);
    let texture_attr = VertexAttribute::new("atex", TEXTURE_ATTR.to_vec(), 2, false);

    let mut surface =
        ModelBuilder::new(program, Usage::Static, Primitive::Triangles, position_attr)
            .and_then(|b| b.texture_attributes(texture_attr))
            .and_then(|b| b.indices(INDICES.to_vec()))
            .and_then(|b| b.build())?;

    gl_context.run_event_loop(|ctx, event| {
        let mut frame = ctx.new_frame();

        match event {
            None => (),
            Some(win_event) => match win_event {
                WindowEvent::Key(key, _, _, modifier) => match (modifier, key) {
                    (Modifiers::Super, Key::W) | (_, Key::Escape) => ctx.set_should_close(true),
                    _ => (),
                },
                WindowEvent::FramebufferSize(width, height) => frame.viewport(0, 0, width, height),
                _ => (),
            },
        }

        frame.clear_color(0.2, 0.2, 0.2, 0.0);
        frame.bind_model(&mut surface);
        frame.use_texture(&mut texture, true);
        frame.render()?;

        Ok(())
    })
}

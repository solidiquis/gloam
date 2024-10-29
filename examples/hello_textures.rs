use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    app,
    shader::{program::Linker, Shader, ShaderType},
    texture::{TextureBuilder, TextureFilterParam, TextureWrapParam},
    vertex::{Primitive, Usage, VOBInit, VertexObjectBuilder},
    Result,
};
use std::path::PathBuf;

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
    let (mut window, mut ctx) = app::init_default_opengl_3_3("HelloTextures")?;
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

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
        .link(&mut ctx)?;

    let texture_wood_src = PathBuf::new().join("examples").join("hello_textures.jpg");
    let texture_wood = TextureBuilder::new_2d_rgba8(texture_wood_src)?
        .s_wrap(TextureWrapParam::Repeat)
        .t_wrap(TextureWrapParam::Repeat)
        .min_filter(TextureFilterParam::LinearMipmapLinear)
        .mag_filter(TextureFilterParam::Linear)
        .build(&mut ctx)?;

    let texture_wood_smiley = PathBuf::new()
        .join("examples")
        .join("hello_textures_smiley.png");
    let texture_smiley = TextureBuilder::new_2d_rgba8(texture_wood_smiley)?
        .s_wrap(TextureWrapParam::Repeat)
        .t_wrap(TextureWrapParam::Repeat)
        .min_filter(TextureFilterParam::LinearMipmapLinear)
        .mag_filter(TextureFilterParam::Linear)
        .build(&mut ctx)?;

    let surface = VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
        .attribute("apos", 3, &POSITION_ATTR)?
        .attribute("acol", 3, &COLOR_ATTR)?
        .attribute("atex", 2, &TEXTURE_ATTR)?
        .indexes(&INDICES)?
        .build(&mut ctx, program)?;

    ctx.use_program(program)?;
    ctx.bind_vertex_object(surface)?;
    let texture_unit_wood = ctx.activate_texture(texture_wood, true)?;
    let texture_unit_smiley = ctx.activate_texture(texture_smiley, true)?;

    ctx.set_uniform_1i("texture1", texture_unit_wood)?;
    ctx.set_uniform_1i("texture2", texture_unit_smiley)?;

    window.run_event_loop(|win, event| {
        match event {
            None => (),
            Some(win_event) => match win_event {
                WindowEvent::Key(key, _, _, modifier) => match (modifier, key) {
                    (Modifiers::Super, Key::W) | (_, Key::Escape) => win.set_should_close(true),
                    _ => (),
                },
                WindowEvent::FramebufferSize(width, height) => ctx.viewport(0, 0, width, height),
                _ => (),
            },
        }
        ctx.try_render()?;
        win.draw();

        Ok(())
    })
}

use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    app,
    shader::{program::Linker, Shader, ShaderType},
    texture::{TextureBuilder, TextureFilterParam, TextureWrapParam},
    vertex::{Primitive, Usage, VOBInit, VertexObjectBuilder},
};
use nalgebra_glm as glm;
use std::{f32::consts::PI, path::PathBuf};

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

fn main() {
    let (mut window, mut ctx) = app::init_default_opengl_3_3("HelloTextures").unwrap();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let vertex_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_textures_vertex.glsl");
    let fragment_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_textures_fragment.glsl");

    let vertex_shader = Shader::new(vertex_shader_src, ShaderType::Vertex).unwrap();
    let fragment_shader = Shader::new(fragment_shader_src, ShaderType::Fragment).unwrap();

    let program = Linker::new()
        .attach_shader(vertex_shader)
        .attach_shader(fragment_shader)
        .link(&mut ctx)
        .unwrap();

    let texture_wood_src = PathBuf::new().join("examples").join("hello_textures.jpg");
    let texture_wood = TextureBuilder::new_2d_rgba8(texture_wood_src)
        .map(|b| b.s_wrap(TextureWrapParam::Repeat))
        .map(|b| b.t_wrap(TextureWrapParam::Repeat))
        .map(|b| b.min_filter(TextureFilterParam::LinearMipmapLinear))
        .map(|b| b.mag_filter(TextureFilterParam::Linear))
        .and_then(|b| b.build(&mut ctx))
        .unwrap();

    let texture_wood_smiley = PathBuf::new()
        .join("examples")
        .join("hello_textures_smiley.png");
    let texture_smiley = TextureBuilder::new_2d_rgba8(texture_wood_smiley)
        .map(|b| b.s_wrap(TextureWrapParam::Repeat))
        .map(|b| b.t_wrap(TextureWrapParam::Repeat))
        .map(|b| b.min_filter(TextureFilterParam::LinearMipmapLinear))
        .map(|b| b.mag_filter(TextureFilterParam::Linear))
        .and_then(|b| b.build(&mut ctx))
        .unwrap();

    let surface = VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
        .attribute("apos", 3, &POSITION_ATTR)
        .and_then(|b| b.attribute("acol", 3, &COLOR_ATTR))
        .and_then(|b| b.attribute("atex", 2, &TEXTURE_ATTR))
        .and_then(|b| b.indexes(&INDICES))
        .and_then(|b| b.build(&mut ctx, program))
        .unwrap();

    ctx.try_use_program(program).unwrap();
    ctx.try_bind_vertex_object(surface).unwrap();
    let texture_unit_wood = ctx.activate_texture(texture_wood, true).unwrap();
    let texture_unit_smiley = ctx.activate_texture(texture_smiley, true).unwrap();

    ctx.try_set_uniform_1i(program, "texture1", texture_unit_wood)
        .unwrap();
    ctx.try_set_uniform_1i(program, "texture2", texture_unit_smiley)
        .unwrap();

    let mut model_matrix = glm::identity::<f32, 4>();
    model_matrix = glm::rotate(&model_matrix, PI / 2.0, &glm::vec3(0.0, 0.0, 1.0));
    model_matrix = glm::scale(&model_matrix, &glm::vec3(0.5, 0.5, 0.5));

    ctx.try_set_uniform_matrix_4fv(program, "transform", glm::value_ptr(&model_matrix), false)
        .unwrap();

    let _ = window.run_event_loop(|win, event| {
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
        ctx.try_render().unwrap();
        win.draw();

        Ok(())
    });
}

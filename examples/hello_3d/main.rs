use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    app,
    context::ClearMask,
    error::Result,
    shader::{program::Linker, Shader, ShaderType},
    texture::{TextureBuilder, TextureFilterParam, TextureWrapParam},
    vertex::{Primitive, Usage, VOBInit, VertexObjectBuilder},
};
use nalgebra_glm as glm;
use std::{f32::consts::PI, path::PathBuf, thread};

fn main() {
    let cube_positions: [glm::TVec3<f32>; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let (mut window, mut ctx) = app::init_default_opengl_3_3("Hello3D").unwrap();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let aspect_ratio = {
        let (width, height) = window.get_size();
        width as f32 / height as f32
    };

    ctx.try_enable_depth_test(None).unwrap();

    let vertex_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_3d")
        .join("hello_3d_vertex.glsl");
    let fragment_shader_src = PathBuf::new()
        .join("examples")
        .join("hello_3d")
        .join("hello_3d_fragment.glsl");

    let vertex_shader = Shader::new(vertex_shader_src, ShaderType::Vertex).unwrap();
    let fragment_shader = Shader::new(fragment_shader_src, ShaderType::Fragment).unwrap();

    let program = Linker::new()
        .attach_shader(vertex_shader)
        .attach_shader(fragment_shader)
        .link(&mut ctx)
        .unwrap();

    let texture_metal_src = PathBuf::new()
        .join("examples")
        .join("hello_3d")
        .join("hello_3d_metal.jpg");
    let texture_sift_src = PathBuf::new()
        .join("examples")
        .join("hello_3d")
        .join("hello_3d_sift.png");

    let t = std::time::Instant::now();

    let raw_textures = thread::scope(|s| {
        let metal = s.spawn(|| TextureBuilder::new_2d_rgba8(texture_metal_src));
        let sift = s.spawn(|| TextureBuilder::new_2d_rgba8(texture_sift_src));
        [metal, sift]
            .into_iter()
            .map(|j| j.join().unwrap())
            .collect::<Vec<_>>()
    });

    println!(
        "time to load textures: {}ms",
        std::time::Instant::now().duration_since(t).as_millis()
    );

    let [raw_texture_metal, raw_texture_sift]: [Result<TextureBuilder>; 2] =
        raw_textures.try_into().unwrap();

    let texture_metal = raw_texture_metal
        .map(|b| b.s_wrap(TextureWrapParam::Repeat))
        .map(|b| b.t_wrap(TextureWrapParam::Repeat))
        .map(|b| b.min_filter(TextureFilterParam::LinearMipmapLinear))
        .map(|b| b.mag_filter(TextureFilterParam::Linear))
        .and_then(|b| b.build(&mut ctx))
        .unwrap();

    let texture_sift = raw_texture_sift
        .map(|b| b.s_wrap(TextureWrapParam::Repeat))
        .map(|b| b.t_wrap(TextureWrapParam::Repeat))
        .map(|b| b.min_filter(TextureFilterParam::LinearMipmapLinear))
        .map(|b| b.mag_filter(TextureFilterParam::Linear))
        .and_then(|b| b.build(&mut ctx))
        .unwrap();

    let surface = VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
        .attribute("pos_attr", 3, &POSITION_ATTR)
        .and_then(|b| b.attribute("tex_attr", 2, &TEXTURE_ATTR))
        .and_then(|b| b.build(&mut ctx, program))
        .unwrap();

    ctx.try_use_program(program).unwrap();
    ctx.try_bind_vertex_object(surface).unwrap();
    let texture_unit_metal = ctx.activate_texture(texture_metal, true).unwrap();
    let texture_unit_sift = ctx.activate_texture(texture_sift, true).unwrap();

    ctx.try_set_uniform_1i("metal", texture_unit_metal).unwrap();
    ctx.try_set_uniform_1i("sift", texture_unit_sift).unwrap();

    let identity_matrix = glm::identity::<f32, 4>();
    let view_matrix = glm::translate(&identity_matrix, &glm::vec3(0.0, 0.0, -3.0));
    ctx.try_set_uniform_matrix_4fv("view", glm::value_ptr(&view_matrix), false)
        .unwrap();

    let projection_matrix = glm::perspective(PI / 4.0, aspect_ratio, 0.1, 100.0);
    ctx.try_set_uniform_matrix_4fv("projection", glm::value_ptr(&projection_matrix), false)
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

        ctx.clear(&[ClearMask::DepthBuffer, ClearMask::Color(0.2, 0.3, 0.3, 1.0)]);

        let time = win.get_time() as f32;
        for (i, position) in cube_positions.iter().enumerate() {
            let angle = time * 20.0 * (i as f32 + 1.0);
            let mut model_matrix = glm::translate(&identity_matrix, position);
            model_matrix =
                glm::rotate(&model_matrix, PI * angle / 180.0, &glm::vec3(1.0, 0.3, 0.5));
            ctx.try_set_uniform_matrix_4fv("model", glm::value_ptr(&model_matrix), false)
                .unwrap();
            ctx.try_render().unwrap();
        }
        win.draw();

        Ok(())
    });
}

const POSITION_ATTR: [f32; 108] = [
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
    -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
    0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
    -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5,
    -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
];

const TEXTURE_ATTR: [f32; 72] = [
    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0,
    1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

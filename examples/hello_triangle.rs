use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    app,
    shader::{program::Linker, Shader, ShaderType},
    vertex::{Primitive, Usage, VOBInit, VertexObjectBuilder},
    Result,
};
use std::path::PathBuf;

fn main() -> Result<()> {
    let (mut window, mut ctx) = app::init_default_opengl_3_3("HelloTriangle")?;
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let vs_src = PathBuf::new()
        .join("examples")
        .join("hello_triangle_vertex.glsl");
    let vertex_shader = Shader::new(vs_src, ShaderType::Vertex)?;

    let fs_src = PathBuf::new()
        .join("examples")
        .join("hello_triangle_fragment.glsl");
    let fragment_shader = Shader::new(fs_src, ShaderType::Fragment)?;

    let program = Linker::new()
        .attach_shader(vertex_shader)
        .attach_shader(fragment_shader)
        .link(&mut ctx)?;

    let triangle = VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
        .attribute(
            "aPosition",
            3,
            &[-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
        )?
        .attribute("aColor", 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])?
        .build(&mut ctx, program)?;

    ctx.use_program(program)?;
    ctx.bind_vertex_object(triangle)?;

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
        ctx.clear_color(0.2, 0.2, 0.2, 0.0);
        ctx.try_render()?;
        win.draw();

        Ok(())
    })
}

use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    context::{GLContext, GLContextConfig},
    model::{primitives::Primitive, usage::Usage, ModelBuilder, VertexAttribute},
    shader::{program::Linker, Shader, ShaderType},
    Result,
};
use std::path::PathBuf;
use std::rc::Rc;

fn main() -> Result<()> {
    let mut gl_ctx = GLContext::new(GLContextConfig {
        title: "HelloTriangle",
        ..Default::default()
    })?;
    gl_ctx.set_key_polling(true);
    gl_ctx.set_framebuffer_size_polling(true);

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
        .link()?;

    let position_attrs = VertexAttribute::new(
        "position",
        vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
        3,
        false,
    );

    let color_attrs = VertexAttribute::new(
        "color",
        vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        3,
        false,
    );

    let triangle = ModelBuilder::new(program, Usage::Static, Primitive::Triangles, position_attrs)
        .and_then(|b| b.color_attributes(color_attrs))
        .and_then(|b| b.build())
        .map(Rc::new)?;

    gl_ctx.bind_model(triangle.clone());

    gl_ctx.run_event_loop(|ctx, event| {
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
        ctx.clear_color(0.2, 0.2, 0.2, 0.0);
        ctx.try_render()?;
        ctx.draw();

        Ok(())
    })
}

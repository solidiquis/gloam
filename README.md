# GLoam

A work-in-progress OpenGL library for Rust built on top of [gl](https://docs.rs/gl/latest/gl/) and [glfw](https://docs.rs/glfw/latest/glfw/).

## FAQ

- Why are you doing this?
    - To learn OpenGL.
- Why are you learning such an ancient and outdated technology? Have you heard of [wgpu](https://wgpu.rs/)?
    - Yes and yes.
- Should I use this in production?
    - Only if it's for shits and giggles; otherwise, no.
- Are you trying to compete with [glium](https://github.com/glium/glium)?
    - No.
- Who is the Gloam-Eyed Queen?
    - We may never know.

## Demo

```rust
use glfw::{Key, Modifiers, WindowEvent};
use gloam::{
    context::{GLContext, GLContextConfig},
    model::{primitives::Primitive, usage::Usage, ModelBuilder, VertexAttribute},
    shader::{program::Linker, Shader, ShaderType},
    Result,
};
use std::path::PathBuf;

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

    let mut triangle =
        ModelBuilder::new(program, Usage::Static, Primitive::Triangles, position_attrs)
            .and_then(|b| b.color_attributes(color_attrs))
            .and_then(|b| b.build())?;

    gl_ctx.run_event_loop(|ctx, event| {
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
        frame.bind_model(&mut triangle);
        frame.render()?;

        Ok(())
    })
}
```

<img src="https://github.com/solidiquis/gloam/blob/master/screenshots/hello_triangle.png?raw=true">

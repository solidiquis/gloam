use glfw::{Action, Key, Modifiers, WindowEvent};
use gloam::{
    app::init_default_opengl_3_3,
    error::Result,
    camera::Camera,
    context::ClearMask,
};
use nalgebra_glm as glm;
use std::f32::consts::PI;

mod light_source;
use light_source::LightSource;

fn main() -> Result<()> {
    let (mut window, mut ctx) = init_default_opengl_3_3("HelloPhongIllumination")?;
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    ctx.try_enable_depth_test(None)?;

    let camera = Camera::new(
        glm::vec3(0.0, 0.0, 5.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    let projection_matrix = glm::perspective(window.get_aspect_ratio(), PI / 4.0, 0.1, 100.0);

    let mut light_source = LightSource::new(&mut ctx, glm::vec3(0.0, 0.0, 0.0))?;
    light_source.set_projection_matrix(&projection_matrix);
    light_source.set_view_matrix(&camera.get_view_matrix());

    window.run_event_loop(|win, event| {
        ctx.clear(&[ClearMask::Color(0.0, 0.0, 0.0, 0.0), ClearMask::DepthBuffer]);
        match event {
            Some(ev) => match ev {
                WindowEvent::FramebufferSize(width, height) => {
                    ctx.viewport(0, 0, width, height);
                }
                WindowEvent::Key(key, scan_code, action, modifier) => {
                    match (key, scan_code, action, modifier) {
                        (Key::W, _, Action::Press, Modifiers::Super) => win.set_should_close(true),
                        _ => (),
                    }
                }
                _ => (),
            },
            _ => (),
        }

        light_source.try_render(&mut ctx)?;
        win.draw();
        Ok(())
    })
}

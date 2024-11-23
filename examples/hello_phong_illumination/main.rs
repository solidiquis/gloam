use glfw::{Action, Key, Modifiers, WindowEvent};
use gloam::{
    app::init_default_opengl_3_3,
    camera::{Camera, FreeCamera},
    context::ClearMask,
    error::Result,
    uniform::Uniform,
};
use nalgebra_glm as glm;
use std::f32::consts::PI;

mod enclosure;
use enclosure::init_enclosure;

mod light_source;
use light_source::init_light_source;

const COLOR_INCREMENT: f32 = 0.05;
const POSITION_INCREMENT: f32 = 0.1;

fn main() -> Result<()> {
    env_logger::init();

    let (mut window, mut ctx) = init_default_opengl_3_3("HelloPhongIllumination")?;
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    ctx.try_enable_depth_test(None)?;

    let camera = FreeCamera::new(
        glm::vec3(0.0, 0.0, 2.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );

    let projection_matrix = glm::perspective(window.get_aspect_ratio(), PI / 4.0, 0.1, 100.0);

    let mut light_translation_vector = glm::vec3(0.0, 0.0, -20.0);
    let mut light_color = glm::vec3(1.0, 1.0, 1.0);
    let ambient_light_intensity = 0.2;
    let specular_light_intensity = 0.5;

    let mut light_source = init_light_source(&mut ctx, light_color)?;
    let light_source_base_model_matrix = glm::scale(&glm::identity(), &glm::vec3(0.1, 0.1, 0.1));
    light_source.set_uniform_on_cpu(Uniform::new_mat4fv(
        "projectionMatrix",
        projection_matrix,
        false,
    ))?;
    light_source.set_uniform_on_cpu(Uniform::new_mat4fv(
        "viewMatrix",
        camera.get_view_matrix(),
        false,
    ))?;

    let enclosure_position = glm::vec3(0.0, 0.0, 0.0);
    let mut enclosure = init_enclosure(&mut ctx, glm::vec3(0.2, 0.2, 0.2))?;
    let enclosure_model_matrix = glm::translate(
        &glm::scale(&glm::identity(), &glm::vec3(3.0, 1.5, 7.0)),
        &enclosure_position,
    );
    let enclosure_normal_matrix = enclosure_model_matrix
        .fixed_view::<3, 3>(0, 0)
        .try_inverse()
        .unwrap()
        .transpose();
    enclosure.set_uniform_on_cpu(Uniform::new_mat4fv(
        "modelMatrix",
        enclosure_model_matrix,
        false,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_mat3fv(
        "normalMatrix",
        enclosure_normal_matrix,
        false,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_1f(
        "ambientLightIntensity",
        ambient_light_intensity,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_1f(
        "specularLightIntensity",
        specular_light_intensity,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_mat4fv(
        "projectionMatrix",
        projection_matrix,
        false,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_mat4fv(
        "viewMatrix",
        camera.get_view_matrix(),
        false,
    ))?;
    enclosure.set_uniform_on_cpu(Uniform::new_3f("cameraPosition", camera.position.clone()))?;

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
                        (Key::R, _, Action::Press | Action::Repeat, Modifiers::Shift) => {
                            light_color.x = (light_color.x - COLOR_INCREMENT).max(0.0);
                        }
                        (Key::R, _, Action::Press | Action::Repeat, _) => {
                            light_color.x = (light_color.x + COLOR_INCREMENT).min(1.0);
                        }
                        (Key::G, _, Action::Press | Action::Repeat, Modifiers::Shift) => {
                            light_color.y = (light_color.y - COLOR_INCREMENT).max(0.0);
                        }
                        (Key::G, _, Action::Press | Action::Repeat, _) => {
                            light_color.y = (light_color.y + COLOR_INCREMENT).min(1.0);
                        }
                        (Key::B, _, Action::Press | Action::Repeat, Modifiers::Shift) => {
                            light_color.z = (light_color.z - COLOR_INCREMENT).max(0.0);
                        }
                        (Key::B, _, Action::Press | Action::Repeat, _) => {
                            light_color.z = (light_color.z + COLOR_INCREMENT).min(1.0);
                        }
                        (Key::Up, _, Action::Press | Action::Repeat, Modifiers::Shift) => {
                            light_translation_vector.y += POSITION_INCREMENT;
                        }
                        (Key::Up, _, Action::Press | Action::Repeat, _) => {
                            light_translation_vector.z -= POSITION_INCREMENT;
                        }
                        (Key::Down, _, Action::Press | Action::Repeat, Modifiers::Shift) => {
                            light_translation_vector.y -= POSITION_INCREMENT;
                        }
                        (Key::Down, _, Action::Press | Action::Repeat, _) => {
                            light_translation_vector.z += POSITION_INCREMENT;
                        }
                        (Key::Right, _, Action::Press | Action::Repeat, _) => {
                            light_translation_vector.x += POSITION_INCREMENT;
                        }
                        (Key::Left, _, Action::Press | Action::Repeat, _) => {
                            light_translation_vector.x -= POSITION_INCREMENT;
                        }
                        _ => (),
                    }
                }
                _ => (),
            },
            _ => (),
        }

        let light_model_matrix =
            glm::translate(&light_source_base_model_matrix, &light_translation_vector);
        light_source.try_set_uniforms_and_render(
            &mut ctx,
            vec![
                Uniform::new_mat4fv("modelMatrix", light_model_matrix, false),
                Uniform::new_1f("red", light_color.x),
                Uniform::new_1f("green", light_color.y),
                Uniform::new_1f("blue", light_color.z),
            ],
        )?;
        light_source.deactivate(&mut ctx);

        let light_position = light_model_matrix.fixed_view::<3, 1>(0, 3).into_owned();
        log::info!("light source position: {light_position:?}");
        log::info!("light source color: {light_color:?}");
        enclosure.try_set_uniforms_and_render(
            &mut ctx,
            vec![
                Uniform::new_3f("lightPosition", light_position),
                Uniform::new_3f("lightColor", light_color),
            ],
        )?;
        enclosure.deactivate(&mut ctx);

        win.draw();
        Ok(())
    })
}

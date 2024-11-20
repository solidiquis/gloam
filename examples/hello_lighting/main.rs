use anyhow::Result;
use glfw::{Action, Key, Modifiers, WindowEvent};
use gloam::{
    app::init_default_opengl_3_3,
    camera::Camera,
    context::ClearMask,
    polygons,
    shader::{program::Linker, Shader, ShaderType},
    vertex::{Primitive, Usage, VOBInit, VertexObjectBuilder},
};
use nalgebra_glm as glm;
use std::{f32::consts::PI, path::PathBuf};

fn main() -> Result<()> {
    env_logger::init();

    let (mut window, mut ctx) = init_default_opengl_3_3("HelloLighting")?;
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);

    ctx.try_enable_depth_test(None)?;

    let main_path = PathBuf::from("examples").join("hello_lighting");

    let icasohedron_program = {
        let vs_src = main_path.join("icasohedron_vs.glsl");
        let fs_src = main_path.join("icasohedron_fs.glsl");

        let icasohedron_vs = Shader::new(vs_src, ShaderType::Vertex)?;
        let icasohedron_fs = Shader::new(fs_src, ShaderType::Fragment)?;
        Linker::new()
            .attach_shader(icasohedron_vs)
            .attach_shader(icasohedron_fs)
            .link(&mut ctx)?
    };

    let icasohedron = {
        let color_attrs = [0.78, 0.38, 0.19].repeat(polygons::cube::POSITION_ATTR.len() / 3);

        VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
            .attribute("position", 3, &polygons::cube::POSITION_ATTR)?
            .attribute("normal", 3, &polygons::cube::NORMAL_ATTR)?
            .attribute("color", 3, &color_attrs)?
            .build(&mut ctx, icasohedron_program)?
    };

    let light_source_program = {
        let vs_src = main_path.join("light_source_vs.glsl");
        let fs_src = main_path.join("light_source_fs.glsl");

        let light_source_vs = Shader::new(vs_src, ShaderType::Vertex)?;
        let light_source_fs = Shader::new(fs_src, ShaderType::Fragment)?;
        Linker::new()
            .attach_shader(light_source_vs)
            .attach_shader(light_source_fs)
            .link(&mut ctx)?
    };

    let light_color = [1.0, 1.0, 1.0];
    let light_position = glm::vec3(-1.0, 0.0, 1.5);
    let ambient_light_intensity = 0.1;
    let specular_light_intensity = 0.5;

    let light_source = {
        let color_attrs = light_color.repeat(polygons::cube::POSITION_ATTR.len() / 3);

        VertexObjectBuilder::<VOBInit>::new(Primitive::Triangles, Usage::Static)
            .attribute("position", 3, &polygons::cube::POSITION_ATTR)?
            .attribute("color", 3, &color_attrs)?
            .build(&mut ctx, light_source_program)?
    };

    let camera = Camera::new(
        glm::vec3(0.0, 0.0, 6.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        0.0,
        0.0,
    );
    let mut view_matrix = camera.get_view_matrix();
    let mut aspect_ratio = window.get_aspect_ratio();

    window.run_event_loop(|win, event| {
        ctx.clear(&[ClearMask::Color(0.0, 0.0, 0.0, 0.0), ClearMask::DepthBuffer]);
        match event {
            Some(ev) => match ev {
                WindowEvent::FramebufferSize(width, height) => {
                    ctx.viewport(0, 0, width, height);
                    aspect_ratio = win.get_aspect_ratio();
                    view_matrix = camera.get_view_matrix();
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

        // Light source
        ctx.try_bind_vertex_object(light_source)?;
        ctx.try_use_program(light_source_program)?;
        ctx.try_set_uniform_matrix_4fv("view", &glm::value_ptr(&view_matrix), false)?;
        ctx.try_set_uniform_matrix_4fv(
            "model",
            &glm::value_ptr(&glm::scale(
                &glm::translate(&glm::identity(), &light_position),
                &glm::vec3(0.25, 0.25, 0.25),
            )),
            false,
        )?;
        ctx.try_set_uniform_matrix_4fv(
            "projection",
            &glm::value_ptr(&glm::perspective(aspect_ratio, PI / 4.0, 0.1, 100.0)),
            false,
        )?;
        ctx.try_render()?;

        // Cube
        ctx.try_bind_vertex_object(icasohedron)?;
        ctx.try_use_program(icasohedron_program)?;
        ctx.try_set_uniform_matrix_4fv("view", &glm::value_ptr(&view_matrix), false)?;
        let mut model_matrix = glm::rotate(
            &glm::identity(),
            win.get_time() as f32,
            &glm::vec3(0.0, 1.0, 0.0),
        );
        model_matrix = glm::scale(&model_matrix, &glm::vec3(2.0, 2.0, 2.0));
        ctx.try_set_uniform_matrix_4fv("model", &glm::value_ptr(&model_matrix), false)?;
        let normal_matrix = model_matrix
            .fixed_view::<3, 3>(0, 0)
            .try_inverse()
            .unwrap()
            .transpose();
        ctx.try_set_uniform_matrix_3fv("normalMatrix", &glm::value_ptr(&normal_matrix), false)?;
        ctx.try_set_uniform_matrix_4fv(
            "projection",
            &glm::value_ptr(&glm::perspective(aspect_ratio, PI / 4.0, 0.1, 100.0)),
            false,
        )?;
        ctx.try_set_uniform_3f(
            "cameraPosition",
            camera.position.x,
            camera.position.y,
            camera.position.z,
        )?;
        ctx.try_set_uniform_3f(
            "lightPosition",
            light_position.x,
            light_position.y,
            light_position.z,
        )?;
        ctx.try_set_uniform_3f("lightColor", light_color[0], light_color[1], light_color[2])?;
        ctx.try_set_uniform_1f("ambientLightIntensity", ambient_light_intensity)?;
        ctx.try_set_uniform_1f("specularLightIntensity", specular_light_intensity)?;
        ctx.try_render()?;

        win.draw();
        Ok(())
    })?;

    Ok(())
}

use std::{
    cell::RefCell,
    f32::consts::PI,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use glfw::Key;
use magiebleue::{
    Application, WindowContext,
    functions::{gen_patches, get_error, set_clear_color},
    gl_objects::{Primitive, VertexArray},
    shaders::ShaderProgram,
    textures::{self, TexScaleOp, TexWrapBehaviour, Texture, TextureType},
    wavefront_parser::Vertex,
};
use noise::permutationtable::PermutationTable;
use ultraviolet::{IVec2, Vec3, Vec4};

fn main() {
    const SPEED: f32 = 20.0;
    const MOUSE_SENS: f32 = 0.002;

    let pitch = Rc::new(RefCell::new(0.0_f32));
    let yaw = Rc::new(RefCell::new(0.0_f32));

    let mut last_time = 0.0;

    let mut camera_right = Default::default();
    let camera_up;
    let mut camera_front = Default::default();
    let mut camera_pos;
    let mut model = Default::default();
    let mut view = Default::default();
    let proj;

    let heightmap_shader;
    let base_shader;

    let mut heightmap_vao;
    let mut plane_vao;

    let heightmap_texture;

    let mut application = Application::start(WindowContext {
        size: IVec2::new(1920, 1080),
        window_title: "Magiebleue - Heightmap".to_owned(),
        window_mode: glfw::WindowMode::Windowed,
    });
    heightmap_shader = ShaderProgram::from_filepath(
        "shaders/heightmap.vert",
        Some("shaders/heightmap.tesc"),
        Some("shaders/heightmap.tese"),
        None,
        "shaders/heightmap.frag",
    )
    .unwrap();

    base_shader =
        ShaderProgram::from_filepath("shaders/base.vert", None, None, None, "shaders/base.frag")
            .unwrap();

    set_clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));

    let mut vertices: Option<Vec<Vertex>> = Some(Vec::new());
    gen_patches(
        vertices.as_mut().unwrap(),
        64,
        256.0,
        Vec3 {
            x: -128.0,
            y: 0.0,
            z: -128.0,
        },
    );

    let plane_data: [Vertex; 4] = [
        [-5.0, 0.0, -5.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        [5.0, 0.0, -5.0, 1.0, 0.0, 0.0, 1.0, 0.0],
        [5.0, 0.0, 5.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [-5.0, 0.0, 5.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    ];

    heightmap_vao = VertexArray::new().expect("VAO should create");
    heightmap_vao.attach_vertex(vertices.unwrap());

    plane_vao = VertexArray::new().expect("VAO should create");
    plane_vao.attach_vertex(plane_data.to_vec());

    unsafe {
        gl::PatchParameteri(gl::PATCH_VERTICES, 4);
        gl::Enable(gl::DEPTH_TEST);
    }

    let permtable = PermutationTable::new(
        // seed based on time
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("should not be in the future")
            .subsec_nanos(),
    );

    heightmap_texture = Texture::new().expect("texture should create");
    heightmap_texture.bind(TextureType::Tex2d);
    Texture::fill_noise(128, &permtable);
    Texture::gen_mipmap(TextureType::Tex2d);
    Texture::set_dual_wrap_behaviour(TextureType::Tex2d, TexWrapBehaviour::ClampToBorder);
    Texture::set_border_colour(TextureType::Tex2d, Vec4::zero());
    Texture::set_dual_scale_behaviour(TextureType::Tex2d, TexScaleOp::Linear);

    let yaw_cb = Rc::clone(&yaw);
    let pitch_cb = Rc::clone(&pitch);

    camera_pos = Vec3::new(-3.0, 1.0, 5.0);
    camera_up = Vec3::unit_y();

    proj = ultraviolet::projection::perspective_infinite_z_gl(PI / 0.6, 1920.0 / 1080.0, 0.01);
    
    application.window.set_cursor_pos_callback(move |w, x, y| {
        let dx: f32 = x as f32 - 1920.0_f32 / 2.0_f32;
        let dy: f32 = y as f32 - 1080.0_f32 / 2.0_f32;

        let mut yaw = yaw_cb.borrow_mut();
        let mut pitch = pitch_cb.borrow_mut();

        *yaw += dx * MOUSE_SENS;
        *pitch -= dy * MOUSE_SENS;
        *yaw %= 2.0 * PI;
        *pitch = pitch.clamp(-PI / 2.1, PI / 2.1);

        println!("{} yaw, {} pitch", *yaw / PI * 180.0, *pitch / PI * 180.0);

        w.set_cursor_pos(1920.0 / 2.0, 1080.0 / 2.0);
    });
    
    while !application.window.should_close() {
        application.update(|window, keys_pressed, time| {
            let delta_time = time - last_time;
            last_time = time;

            let (yaw_val, pitch_val) = {
                let yaw = *yaw.borrow();
                let pitch = *pitch.borrow();
                (yaw, pitch)
            };

            camera_front = Vec3::new(
                yaw_val.cos() * pitch_val.cos(),
                pitch_val.sin(),
                yaw_val.sin() * pitch_val.cos(),
            );
            camera_front.normalize();

            camera_right = camera_front.cross(camera_up).normalized();

            model = ultraviolet::Mat4::identity();

            view = ultraviolet::Mat4::look_at(camera_pos, camera_pos + camera_front, -camera_up);

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                base_shader.use_program();
                base_shader.set_matrix_uniforms(&model, &view, &proj);

                plane_vao.draw(Primitive::TriangleFan);

                textures::set_texture_slot(0);
                heightmap_texture.bind(TextureType::Tex2d);

                heightmap_shader.use_program();
                heightmap_shader.set_matrix_uniforms(&model, &view, &proj);

                heightmap_vao.draw(Primitive::Patches);
                get_error(Some("end of render"));
            }

            for item in keys_pressed.iter() {
                match *item {
                    Key::W => {
                        camera_pos += camera_front * delta_time as f32 * SPEED;
                        if keys_pressed.contains(&Key::LeftShift)
                            || keys_pressed.contains(&Key::RightShift)
                        {
                            camera_pos.y -= (camera_front * delta_time as f32 * SPEED).y;
                        }
                    }
                    Key::S => {
                        camera_pos -= camera_front * delta_time as f32 * SPEED;
                        if keys_pressed.contains(&Key::LeftShift)
                            || keys_pressed.contains(&Key::RightShift)
                        {
                            camera_pos.y += (camera_front * delta_time as f32 * SPEED).y;
                        }
                    }
                    Key::A => {
                        camera_pos -= camera_right * delta_time as f32 * SPEED;
                    }
                    Key::D => {
                        camera_pos += camera_right * delta_time as f32 * SPEED;
                    }
                    Key::Escape => {
                        window.set_should_close(true);
                    }
                    _ => {}
                }
            }
        });
    }
}

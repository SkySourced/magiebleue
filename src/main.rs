extern crate gl;

pub mod functions;
pub mod gl_objects;
pub mod shaders;
pub mod textures;
pub mod wavefront_parser;

use glfw::WindowEvent;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::{
    f32::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use glfw::{Action, Context, CursorMode, Key};
use noise::permutationtable::PermutationTable;
use ultraviolet::{Vec3, Vec4};

use crate::functions::get_error;
use crate::gl_objects::Primitive;
use crate::{
    functions::set_clear_color,
    gl_objects::VertexArray,
    shaders::ShaderProgram,
    textures::{TexScaleOp, TexWrapBehaviour, Texture, TextureType},
    wavefront_parser::Vertex,
};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(1920, 1080, "Magiebleue", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.set_cursor_mode(CursorMode::Disabled);
    window.set_cursor_pos(1920.0 / 2.0, 1080.0 / 2.0);
    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

    let shader_program_geom = ShaderProgram::from_vert_geom_frag_file(
        "shaders/heightmap.vert",
        "shaders/heightmap.geom",
        "shaders/heightmap.frag",
    )
    .unwrap();

    let shader_program =
        ShaderProgram::from_vert_frag_file("shaders/base.vert", "shaders/base.frag").unwrap();

    set_clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));

    let vertices: Option<Vec<Vertex>> = Some(
        [
            [0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [2.0, 2.0, 0.0, 0.25, 0.0, 0.0, 1.0, 0.0],
            [2.0, 2.0, 0.0, 0.25, 0.0, 0.0, 1.0, 0.0],
            [4.0, 2.0, 0.0, 0.5, 0.0, 0.0, 1.0, 0.0],
            [4.0, 2.0, 0.0, 0.5, 0.0, 0.0, 1.0, 0.0],
            [6.0, 2.0, 0.0, 0.75, 0.0, 0.0, 1.0, 0.0],
            [6.0, 2.0, 0.0, 0.75, 0.0, 0.0, 1.0, 0.0],
            [8.0, 2.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 2.0, 2.0, 0.0, 0.25, 0.0, 1.0, 0.0],
            [2.0, 2.0, 2.0, 0.25, 0.25, 0.0, 1.0, 0.0],
            [2.0, 2.0, 2.0, 0.25, 0.25, 0.0, 1.0, 0.0],
            [4.0, 2.0, 2.0, 0.5, 0.25, 0.0, 1.0, 0.0],
            [4.0, 2.0, 2.0, 0.5, 0.25, 0.0, 1.0, 0.0],
            [6.0, 2.0, 2.0, 0.75, 0.25, 0.0, 1.0, 0.0],
            [6.0, 2.0, 2.0, 0.75, 0.25, 0.0, 1.0, 0.0],
            [8.0, 2.0, 2.0, 1.0, 0.25, 0.0, 1.0, 0.0],
            [0.0, 2.0, 4.0, 0.0, 0.5, 0.0, 1.0, 0.0],
            [2.0, 2.0, 4.0, 0.25, 0.5, 0.0, 1.0, 0.0],
            [2.0, 2.0, 4.0, 0.25, 0.5, 0.0, 1.0, 0.0],
            [4.0, 2.0, 4.0, 0.5, 0.5, 0.0, 1.0, 0.0],
            [4.0, 2.0, 4.0, 0.5, 0.5, 0.0, 1.0, 0.0],
            [6.0, 2.0, 4.0, 0.75, 0.5, 0.0, 1.0, 0.0],
            [6.0, 2.0, 4.0, 0.75, 0.5, 0.0, 1.0, 0.0],
            [8.0, 2.0, 4.0, 1.0, 0.5, 0.0, 1.0, 0.0],
            [0.0, 2.0, 6.0, 0.0, 0.75, 0.0, 1.0, 0.0],
            [2.0, 2.0, 6.0, 0.25, 0.75, 0.0, 1.0, 0.0],
            [2.0, 2.0, 6.0, 0.25, 0.75, 0.0, 1.0, 0.0],
            [4.0, 2.0, 6.0, 0.5, 0.75, 0.0, 1.0, 0.0],
            [4.0, 2.0, 6.0, 0.5, 0.75, 0.0, 1.0, 0.0],
            [6.0, 2.0, 6.0, 0.75, 0.75, 0.0, 1.0, 0.0],
            [6.0, 2.0, 6.0, 0.75, 0.75, 0.0, 1.0, 0.0],
            [8.0, 2.0, 6.0, 1.0, 0.75, 0.0, 1.0, 0.0],
            [0.0, 2.0, 8.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            [2.0, 2.0, 8.0, 0.25, 1.0, 0.0, 1.0, 0.0],
            [2.0, 2.0, 8.0, 0.25, 1.0, 0.0, 1.0, 0.0],
            [4.0, 2.0, 8.0, 0.5, 1.0, 0.0, 1.0, 0.0],
            [4.0, 2.0, 8.0, 0.5, 1.0, 0.0, 1.0, 0.0],
            [6.0, 2.0, 8.0, 0.75, 1.0, 0.0, 1.0, 0.0],
            [6.0, 2.0, 8.0, 0.75, 1.0, 0.0, 1.0, 0.0],
            [8.0, 2.0, 8.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        ]
        .to_vec(),
    );

    let plane_data: [Vertex; 4] = [
        [-5.0, 0.0, -5.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        [5.0, 0.0, -5.0, 1.0, 0.0, 0.0, 1.0, 0.0],
        [5.0, 0.0, 5.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [-5.0, 0.0, 5.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    ];

    let mut heightmap_vao = VertexArray::new().expect("VAO should create");
    heightmap_vao.attach_vertex(vertices.unwrap());

    let mut plane_vao = VertexArray::new().expect("VAO should create");
    plane_vao.attach_vertex(plane_data.to_vec());

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let permtable = PermutationTable::new(
        // seed based on time
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("should not be in the future")
            .subsec_nanos(),
    );

    let tex = Texture::new().expect("texture should create");
    tex.bind(TextureType::Tex2d);
    Texture::fill_noise(32, &permtable);
    Texture::gen_mipmap(TextureType::Tex2d);
    Texture::set_dual_wrap_behaviour(TextureType::Tex2d, TexWrapBehaviour::ClampToBorder);
    Texture::set_border_colour(TextureType::Tex2d, Vec4::zero());
    Texture::set_dual_scale_behaviour(TextureType::Tex2d, TexScaleOp::Linear);

    let mut last_time = 0.0;
    let mut delta_time;
    const SPEED: f32 = 20.0;
    const MOUSE_SENS: f32 = 0.002;

    let pitch = Rc::new(RefCell::new(0.0));
    let yaw = Rc::new(RefCell::new(0.0));
    let yaw_cb = Rc::clone(&yaw);
    let pitch_cb = Rc::clone(&pitch);

    let mut model: ultraviolet::Mat4;
    let mut view: ultraviolet::Mat4;

    let mut camera_pos = Vec3::new(-3.0, 1.0, 5.0);
    let camera_up = Vec3::unit_y();
    let mut camera_right;
    let mut camera_front;

    let proj = ultraviolet::projection::perspective_infinite_z_gl(PI / 0.6, 1920.0 / 1080.0, 0.01);

    window.set_cursor_pos_callback(move |w, x, y| {
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

    let mut keys_pressed = HashSet::<glfw::Key>::new();

    while !window.should_close() {
        delta_time = glfw.get_time() - last_time;
        last_time = glfw.get_time();

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

            shader_program.use_program();
            shader_program.set_matrix_uniforms(&model, &view, &proj);

            plane_vao.draw(Primitive::TriangleFan);
            // heightmap_vao.draw(Primitive::Lines);

            shader_program_geom.use_program();
            shader_program_geom.set_matrix_uniforms(&model, &view, &proj);

            heightmap_vao.draw(Primitive::Lines);
            get_error(None);
        }

        // swap buffer
        window.swap_buffers();

        // events
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            if let WindowEvent::Key(key, _, action, _) = event {
                match action {
                    Action::Press => {
                        keys_pressed.insert(key);
                    }
                    Action::Release => {
                        keys_pressed.remove(&key);
                    }
                    _ => {}
                }
            }
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
    }
}

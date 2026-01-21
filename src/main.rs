extern crate gl;

pub mod functions;
pub mod gl_objects;
pub mod shaders;
pub mod textures;

use std::{
    f32::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use gl::*;
use glfw::{Action, Context, Key};
use noise::permutationtable::PermutationTable;
use ultraviolet::{Bivec3, Vec3, Vec4};

use crate::{
    functions::set_clear_color,
    gl_objects::{Buffer, BufferType, VertexArray, buffer_data},
    shaders::ShaderProgram,
    textures::{TexScaleOp, TexWrapBehaviour, Texture, TextureType},
};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(1920, 1080, "wow a window", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

    let shader_program =
        ShaderProgram::from_vert_frag_file("shaders/base.vert", "shaders/base.frag").unwrap();
    shader_program.use_program();

    set_clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));

    let vao = VertexArray::new().expect("VAO should create");
    vao.bind();

    let vbo = Buffer::new().expect("VBO should create");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        gl::STATIC_DRAW,
    );

    let ebo = Buffer::new().expect("EBO should create");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        gl::STATIC_DRAW,
    );

    type Vertex = [f32; 5];
    type TriIndices = [u32; 3];

    const VERTICES: [Vertex; 36] = [
        [-0.5, -0.5, -0.5, 0.0, 0.0],
        [0.5, -0.5, -0.5, 1.0, 0.0],
        [0.5, 0.5, -0.5, 1.0, 1.0],
        [0.5, 0.5, -0.5, 1.0, 1.0],
        [-0.5, 0.5, -0.5, 0.0, 1.0],
        [-0.5, -0.5, -0.5, 0.0, 0.0],
        [-0.5, -0.5, 0.5, 0.0, 0.0],
        [0.5, -0.5, 0.5, 1.0, 0.0],
        [0.5, 0.5, 0.5, 1.0, 1.0],
        [0.5, 0.5, 0.5, 1.0, 1.0],
        [-0.5, 0.5, 0.5, 0.0, 1.0],
        [-0.5, -0.5, 0.5, 0.0, 0.0],
        [-0.5, 0.5, 0.5, 1.0, 0.0],
        [-0.5, 0.5, -0.5, 1.0, 1.0],
        [-0.5, -0.5, -0.5, 0.0, 1.0],
        [-0.5, -0.5, -0.5, 0.0, 1.0],
        [-0.5, -0.5, 0.5, 0.0, 0.0],
        [-0.5, 0.5, 0.5, 1.0, 0.0],
        [0.5, 0.5, 0.5, 1.0, 0.0],
        [0.5, 0.5, -0.5, 1.0, 1.0],
        [0.5, -0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, 0.5, 0.0, 0.0],
        [0.5, 0.5, 0.5, 1.0, 0.0],
        [-0.5, -0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, -0.5, 1.0, 1.0],
        [0.5, -0.5, 0.5, 1.0, 0.0],
        [0.5, -0.5, 0.5, 1.0, 0.0],
        [-0.5, -0.5, 0.5, 0.0, 0.0],
        [-0.5, -0.5, -0.5, 0.0, 1.0],
        [-0.5, 0.5, -0.5, 0.0, 1.0],
        [0.5, 0.5, -0.5, 1.0, 1.0],
        [0.5, 0.5, 0.5, 1.0, 0.0],
        [0.5, 0.5, 0.5, 1.0, 0.0],
        [-0.5, 0.5, 0.5, 0.0, 0.0],
        [-0.5, 0.5, -0.5, 0.0, 1.0],
    ];
    const INDICES: [TriIndices; 2] = [[0, 1, 2], [1, 2, 3]];

    unsafe {
        #[allow(clippy::zero_ptr)]
        gl::VertexAttribPointer(
            0,
            3,
            FLOAT,
            FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            FLOAT,
            FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            (3 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
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

    let mut model;
    let view = ultraviolet::Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0));
    let proj = ultraviolet::projection::perspective_infinite_z_gl(PI * 1.2, 1920.0 / 1080.0, 0.01);

    while !window.should_close() {
        unsafe {
            gl::Clear(COLOR_BUFFER_BIT);
            gl::DrawArrays(TRIANGLES, 0, 36);
            // gl::DrawElements(TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            model = ultraviolet::Mat4::from_angle_plane(
                PI / 4.0 + (glfw.get_time() as f32),
                Bivec3::new(1.0, 1.0, 1.0),
            );

            gl::UniformMatrix4fv(
                shader_program.get_uniform_location("model"),
                1,
                FALSE,
                model.as_array().as_ptr(),
            );

            gl::UniformMatrix4fv(
                shader_program.get_uniform_location("view"),
                1,
                FALSE,
                view.as_array().as_ptr(),
            );

            gl::UniformMatrix4fv(
                shader_program.get_uniform_location("proj"),
                1,
                FALSE,
                proj.as_array().as_ptr(),
            );
        }

        // swap buffer
        window.swap_buffers();

        // events
        glfw.poll_events();
        for (id, event) in glfw::flush_messages(&events) {
            #[allow(clippy::single_match)]
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}

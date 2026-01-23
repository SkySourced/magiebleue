extern crate gl;

pub mod functions;
pub mod gl_objects;
pub mod shaders;
pub mod textures;
pub mod wavefront_parser;

use glfw::{Action, Context, CursorMode, WindowEvent};
use std::collections::HashSet;

pub struct WindowContext {
    pub size: ultraviolet::IVec2,
    pub window_title: String,
    pub window_mode: glfw::WindowMode<'static>,
}

pub struct Application {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    keys_pressed: HashSet<glfw::Key>,
}

impl Application {
    /// Starts the Magiebleue application. Takes context from `context`.
    pub fn start(ctx: WindowContext) -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(
                ctx.size.x.try_into().expect("window size must be positive"),
                ctx.size.y.try_into().expect("window size must be positive"),
                &ctx.window_title,
                ctx.window_mode,
            )
            .expect("Failed to create window");

        window.set_cursor_mode(CursorMode::Disabled);
        window.set_cursor_pos(1920.0 / 2.0, 1080.0 / 2.0);
        window.make_current();
        window.set_key_polling(true);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        let keys_pressed = HashSet::<glfw::Key>::new();

        Application {
            glfw,
            window,
            events,
            keys_pressed,
        }
    }

    /// Runs the `loop` closure then swaps GL buffers and updates `Application::keysPressed`
    pub fn update<C>(&mut self, mut r#loop: C)
    where
        C: FnMut(&mut glfw::PWindow, &HashSet<glfw::Key>, f64),
    {
        r#loop(&mut self.window, &self.keys_pressed, self.glfw.get_time());

        // swap buffer
        self.window.swap_buffers();

        // events
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            if let WindowEvent::Key(key, _, action, _) = event {
                match action {
                    Action::Press => {
                        self.keys_pressed.insert(key);
                    }
                    Action::Release => {
                        self.keys_pressed.remove(&key);
                    }
                    _ => {}
                }
            }
        }
    }
}

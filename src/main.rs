extern crate glfw;

use gl::types::*;
use opengl_yt_tutorial_rs::utils;
use self::glfw::{Context, Key, Action};

extern crate gl;

use std::mem;
use std::ptr;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) =
        glfw.create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "Learn OpenGL",
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {

        // Program
        let shaders = utils::parse_shader("./src/shader/Basic.shader");
        let vs = utils::compile_shader(&shaders[0]);
        let fs = utils::compile_shader(&shaders[1]);
        let program = utils::link_program(vs, fs); 

        // Buffer
        let positions: [GLfloat; 6] = [
            -0.5, -0.5,
            0.5, -0.5,
            0.0, 0.5
        ];
        let mut va: u32 = 0;
        gl::GenVertexArrays(1, &mut va);
        gl::BindVertexArray(va);
        let mut buffer: u32 = 0;
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(gl::ARRAY_BUFFER, (6 * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&positions[0]), gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (2 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        

        // Set Program
        gl::UseProgram(program);

        // Draw
        // gl::ClearColor(1.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        window.swap_buffers();

        while !window.should_close() {
            for (time, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, (time, event));
            }
            // window.swap_buffers();
            glfw.poll_events();
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        gl::DeleteProgram(program);
    }
}

fn handle_window_event(window: &mut glfw::Window, (_, event): (f64, glfw::WindowEvent)) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) };
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
use gl::types::*;
use opengl_yt_tutorial_rs::utils;
use glfw::{Context, Key, Action};

extern crate gl;

use std::ffi::CString;
use std::mem;
use std::ptr;

#[macro_use] extern crate opengl_yt_tutorial_rs;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
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
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
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
        let positions: [GLfloat; 8] = [
            -0.5, -0.5,
            0.5, -0.5,
            0.5, 0.5,
            -0.5, 0.5
        ];
        let indices: [GLuint; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
        let mut va: u32 = 0;
        gl::GenVertexArrays(1, &mut va);
        gl::BindVertexArray(va);
        let mut buffer: u32 = 0;
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(gl::ARRAY_BUFFER, (8 * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&positions[0]), gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (2 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        let mut ibo: u32 = 0;
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (6 * mem::size_of::<GLuint>()) as GLsizeiptr, mem::transmute(&indices[0]), gl::STATIC_DRAW);
        

        // Set Program
        gl::UseProgram(program);

        // Set Uniform
        let uniform_name = CString::new("u_Color").unwrap();
        let location = gl::GetUniformLocation(program, uniform_name.as_ptr());
        gl_check_error!();

        let mut r: f32 = 0.5;
        let rate: f32 = 0.05;
        // Draw
        while !window.should_close() {
            for (time, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, (time, event));
            }
            let time = glfw.get_time() as f32;
            let red = time.sin() / 2.0 + 0.5;
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Uniform4f(location, red, 0.3, 0.8, 1.0);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl_check_error!();

            if r > 1.0 {
                r -= rate
            } else if r < 0.0 {
                r += rate
            }

            window.swap_buffers();
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
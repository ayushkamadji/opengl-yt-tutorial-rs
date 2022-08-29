extern crate glfw;

use gl::types::*;

use self::glfw::{Context, Key, Action};

extern crate gl;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


// Shader sources
static VS_SRC: &'static str = "
#version 330
in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &'static str = "
#version 330
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

fn compile_shader(type_: GLenum, source: &str) -> GLuint {
    let shader: u32;
    unsafe {
        shader = gl::CreateShader(type_);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len(len as usize); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            let bufstr = str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8");
            panic!(
                "{}",
                bufstr
            );
        }
    }
    shader
}

fn link_program(vs: u32, fs: u32) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len(len as usize); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}

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
        let vs = compile_shader(gl::VERTEX_SHADER, VS_SRC);
        let fs = compile_shader(gl::FRAGMENT_SHADER, FS_SRC);
        let program = link_program(vs, fs); 

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
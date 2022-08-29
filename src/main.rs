extern crate glfw;

use std::ffi::CStr;

use self::glfw::{Context, Key, Action};

extern crate gl;


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
            "OpenGL",
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        let version = gl::GetString(gl::VERSION);
        let version_c_str = CStr::from_ptr(version as *const i8);
        let version_str = version_c_str.to_str().unwrap();
        print!("OpenGL version: {}", version_str);
    }

    while !window.should_close() {
        for (time, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, (time, event));
        }
        window.swap_buffers();
        glfw.poll_events();
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
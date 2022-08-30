use std::ffi::CString;
use std::fs::read_to_string;
use gl::types::*;
use std::ptr;
use std::str;

#[derive(Debug, Clone)]
pub enum ShaderType {
    VertexShader,
    FragmentShader
}
impl ShaderType {
    pub fn as_gl_enum(&self) -> GLenum {
        match self {
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}


#[derive(Debug, Clone)]
pub struct ShaderSource {
    type_: ShaderType,
    source: Box<String>
}

impl ShaderSource {
  pub fn new(type_: ShaderType, source: &str) -> ShaderSource {
    ShaderSource {
      type_,
      source: Box::new(String::from(source))
    }
  }

  pub fn append_line(&mut self, line: &str) {
    let line = format!("{}\n", line);
    self.source.push_str(line.as_str());
  }
}



pub fn compile_shader(shader_source: &ShaderSource) -> GLuint {
    let type_ = shader_source.type_.as_gl_enum();
    let source = shader_source.source.as_str();
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

pub fn link_program(vs: u32, fs: u32) -> GLuint {
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

pub fn parse_shader(path: &str) -> Vec<ShaderSource>  {
  let src = read_to_string(path).expect("Failed to read shader");

  let mut shaders: Vec<ShaderSource> = Vec::new();

  let lines = src.lines();
  for line in lines {
    if let Some(_) = line.find("#shader") {
      if let Some(_) = line.find("vertex") {
        shaders.push(ShaderSource::new(ShaderType::VertexShader, ""));
      } else if let Some(_) = line.find("fragment") {
        shaders.push(ShaderSource::new(ShaderType::FragmentShader, ""));
      }
    } else {
      if let Some(shader) = shaders.last_mut() {
        shader.append_line(line);
      }
    };
  }
  shaders
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_shader_should_succeed() {
    let result = parse_shader("./src/shader/Basic.shader");
    assert_eq!(result[0].source.as_str(), "#version 330 core\n\nlayout(location = 0) in vec4 position;\n\nvoid main()\n{\n    gl_Position = position;\n};\n\n");
    assert_eq!(result[1].source.as_str(), "#version 330 core\n\nlayout(location = 0) out vec4 color;\n\nvoid main()\n{\n    color = vec4(1.0, 0.0, 0.0, 1.0);\n};\n");
  }
}
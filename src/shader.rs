use glad_gles2::gl;
use std::ffi::{CStr, CString};
use std::ptr::null;

pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    fn gl_type(&self) -> gl::GLenum {
        match *self {
            ShaderType::Vertex => gl::GL_VERTEX_SHADER,
            ShaderType::Fragment => gl::GL_FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    shader: Vec<gl::GLuint>,
    program: gl::GLuint,
    ready: bool,
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.ready {
            unsafe {
                gl::DeleteProgram(self.program);
            }
        }
        for i in &self.shader {
            unsafe {
                gl::DeleteShader(*i);
            }
        }
    }
}

impl Shader {
    pub fn new() -> Shader {
        Shader {
            shader: Vec::new(),
            program: 0,
            ready: false,
        }
    }

    pub fn activate(&self) {
        if self.ready {
            unsafe {
                gl::UseProgram(self.program);
            }
        }
    }

    pub fn attach(&mut self, source: &str, kind: ShaderType) {
        let shdr;
        let source = CString::new(source).unwrap();
        unsafe {
            shdr = gl::CreateShader(kind.gl_type());
            gl::ShaderSource(shdr, 1, &source.as_ptr(), null());
            gl::CompileShader(shdr);
            let mut status: gl::GLint = 0;
            gl::GetShaderiv(shdr, gl::GL_COMPILE_STATUS, &mut status);
            if status == gl::GL_FALSE as gl::GLint {
                gl::GetShaderiv(shdr, gl::GL_INFO_LOG_LENGTH, &mut status);
                let size = status as usize;
                let mut buffer = vec![0_u8; size];
                gl::GetShaderInfoLog(shdr, status, &mut status, buffer.as_mut_ptr() as *mut i8);
                let cstr = CStr::from_bytes_with_nul(&buffer)
                    .unwrap()
                    .to_string_lossy();
                error!("Cannot compile shader: {}", cstr);
                panic!();
            }
        }
        self.shader.push(shdr);
    }

    pub fn compile(&mut self) {
        if self.ready {
            return;
        }
        let program;
        unsafe {
            program = gl::CreateProgram();
            for i in &self.shader {
                gl::AttachShader(program, *i);
            }
            gl::LinkProgram(program);
            let mut status: gl::GLint = 0;
            gl::GetProgramiv(program, gl::GL_LINK_STATUS, &mut status);
            if status == gl::GL_FALSE as gl::GLint {
                gl::GetProgramiv(program, gl::GL_INFO_LOG_LENGTH, &mut status);
                let size = status as usize;
                let mut buffer = vec![0_u8; size];
                gl::GetProgramInfoLog(program, status, &mut status, buffer.as_mut_ptr() as *mut i8);
                let cstr = CStr::from_bytes_with_nul(&buffer)
                    .unwrap()
                    .to_string_lossy();
                error!("Cannot link shader: {}", cstr);
                panic!();
            }
        }
        self.program = program;
        self.ready = true;
    }
}

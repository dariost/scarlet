use crate::{have_debug_gl, have_gl};
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
        if have_gl() {
            unsafe {
                if self.ready {
                    gl::DeleteProgram(self.program);
                }
                for i in &self.shader {
                    gl::DeleteShader(*i);
                }
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

    pub fn uniform1ui(&mut self, name: &str, value: u32) {
        let name = CString::new(name).expect("Cannot convert to CString");
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform1ui(location, value);
        }
    }

    pub fn uniform1f(&mut self, name: &str, value: f32) {
        let name = CString::new(name).expect("Cannot convert to CString");
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn uniform4f(&mut self, name: &str, value: [f32; 4]) {
        let name = CString::new(name).expect("Cannot convert to CString");
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform4fv(location, 1, value.as_ptr());
        }
    }

    pub fn uniform3f(&mut self, name: &str, value: [f32; 3]) {
        let name = CString::new(name).expect("Cannot convert to CString");
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform3fv(location, 1, value.as_ptr());
        }
    }

    #[allow(non_snake_case)]
    pub fn uniformMat4f(&mut self, name: &str, value: [[f32; 4]; 4]) {
        let mut mvalue = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                mvalue.push(value[i][j]);
            }
        }
        let name = CString::new(name).expect("Cannot convert to CString");
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, 0, mvalue.as_ptr());
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
        self.attach_with_name(source, kind, "NULL");
    }

    pub fn attach_with_name(&mut self, source: &str, kind: ShaderType, debug_name: &str) {
        let shdr;
        let source = CString::new(source).unwrap();
        unsafe {
            shdr = gl::CreateShader(kind.gl_type());
            if have_debug_gl() {
                let debug_name = debug_name.as_bytes();
                gl::ObjectLabelKHR(
                    gl::GL_SHADER_KHR,
                    shdr,
                    debug_name.len() as i32,
                    debug_name.as_ptr() as *const i8,
                );
            }
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
                panic!("Cannot compile shader: {}", cstr);
            }
        }
        self.shader.push(shdr);
    }

    pub fn compile(&mut self) {
        self.compile_with_name("NULL");
    }

    pub fn compile_with_name(&mut self, debug_name: &str) {
        if self.ready {
            return;
        }
        let program;
        unsafe {
            program = gl::CreateProgram();
            if have_debug_gl() {
                let debug_name = debug_name.as_bytes();
                gl::ObjectLabelKHR(
                    gl::GL_PROGRAM_KHR,
                    program,
                    debug_name.len() as i32,
                    debug_name.as_ptr() as *const i8,
                );
            }
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
                panic!("Cannot link shader: {}", cstr);
            }
        }
        self.program = program;
        self.ready = true;
    }
}

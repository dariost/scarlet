extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use core::ptr::null;
use glutin::window::{Window, WindowBuilder};
use glutin::event_loop::{EventLoop};
use glutin::event::{WindowEvent, Event};
use glutin::{ContextWrapper, GlRequest, PossiblyCurrent};
use glad_gles2::gl;
use scarlet::{Shader, ShaderType};
use std::ffi::c_void;
use std::mem::size_of;
use glutin::event_loop::ControlFlow;

fn main() {
    let width: f32 = 1280.0;
    let height: f32 = 720.0;
    pretty_env_logger::init();
    let gl_version = GlRequest::GlThenGles {
        opengl_version: (4, 3),
        opengles_version: (3, 0),
    };
    let events = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Hello triangle!")
        .with_inner_size(glutin::dpi::LogicalSize::new(width.into(), height.into()))
        .with_resizable(false);
    let windowed_context = glutin::ContextBuilder::new()
        .with_gl(gl_version)
        .with_vsync(true)
        .build_windowed(window_builder, &events)
        .unwrap();
    let context = unsafe { windowed_context.make_current().unwrap() };
    gl::load(|s| context.get_proc_address(s) as *const std::ffi::c_void);
    let limit = (2.0 / f32::sqrt(3.0)) * (height / width);
    #[rustfmt::skip]
    let triangle: Vec<f32> = vec![
        -limit, -1.0, 0.0, 1.0, 0.0, 0.0,
         limit, -1.0, 0.0, 0.0, 1.0, 0.0,
           0.0,  1.0, 0.0, 0.0, 0.0, 1.0
    ];
    let mut vbo: gl::GLuint = 0;
    let mut vao: gl::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::GL_ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::GL_ARRAY_BUFFER,
            (triangle.len() * size_of::<gl::GLfloat>()) as isize,
            triangle.as_ptr() as *const c_void,
            gl::GL_STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::GL_FLOAT,
            gl::GL_FALSE,
            6 * size_of::<gl::GLfloat>() as i32,
            null(),
        );
        gl::VertexAttribPointer(
            1,
            3,
            gl::GL_FLOAT,
            gl::GL_FALSE,
            6 * size_of::<gl::GLfloat>() as i32,
            null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
        );
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::GL_ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    let mut shader = Shader::new();
    shader.attach(include_str!("shader.vert"), ShaderType::Vertex);
    shader.attach(include_str!("shader.frag"), ShaderType::Fragment);
    shader.compile();
    events.run(move |ev, _, cf| {
        *cf = ControlFlow::Wait;
        match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *cf = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => unsafe {
                gl::Clear(gl::GL_COLOR_BUFFER_BIT);
                shader.activate();
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::GL_TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
                gl::Flush();
                context.swap_buffers().unwrap();
            },
            _ => {}
        };
    });
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
    }
}

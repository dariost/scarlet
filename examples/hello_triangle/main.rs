extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use core::ptr::null;
use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{Application, ApplicationAction, Shader, ShaderType};
use std::ffi::c_void;
use std::mem::size_of;

fn main() {
    pretty_env_logger::init();
    let app = Application::with_title("Hello triangle!");
    let (width, height) = app.size();
    let (width, height) = (width as f32, height as f32);
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
    app.run((), move |_, ev| {
        trace!("{:?}", ev);
        match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ApplicationAction::Quit,
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            }
            | Event::NewEvents(StartCause::ResumeTimeReached { .. }) => unsafe {
                gl::Clear(gl::GL_COLOR_BUFFER_BIT);
                shader.activate();
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::GL_TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
    #[allow(unreachable_code)]
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
    }
}

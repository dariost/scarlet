extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;

use core::ptr::null;
use glad_gles2::gl;
use scarlet::logging;
use scarlet::{Application, Shader, ShaderType};
use std::ffi::c_void;
use std::mem::size_of;

fn main() {
    let width: f32 = 1280.0;
    let height: f32 = 720.0;
    logging::init();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello triangle!")
        .with_dimensions(glutin::dpi::LogicalSize::new(width.into(), height.into()))
        .with_resizable(false);
    let mut app = Application::new(window_builder, true);
    scarlet::init(&mut app, |app| {
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
        let cycle = move |ev: &mut glutin::EventsLoop| {
            let mut should_stop = false;
            unsafe {
                gl::Clear(gl::GL_COLOR_BUFFER_BIT);
                shader.activate();
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::GL_TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
                gl::Flush();
                trace!("glGetError() = {}", gl::GetError());
            }
            ev.poll_events(|e| match e {
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::CloseRequested,
                    ..
                } => should_stop = true,
                _ => {}
            });
            !should_stop
        };
        app.run(cycle);
        unsafe {
            gl::DeleteVertexArrays(1, &mut vao);
            gl::DeleteBuffers(1, &mut vbo);
        }
    });
}

extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
//#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction, Shader, ShaderType};

fn main() {
    pretty_env_logger::init();
    let app = Application::with_title("Hello suzanne!");
    let (width, height) = app.size();
    let (width, height) = (width as f32, height as f32);
    let suzanne = import_scene(include_bytes!("suzanne.glb"), width / height);
    let mut shader = Shader::new();
    shader.attach(include_str!("shader.vert"), ShaderType::Vertex);
    shader.attach(include_str!("shader.frag"), ShaderType::Fragment);
    shader.compile();
    app.run((suzanne, shader), move |up, ev| {
        let suzanne = &mut up.0;
        let shader = &mut up.1;
        //trace!("{:?}", ev);
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
                suzanne.draw(shader);
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

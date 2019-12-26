extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
//#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;
use std::env;
use std::fs;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{
    import_scene, Application, ApplicationAction, ApplicationOptions, Shader, ShaderType,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Model viewer");
    opt.fps = 60.0;
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let (width, height) = (width as f32, height as f32);
    let model = import_scene(
        &fs::read(&args[1]).expect("cannot read file"),
        width / height,
    );
    let mut shader = Shader::new();
    shader.attach(include_str!("shader.vert"), ShaderType::Vertex);
    shader.attach(include_str!("shader.frag"), ShaderType::Fragment);
    shader.compile();
    app.run((model, shader), move |up, ev| {
        let model = &mut up.0;
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
                gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
                model.draw(shader);
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

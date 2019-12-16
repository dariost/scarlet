extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction};

fn main() {
    pretty_env_logger::init();
    let app = Application::with_title("Hello suzanne!");
    let (width, height) = app.size();
    let (width, height) = (width as f32, height as f32);
    let _suzanne = import_scene(include_bytes!("suzanne.glb"), width / height);
    //println!("{:?}", suzanne);
    app.run(move |ev| {
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
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

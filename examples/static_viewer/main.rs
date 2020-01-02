extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
//#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};
use std::env::args;
use std::fs;

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Scarlet model viewer");
    opt.fps = 60.0;
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let model = import_scene(
        &fs::read(args().nth(1).expect("missing args")).expect("cannot read file"),
        width,
        height,
    );
    app.run(model, move |model, ev| {
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
                model.draw();
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

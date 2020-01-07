extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
//#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use na::geometry::UnitQuaternion;
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Hello Suzanne!");
    opt.fps = 60.0;
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let suzanne = import_scene(include_bytes!("suzanne.glb"), width, height);
    let sm = suzanne.get_node("Suzanne").expect("No monkey!");
    app.run((suzanne, sm), move |up, ev| {
        let suzanne = &mut up.0;
        let sm = &mut up.1;
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
                sm.borrow_mut()
                    .transform
                    .append_rotation_mut(&UnitQuaternion::<f32>::from_euler_angles(0.0, 0.01, 0.0));
                suzanne.draw("final");
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

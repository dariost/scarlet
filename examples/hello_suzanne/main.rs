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
use scarlet::{
    import_scene, Application, ApplicationAction, ApplicationOptions, Shader, ShaderType,
};

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Hello Suzanne!");
    opt.fps = 60.0;
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let (width, height) = (width as f32, height as f32);
    let suzanne = import_scene(include_bytes!("suzanne.glb"), width / height);
    let sm = suzanne.get_node("Suzanne").expect("No monkey!");
    let mut shader = Shader::new();
    shader.attach(include_str!("shader.vert"), ShaderType::Vertex);
    shader.attach(include_str!("shader.frag"), ShaderType::Fragment);
    shader.compile();
    app.run((suzanne, shader, sm), move |up, ev| {
        let suzanne = &mut up.0;
        let shader = &mut up.1;
        let sm = &mut up.2;
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
                gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
                suzanne.draw(shader);
                gl::Flush();
                ApplicationAction::Refresh
            },
            _ => ApplicationAction::Nothing,
        }
    });
}

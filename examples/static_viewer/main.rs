extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};
use std::env::args;
use std::fs;
use std::time::Instant;

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Scarlet model viewer");
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let model = import_scene(
        &fs::read(args().nth(1).expect("missing args")).expect("cannot read file"),
        width,
        height,
    );
    let last_time = Instant::now();
    let frame_index = 0;
    app.run(
        (model, last_time, frame_index),
        move |(model, last_time, frame_index), ev| {
            //trace!("{:?}", ev);
            let frames = [
                "ssr-final",
                "position",
                "normal",
                "albedo",
                "metalness",
                "roughness",
                "depth",
                "pbr",
                "ssr",
                "ssr-blur-horiz",
                "ssr-blur-vert",
            ];
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
                    model.draw(frames[*frame_index]);
                    gl::Flush();
                    let now = Instant::now();
                    if now.duration_since(*last_time).as_secs_f64() >= 1.0 {
                        *last_time = now;
                        info!("FPS: {}", model.get_fps());
                    }
                    ApplicationAction::Refresh
                },
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Right),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    *frame_index = (*frame_index + 1) % frames.len();
                    ApplicationAction::Nothing
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Left),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    *frame_index = (*frame_index + frames.len() - 1) % frames.len();
                    ApplicationAction::Nothing
                }
                _ => ApplicationAction::Nothing,
            }
        },
    );
}

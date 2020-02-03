extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent};
use scarlet::scene::Scene;
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};
use std::env::args;
use std::f32;
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
            let get_roughness = |model: &mut Scene| {
                let floor = model.get_node("Plane.002").unwrap();
                let mut floor = floor.borrow_mut();
                let mesh = floor.mesh.as_mut().unwrap();
                let material = &mut mesh.data[0].material;
                material.roughness
            };
            let set_roughness = |model: &mut Scene, factor: f32| {
                let floor = model.get_node("Plane.002").unwrap();
                let mut floor = floor.borrow_mut();
                let mesh = floor.mesh.as_mut().unwrap();
                let mut material = &mut mesh.data[0].material;
                material.roughness = factor;
            };
            let frames = [
                "final",
                "position",
                "normal",
                "albedo",
                "metalness",
                "roughness",
                "depth",
                "pbr",
                "ssr",
            ];
            match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => ApplicationAction::Quit,
                Event::RedrawRequested { .. }
                | Event::NewEvents(StartCause::ResumeTimeReached { .. }) => unsafe {
                    model.draw(frames[*frame_index], true);
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
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Up),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let r = f32::min(1.0, get_roughness(model) + 0.05);
                    set_roughness(model, r);
                    info!("Roughness: {}", r);
                    ApplicationAction::Nothing
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Down),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let r = f32::max(0.0, get_roughness(model) - 0.05);
                    set_roughness(model, r);
                    info!("Roughness: {}", r);
                    ApplicationAction::Nothing
                }
                _ => ApplicationAction::Nothing,
            }
        },
    );
}

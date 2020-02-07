extern crate glad_gles2;
extern crate glutin;
extern crate nalgebra as na;
extern crate pretty_env_logger;
extern crate scarlet;
extern crate serde;
extern crate serde_json;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};
use serde::Serialize;
use std::env::args;
use std::fs;
use std::time::Instant;

#[derive(Serialize)]
struct BenchOutput {
    name: String,
    fps: Vec<f64>,
    time: Vec<f64>,
}

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Scarlet model viewer");
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let mname = args().nth(1).expect("missing args");
    let model = import_scene(&fs::read(&mname).expect("cannot read file"), width, height);
    let frame_index = 0;
    let result = BenchOutput {
        name: mname,
        fps: Vec::new(),
        time: Vec::new(),
    };
    let last_time = Instant::now();
    let start_time = Instant::now();
    app.run(
        (model, last_time, frame_index, result, start_time),
        move |(model, last_time, frame_index, result, start_time), ev| {
            //trace!("{:?}", ev);
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
                    let ret = model.draw(frames[*frame_index], true);
                    gl::Flush();
                    let now = Instant::now();
                    if now.duration_since(*last_time).as_secs_f64() >= 0.25 {
                        *last_time = now;
                        result.fps.push(model.get_fps());
                        result
                            .time
                            .push(now.duration_since(*start_time).as_secs_f64());
                    }
                    if ret {
                        fs::write("benchmark.json", serde_json::to_string(&result).unwrap())
                            .unwrap();
                        return ApplicationAction::Quit;
                    }
                    ApplicationAction::Refresh
                },
                _ => ApplicationAction::Nothing,
            }
        },
    );
}

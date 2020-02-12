extern crate glad_gles2;
extern crate glutin;
extern crate scarlet;
#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate png;
extern crate pretty_env_logger;

use glad_gles2::gl;
use glutin::event::{Event, StartCause, WindowEvent};
use scarlet::{import_scene, Application, ApplicationAction, ApplicationOptions};
use std::env::args;
use std::ffi::c_void;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

fn main() {
    pretty_env_logger::init();
    let mut opt = ApplicationOptions::default();
    opt.title = String::from("Scarlet model viewer");
    let app = Application::with_options(&opt);
    let (width, height) = app.size();
    let framebuffer = vec![0_u8; width as usize * height as usize * 3];
    let model = import_scene(
        &fs::read(args().nth(1).expect("missing args")).expect("cannot read file"),
        width,
        height,
    );
    let last_time = Instant::now();
    let frame_index = 0;
    let frame_cnt = 0;
    app.run(
        (
            model,
            last_time,
            frame_index,
            framebuffer,
            width,
            height,
            frame_cnt,
        ),
        move |(model, last_time, frame_index, framebuffer, w, h, frame_cnt), ev| {
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
                    let ret = model.draw(frames[*frame_index], false);
                    gl::Finish();
                    gl::ReadPixels(
                        0,
                        0,
                        *w as i32,
                        *h as i32,
                        gl::GL_RGB,
                        gl::GL_UNSIGNED_BYTE,
                        framebuffer.as_mut_ptr() as *mut c_void,
                    );
                    gl::Finish();
                    for row in 0..(*h as usize / 2) {
                        let of1 = row * (*w as usize * 3);
                        let of2 = (*h as usize - row - 1) * (*w as usize * 3);
                        for col in 0..(*w as usize * 3) {
                            framebuffer.swap(of1 + col, of2 + col);
                        }
                    }
                    let frame_name = format!("frame{:06}.png", frame_cnt);
                    let path = Path::new(&frame_name);
                    let file = File::create(path).expect(&format!("Cannot create {}", frame_name));
                    let writer = BufWriter::new(file);
                    let mut encoder = png::Encoder::new(writer, *w, *h);
                    encoder.set_color(png::ColorType::RGB);
                    encoder.set_depth(png::BitDepth::Eight);
                    let mut true_writer = encoder.write_header().unwrap();
                    true_writer.write_image_data(&framebuffer).unwrap();
                    info!("Written {}", frame_name);
                    let now = Instant::now();
                    if now.duration_since(*last_time).as_secs_f64() >= 1.0 {
                        *last_time = now;
                        info!("FPS: {}", model.get_fps());
                    }
                    *frame_cnt += 1;
                    if ret {
                        return ApplicationAction::Quit;
                    }
                    ApplicationAction::Refresh
                },
                _ => ApplicationAction::Nothing,
            }
        },
    );
}

#[cfg(target_os = "emscripten")]
use crate::emscripten;
use glad_gles2::gl;
use glutin::{ContextWrapper, EventsLoop, GlRequest, PossiblyCurrent, Window, WindowBuilder};
use std::ffi::CStr;
use std::thread::yield_now;

pub struct Application {
    events: EventsLoop,
    context: ContextWrapper<PossiblyCurrent, Window>,
}

impl Application {
    pub fn new(window_builder: WindowBuilder, vsync: bool) -> Application {
        let events = EventsLoop::new();
        #[cfg(not(target_os = "emscripten"))]
        let gl_version = GlRequest::GlThenGles {
            opengl_version: (4, 3),
            opengles_version: (3, 0),
        };
        #[cfg(target_os = "emscripten")]
        let gl_version = GlRequest::Specific(glutin::Api::WebGl, (2, 0));
        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(gl_version)
            .with_vsync(vsync)
            .build_windowed(window_builder, &events)
            .unwrap();
        let context = unsafe { windowed_context.make_current().unwrap() };
        gl::load(|s| context.get_proc_address(s) as *const std::ffi::c_void);
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            let version = gl::GetString(gl::GL_VERSION);
            info!(
                "OpenGL version: {}",
                CStr::from_ptr(version as *const i8).to_str().unwrap()
            );
        }
        Application { events, context }
    }

    pub fn run<T: FnMut(&mut EventsLoop) -> bool>(&mut self, mut function: T) {
        #[cfg(not(target_os = "emscripten"))]
        {
            let mut should_continue = true;
            while should_continue {
                should_continue = function(&mut self.events);
                self.context.swap_buffers().unwrap();
                yield_now();
            }
        }
        #[cfg(target_os = "emscripten")]
        {
            emscripten::set_main_loop_callback(|| {
                function(&mut self.events);
                self.context.swap_buffers().unwrap();
                yield_now();
            });
        }
    }
}

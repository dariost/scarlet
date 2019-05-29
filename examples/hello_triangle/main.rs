extern crate glad_gles2;
extern crate glutin;

use glad_gles2::gl;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello triangle!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1280.0, 720.0))
        .with_resizable(false);
    let windowed_context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (4, 3),
            opengles_version: (3, 0),
        })
        .build_windowed(window_builder, &events_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load(|s| windowed_context.get_proc_address(s) as *const std::ffi::c_void);
    let mut should_stop = false;
    unsafe {
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        let version = gl::GetString(gl::GL_VERSION);
        println!(
            "OpenGL version: {}",
            std::ffi::CStr::from_ptr(version as *const i8)
                .to_str()
                .unwrap()
        );
    }
    while !should_stop {
        unsafe {
            gl::Clear(gl::GL_COLOR_BUFFER_BIT);
        }
        events_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent {
                event: glutin::WindowEvent::CloseRequested,
                ..
            } => should_stop = true,
            _ => {}
        });
        windowed_context.swap_buffers().unwrap();
        std::thread::yield_now();
    }
}

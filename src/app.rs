use glad_gles2::gl;
use glutin::dpi::LogicalSize;
use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};
use std::env::var;
use std::ffi::CStr;
use std::thread::yield_now;
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
fn is_wayland(ev: &EventLoop<()>) -> bool {
    use glutin::platform::unix::EventLoopExtUnix;
    ev.is_wayland()
}

#[cfg(not(target_os = "linux"))]
fn is_wayland(_: &EventLoop) -> bool {
    false
}

thread_local! {
    pub(crate) static GL: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub(crate) fn have_gl() -> bool {
    GL.with(|f| f.get())
}

pub(crate) fn set_gl(value: bool) {
    GL.with(|f| f.set(value))
}

pub struct Application {
    pub event_loop: EventLoop<()>,
    pub context: WindowedContext<PossiblyCurrent>,
    width: u32,
    height: u32,
    interval: Duration,
}

pub struct ApplicationOptions {
    title: String,
    fullscreen: bool,
    vsync: bool,
    width: u32,
    height: u32,
    fps: f32,
}

pub enum ApplicationAction {
    Refresh,
    Quit,
    Nothing,
}

impl Application {
    pub fn new() -> Application {
        Self::with_options(&ApplicationOptions::default())
    }

    pub fn with_title(title: &str) -> Application {
        let mut opt = ApplicationOptions::default();
        opt.title = String::from(title);
        Self::with_options(&opt)
    }

    pub fn with_options(options: &ApplicationOptions) -> Application {
        let event_loop = EventLoop::new();
        let primary_monitor = event_loop.primary_monitor();
        let window_builder = WindowBuilder::new();
        let window_builder = window_builder.with_title(&options.title);
        let window_builder = if options.fullscreen {
            window_builder
                .with_inner_size(LogicalSize::from_physical(
                    primary_monitor.size(),
                    primary_monitor.hidpi_factor(),
                ))
                .with_fullscreen(Some(primary_monitor))
        } else {
            window_builder.with_inner_size(LogicalSize::from((options.width, options.height)))
        };
        let window_builder = window_builder.with_resizable(false);
        let context_builder = ContextBuilder::new();
        let context_builder = context_builder.with_vsync(options.vsync);
        let context_builder = context_builder.with_gl(GlRequest::GlThenGles {
            opengl_version: (4, 3),
            opengles_version: (3, 0),
        });
        let context_builder = context_builder.with_gl_profile(GlProfile::Core);
        let context = context_builder
            .build_windowed(window_builder, &event_loop)
            .expect("Cannot create OpenGL context");
        let context = unsafe {
            context
                .make_current()
                .expect("Cannot make OpenGL context current")
        };
        gl::load(|s| context.get_proc_address(s) as *const std::ffi::c_void);
        set_gl(true);
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            let opengl_version = gl::GetString(gl::GL_VERSION);
            info!(
                "OpenGL version: {}",
                CStr::from_ptr(opengl_version as *const i8)
                    .to_str()
                    .unwrap()
            );
        }
        let logic_size = context.window().inner_size();
        let current_monitor = if is_wayland(&event_loop) {
            event_loop.primary_monitor()
        } else {
            context.window().current_monitor()
        };
        let physical_size = logic_size.to_physical(current_monitor.hidpi_factor());
        let (width, height) = (physical_size.width as u32, physical_size.height as u32);
        info!("Window size: {}×{}", width, height);
        let interval = if options.fps > 0.0 {
            Duration::from_micros((1000000.0 / options.fps) as u64)
        } else {
            Duration::from_millis(1)
        };
        info!(
            "Max framerate: {} fps",
            if options.fps > 0.0 {
                format!("{}", options.fps)
            } else {
                String::from("∞")
            }
        );
        Application {
            event_loop,
            context,
            width,
            height,
            interval,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn run<T: 'static + Fn(Event<()>) -> ApplicationAction>(self, f: T) -> ! {
        let context = self.context;
        let interval = self.interval;
        let mut next_refresh = Instant::now() + interval;
        self.event_loop.run(move |ev, _wt, cf| {
            let action = f(ev);
            match action {
                ApplicationAction::Refresh => {
                    context.swap_buffers().expect("Cannot swap buffers");
                    next_refresh = Instant::now() + interval;
                    *cf = ControlFlow::WaitUntil(next_refresh);
                }
                ApplicationAction::Quit => {
                    set_gl(false);
                    *cf = ControlFlow::Exit
                }
                ApplicationAction::Nothing => {
                    *cf = ControlFlow::WaitUntil(next_refresh);
                    yield_now();
                }
            }
        })
    }
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        ApplicationOptions {
            title: String::from("Scarlet"),
            fullscreen: var("SCARLET_FULLSCREEN")
                .ok()
                .map_or(false, |s| s.parse::<usize>().unwrap_or(0) != 0),
            vsync: var("SCARLET_VSYNC")
                .ok()
                .map_or(false, |s| s.parse::<usize>().unwrap_or(0) != 0),
            width: 1280,
            height: 720,
            fps: var("SCARLET_FPS")
                .ok()
                .map_or(0.0, |s| s.parse::<f32>().unwrap_or(0.0)),
        }
    }
}
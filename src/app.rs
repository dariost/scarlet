use glad_gles2::gl;
use glutin::dpi::LogicalSize;
use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};
use std::env::var;
use std::ffi::CStr;
use std::ptr::null;
use std::thread::yield_now;
use std::time::{Duration, Instant};

/*#[cfg(target_os = "linux")]
fn is_wayland(ev: &EventLoop<()>) -> bool {
    use glutin::platform::unix::EventLoopWindowTargetExtUnix;
    ev.is_wayland()
}

#[cfg(not(target_os = "linux"))]
fn is_wayland(_: &EventLoop) -> bool {
    false
}*/

thread_local! {
    pub(crate) static GL: std::cell::Cell<bool> = std::cell::Cell::new(false);
    pub(crate) static DEBUG_GL: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub(crate) fn have_gl() -> bool {
    GL.with(|f| f.get())
}

pub(crate) fn set_gl(value: bool) {
    GL.with(|f| f.set(value))
}

pub(crate) fn have_debug_gl() -> bool {
    DEBUG_GL.with(|f| f.get())
}

pub(crate) fn set_debug_gl(value: bool) {
    DEBUG_GL.with(|f| f.set(value))
}

extern "system" fn gldebug(
    source: gl::GLenum,
    type_: gl::GLenum,
    id: gl::GLuint,
    severity: gl::GLenum,
    _: gl::GLsizei,
    message: *const gl::GLchar,
    _: *mut gl::GLvoid,
) {
    let source = match source {
        gl::GL_DEBUG_SOURCE_API_KHR => "API",
        gl::GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR => "WINDOW_SYSTEM",
        gl::GL_DEBUG_SOURCE_SHADER_COMPILER_KHR => "SHADER_COMPILER",
        gl::GL_DEBUG_SOURCE_THIRD_PARTY_KHR => "THIRD_PARTY",
        gl::GL_DEBUG_SOURCE_APPLICATION_KHR => "APPLICATION",
        gl::GL_DEBUG_SOURCE_OTHER_KHR => "OTHER",
        _ => "NULL",
    };
    let type_ = match type_ {
        gl::GL_DEBUG_TYPE_ERROR_KHR => "ERROR",
        gl::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR => "DEPRECATED_BEHAVIOR",
        gl::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR => "UNDEFINED_BEHAVIOR",
        gl::GL_DEBUG_TYPE_PORTABILITY_KHR => "PORTABILITY",
        gl::GL_DEBUG_TYPE_PERFORMANCE_KHR => "PERFORMANCE",
        gl::GL_DEBUG_TYPE_MARKER_KHR => "MARKER",
        gl::GL_DEBUG_TYPE_PUSH_GROUP_KHR => "PUSH_GROUP",
        gl::GL_DEBUG_TYPE_POP_GROUP_KHR => "POP_GROUP",
        gl::GL_DEBUG_TYPE_OTHER_KHR => "OTHER",
        _ => "NULL",
    };
    let message = unsafe { CStr::from_ptr(message) };
    let msg = message.to_str().unwrap_or("NULL");
    let msg = format!(
        "{} | ID = {} | SOURCE = {} | TYPE = {}",
        msg, id, source, type_
    );
    match severity {
        gl::GL_DEBUG_SEVERITY_HIGH_KHR | gl::GL_DEBUG_SEVERITY_MEDIUM_KHR => error!("{}", msg),
        gl::GL_DEBUG_SEVERITY_LOW_KHR => warn!("{}", msg),
        gl::GL_DEBUG_SEVERITY_NOTIFICATION_KHR => info!("{}", msg),
        _ => info!("{}", msg),
    }
}

pub struct Application {
    pub event_loop: EventLoop<()>,
    pub context: WindowedContext<PossiblyCurrent>,
    width: u32,
    height: u32,
    interval: Duration,
}

pub struct ApplicationOptions {
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub debug_gl: bool,
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
                .with_inner_size(primary_monitor.size())
                .with_decorations(false)
                .with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)))
        } else {
            window_builder.with_inner_size(LogicalSize::new(options.width, options.height))
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
        if options.debug_gl {
            if unsafe { gl::storage::DebugMessageCallbackKHR.is_loaded } {
                set_debug_gl(true);
            } else {
                warn!("Cannot enable GL debugging, GL_KHR_debug not found!");
            }
        }
        unsafe {
            if have_debug_gl() {
                gl::Enable(gl::GL_DEBUG_OUTPUT_KHR);
                gl::DebugMessageCallbackKHR(gldebug, null());
            }
            gl::Enable(gl::GL_DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            let opengl_version = gl::GetString(gl::GL_VERSION);
            info!(
                "OpenGL version: {}",
                CStr::from_ptr(opengl_version as *const i8)
                    .to_str()
                    .unwrap()
            );
            let mut max_draw_buffers: gl::GLint = 0;
            let mut max_color_attachments: gl::GLint = 0;
            gl::GetIntegerv(gl::GL_MAX_DRAW_BUFFERS, &mut max_draw_buffers);
            gl::GetIntegerv(gl::GL_MAX_COLOR_ATTACHMENTS, &mut max_color_attachments);
            info!("GL_MAX_DRAW_BUFFERS: {}", max_draw_buffers);
            info!("GL_MAX_COLOR_ATTACHMENTS: {}", max_color_attachments);
            if max_draw_buffers < 8 || max_color_attachments < 8 {
                error!("GL_MAX_DRAW_BUFFERS or GL_MAX_COLOR_ATTACHMENTS is less than 8, expect breakage");
            }
        }
        /*let current_monitor = if is_wayland(&event_loop) {
            event_loop.primary_monitor()
        } else {
            context.window().current_monitor()
        };*/
        let physical_size = context.window().inner_size();
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

    pub fn run<U: 'static, T: 'static + Fn(&mut U, Event<()>) -> ApplicationAction>(
        self,
        mut up: U,
        f: T,
    ) -> ! {
        let context = self.context;
        let interval = self.interval;
        let mut next_refresh = Instant::now() + interval;
        self.event_loop.run(move |ev, _wt, cf| {
            let action = f(&mut up, ev);
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
            debug_gl: var("SCARLET_DEBUG_GL")
                .ok()
                .map_or(false, |s| s.parse::<usize>().unwrap_or(0) != 0),
        }
    }
}

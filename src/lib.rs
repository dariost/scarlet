extern crate glad_gles2;
extern crate glutin;
#[macro_use]
extern crate log;
#[cfg(target_os = "emscripten")]
extern crate console_log;
#[cfg(not(target_os = "emscripten"))]
extern crate pretty_env_logger;

pub mod application;
#[cfg(target_os = "emscripten")]
pub mod emscripten;
pub mod logging;
pub mod shader;

pub use application::Application;
pub use shader::{Shader, ShaderType};

pub fn init<T: FnOnce(&mut Application)>(app: &mut Application, function: T) {
    function(app);
}

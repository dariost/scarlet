extern crate glad_gles2;
#[macro_use]
extern crate log;
extern crate glutin;

pub mod app;
pub mod shader;

pub(crate) use app::{have_debug_gl, have_gl};

pub use app::{Application, ApplicationAction, ApplicationOptions};
pub use shader::{Shader, ShaderType};

extern crate glad_gles2;
#[macro_use]
extern crate log;
extern crate glutin;

pub mod shader;
pub mod app;

pub(crate) use app::have_gl;

pub use shader::{Shader, ShaderType};
pub use app::{Application, ApplicationOptions, ApplicationAction};

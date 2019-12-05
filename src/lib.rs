extern crate glad_gles2;
#[macro_use]
extern crate log;
extern crate gltf;
extern crate glutin;
extern crate nalgebra;

pub mod app;
pub mod scene;
pub mod shader;

pub(crate) use app::{have_debug_gl, have_gl};

pub use app::{Application, ApplicationAction, ApplicationOptions};
pub use scene::import_scene;
pub use shader::{Shader, ShaderType};

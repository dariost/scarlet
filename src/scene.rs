use crate::have_gl;
use crate::shader::{Shader, ShaderType};
use glad_gles2::gl;
use gltf::animation::util::ReadOutputs;
use gltf::buffer::Source;
use gltf::image;
use gltf::khr_lights_punctual::Kind;
use na::geometry::{Perspective3, Point3, Quaternion, Similarity3, Translation3, UnitQuaternion};
use nalgebra as na;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::rc::{Rc, Weak};
use std::time::Instant;

#[derive(Debug)]
struct RenderPasses {
    g_buffer: gl::GLuint,
    g_position: gl::GLuint,
    g_normal: gl::GLuint,
    g_albedo: gl::GLuint,
    g_metalness: gl::GLuint,
    g_roughness: gl::GLuint,
    g_depth: gl::GLuint,
    g_pbr: gl::GLuint,
    g_pbrfb: gl::GLuint,
    g_ssr: gl::GLuint,
    g_ssrfb: gl::GLuint,
    g_ssra: gl::GLuint,
    g_ssrafb: gl::GLuint,
    q_vao: gl::GLuint,
    q_vbo: gl::GLuint,
    r_rgb: Shader,
    r_r: Shader,
}

#[derive(Debug)]
pub struct Scene {
    root: SceneNode,
    camera: SceneNode,
    lights: Vec<SceneNode>,
    width: u32,
    height: u32,
    passes: RenderPasses,
    prepare_shader: Shader,
    pbr_shader: Shader,
    ssr_shader: Shader,
    ssr_apply_shader: Shader,
    fps: VecDeque<f64>,
    fps_total: f64,
    last_frame_time: Instant,
    animation: HashMap<usize, BTreeMap<u64, Similarity3<f32>>>,
    animation_step: Vec<u64>,
    start_time: Instant,
    frame_count: usize,
}

#[derive(Debug)]
pub struct RealSceneNode {
    pub id: usize,
    pub transform: Similarity3<f32>,
    children: Vec<SceneNode>,
    parent: Option<Weak<RefCell<RealSceneNode>>>,
    name: String,
    pub camera: Option<Camera>,
    pub light: Option<Light>,
    pub mesh: Option<Mesh>,
}

#[derive(Debug)]
pub struct Camera {
    perspective: Perspective3<f32>,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Light {
    color: [f32; 3],
    name: String,
    intensity: f32,
    directional: bool,
}

#[derive(Debug)]
pub struct RenderData {
    vao: gl::GLuint,
    vbo: gl::GLuint,
    mode: gl::GLuint,
    pub material: Material,
    buffer: Vec<f32>,
    n_elements: i32,
}

impl RenderData {
    pub fn draw(&self, shader: &mut Shader) {
        shader.uniform1i("material.albedo_sampler", 0);
        shader.uniform4f("material.albedo", self.material.color);
        shader.uniform1f("material.metalness", self.material.metallic);
        shader.uniform1f("material.roughness", self.material.roughness);
        unsafe {
            gl::ActiveTexture(gl::GL_TEXTURE0);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.material.albedo);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.mode, 0, self.n_elements);
        }
    }
}

impl RenderPasses {
    pub fn new(width: gl::GLsizei, height: gl::GLsizei) -> RenderPasses {
        let mut g_buffer: gl::GLuint = 0;
        let mut g_position: gl::GLuint = 0;
        let mut g_normal: gl::GLuint = 0;
        let mut g_albedo: gl::GLuint = 0;
        let mut g_metalness: gl::GLuint = 0;
        let mut g_roughness: gl::GLuint = 0;
        let mut g_depth: gl::GLuint = 0;
        let mut g_pbr: gl::GLuint = 0;
        let mut g_pbrfb: gl::GLuint = 0;
        let mut g_ssr: gl::GLuint = 0;
        let mut g_ssrfb: gl::GLuint = 0;
        let mut g_ssra: gl::GLuint = 0;
        let mut g_ssrafb: gl::GLuint = 0;
        let mut q_vao: gl::GLuint = 0;
        let mut q_vbo: gl::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut g_buffer);
            gl::GenFramebuffers(1, &mut g_pbrfb);
            gl::GenFramebuffers(1, &mut g_ssrfb);
            gl::GenFramebuffers(1, &mut g_ssrafb);
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, g_buffer);
            let bind = |buf, internal_format, format, kind, attachment| {
                gl::GenTextures(1, buf);
                gl::BindTexture(gl::GL_TEXTURE_2D, *buf);
                gl::TexImage2D(
                    gl::GL_TEXTURE_2D,
                    0,
                    internal_format as gl::GLint,
                    width,
                    height,
                    0,
                    format,
                    kind,
                    null(),
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MIN_FILTER,
                    gl::GL_NEAREST as gl::GLint,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MAG_FILTER,
                    gl::GL_NEAREST as gl::GLint,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_S,
                    gl::GL_CLAMP_TO_EDGE as gl::GLint,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_T,
                    gl::GL_CLAMP_TO_EDGE as gl::GLint,
                );
                gl::FramebufferTexture2D(
                    gl::GL_FRAMEBUFFER,
                    attachment,
                    gl::GL_TEXTURE_2D,
                    *buf,
                    0,
                );
            };
            bind(
                &mut g_position,
                gl::GL_RGB16F,
                gl::GL_RGB,
                gl::GL_HALF_FLOAT,
                gl::GL_COLOR_ATTACHMENT0,
            );
            bind(
                &mut g_normal,
                gl::GL_RGB16F,
                gl::GL_RGB,
                gl::GL_HALF_FLOAT,
                gl::GL_COLOR_ATTACHMENT1,
            );
            bind(
                &mut g_albedo,
                gl::GL_RGB,
                gl::GL_RGB,
                gl::GL_UNSIGNED_BYTE,
                gl::GL_COLOR_ATTACHMENT2,
            );
            bind(
                &mut g_metalness,
                gl::GL_R16F,
                gl::GL_RED,
                gl::GL_HALF_FLOAT,
                gl::GL_COLOR_ATTACHMENT3,
            );
            bind(
                &mut g_roughness,
                gl::GL_R16F,
                gl::GL_RED,
                gl::GL_HALF_FLOAT,
                gl::GL_COLOR_ATTACHMENT4,
            );
            bind(
                &mut g_depth,
                gl::GL_DEPTH_COMPONENT32F,
                gl::GL_DEPTH_COMPONENT,
                gl::GL_FLOAT,
                gl::GL_DEPTH_ATTACHMENT,
            );
            let draw_buffers = [
                gl::GL_COLOR_ATTACHMENT0,
                gl::GL_COLOR_ATTACHMENT1,
                gl::GL_COLOR_ATTACHMENT2,
                gl::GL_COLOR_ATTACHMENT3,
                gl::GL_COLOR_ATTACHMENT4,
            ];
            gl::DrawBuffers(5, draw_buffers.as_ptr());
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, g_pbrfb);
            bind(
                &mut g_pbr,
                gl::GL_RGB,
                gl::GL_RGB,
                gl::GL_UNSIGNED_BYTE,
                gl::GL_COLOR_ATTACHMENT0,
            );
            let draw_buffers = [gl::GL_COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, draw_buffers.as_ptr());
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, g_ssrfb);
            bind(
                &mut g_ssr,
                gl::GL_RGB,
                gl::GL_RGB,
                gl::GL_UNSIGNED_BYTE,
                gl::GL_COLOR_ATTACHMENT0,
            );
            let draw_buffers = [gl::GL_COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, draw_buffers.as_ptr());
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, g_ssrafb);
            bind(
                &mut g_ssra,
                gl::GL_RGB,
                gl::GL_RGB,
                gl::GL_UNSIGNED_BYTE,
                gl::GL_COLOR_ATTACHMENT0,
            );
            let draw_buffers = [gl::GL_COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, draw_buffers.as_ptr());
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, 0);
        }
        #[rustfmt::skip]
        let quad: Vec<f32> = vec![
            -1.0, -1.0, 0.0, 0.0,
            -1.0,  1.0, 0.0, 1.0,
             1.0, -1.0, 1.0, 0.0,
             1.0,  1.0, 1.0, 1.0
        ];
        unsafe {
            gl::GenVertexArrays(1, &mut q_vao);
            gl::GenBuffers(1, &mut q_vbo);
            gl::BindVertexArray(q_vao);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, q_vbo);
            gl::BufferData(
                gl::GL_ARRAY_BUFFER,
                (quad.len() * size_of::<gl::GLfloat>()) as isize,
                quad.as_ptr() as *const c_void,
                gl::GL_STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                2,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                4 * size_of::<gl::GLfloat>() as i32,
                null(),
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                4 * size_of::<gl::GLfloat>() as i32,
                null::<c_void>().offset(2 * size_of::<gl::GLfloat>() as isize),
            );
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        let mut r_rgb = Shader::new();
        let mut r_r = Shader::new();
        r_rgb.attach(include_str!("shaders/quad.vert"), ShaderType::Vertex);
        r_rgb.attach(include_str!("shaders/quad_rgb.frag"), ShaderType::Fragment);
        r_rgb.compile();
        r_r.attach(include_str!("shaders/quad.vert"), ShaderType::Vertex);
        r_r.attach(include_str!("shaders/quad_r.frag"), ShaderType::Fragment);
        r_r.compile();
        RenderPasses {
            g_buffer,
            g_position,
            g_normal,
            g_albedo,
            g_metalness,
            g_roughness,
            g_depth,
            q_vao,
            q_vbo,
            r_rgb,
            r_r,
            g_pbr,
            g_pbrfb,
            g_ssr,
            g_ssrfb,
            g_ssra,
            g_ssrafb,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, self.g_buffer);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
        }
    }

    pub fn bind_pbr(&self, shader: &mut Shader) {
        unsafe {
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_pbr);
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MIN_FILTER,
                gl::GL_NEAREST as gl::GLint,
            );
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MAG_FILTER,
                gl::GL_NEAREST as gl::GLint,
            );
            gl::BindTexture(gl::GL_TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, self.g_pbrfb);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
            shader.uniform1i("position_sampler", 0);
            shader.uniform1i("normal_sampler", 1);
            shader.uniform1i("albedo_sampler", 2);
            shader.uniform1i("metalness_sampler", 3);
            shader.uniform1i("roughness_sampler", 4);
            gl::ActiveTexture(gl::GL_TEXTURE0);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_position);
            gl::ActiveTexture(gl::GL_TEXTURE1);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_normal);
            gl::ActiveTexture(gl::GL_TEXTURE2);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_albedo);
            gl::ActiveTexture(gl::GL_TEXTURE3);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_metalness);
            gl::ActiveTexture(gl::GL_TEXTURE4);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_roughness);
            gl::ActiveTexture(gl::GL_TEXTURE0);
        }
    }

    pub fn bind_ssr(&self, shader: &mut Shader) {
        unsafe {
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, self.g_ssrfb);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
            shader.uniform1i("position_sampler", 0);
            shader.uniform1i("normal_sampler", 1);
            shader.uniform1i("pbr_sampler", 2);
            shader.uniform1i("metalness_sampler", 3);
            shader.uniform1i("roughness_sampler", 4);
            shader.uniform1i("depth_sampler", 5);
            gl::ActiveTexture(gl::GL_TEXTURE0);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_position);
            gl::ActiveTexture(gl::GL_TEXTURE1);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_normal);
            gl::ActiveTexture(gl::GL_TEXTURE2);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_pbr);
            gl::GenerateMipmap(gl::GL_TEXTURE_2D);
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MIN_FILTER,
                gl::GL_LINEAR_MIPMAP_LINEAR as gl::GLint,
            );
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MAG_FILTER,
                gl::GL_LINEAR as gl::GLint,
            );
            gl::ActiveTexture(gl::GL_TEXTURE3);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_metalness);
            gl::ActiveTexture(gl::GL_TEXTURE4);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_roughness);
            gl::ActiveTexture(gl::GL_TEXTURE5);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_depth);
            gl::ActiveTexture(gl::GL_TEXTURE0);
        }
    }

    pub fn bind_ssr_apply(&self, shader: &mut Shader) {
        unsafe {
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, self.g_ssrafb);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
            shader.uniform1i("ssr_sampler", 0);
            shader.uniform1i("metalness_sampler", 1);
            shader.uniform1i("pbr_sampler", 2);
            gl::ActiveTexture(gl::GL_TEXTURE0);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_ssr);
            gl::ActiveTexture(gl::GL_TEXTURE1);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_metalness);
            gl::ActiveTexture(gl::GL_TEXTURE2);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.g_pbr);
            gl::ActiveTexture(gl::GL_TEXTURE0);
            gl::ActiveTexture(gl::GL_TEXTURE0);
        }
    }

    pub fn print_quad(&self) {
        unsafe {
            gl::BindVertexArray(self.q_vao);
            gl::DrawArrays(gl::GL_TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }

    pub fn print_buffer(&self, name: &str) {
        unsafe {
            gl::BindFramebuffer(gl::GL_FRAMEBUFFER, 0);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
        }
        let info = match name {
            "position" => (&self.g_position, &self.r_rgb),
            "normal" => (&self.g_normal, &self.r_rgb),
            "albedo" => (&self.g_albedo, &self.r_rgb),
            "metalness" => (&self.g_metalness, &self.r_r),
            "roughness" => (&self.g_roughness, &self.r_r),
            "depth" => (&self.g_depth, &self.r_r),
            "pbr" => (&self.g_pbr, &self.r_rgb),
            "ssr" => (&self.g_ssr, &self.r_rgb),
            "ssr-final" | "final" => (&self.g_ssra, &self.r_rgb),
            _ => panic!("Non existent render buffer"),
        };
        info.1.activate();
        unsafe {
            gl::BindTexture(gl::GL_TEXTURE_2D, *info.0);
        }
        self.print_quad();
        unsafe {
            gl::BindTexture(gl::GL_TEXTURE_2D, 0);
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    name: String,
    pub data: Vec<RenderData>,
}

impl Mesh {
    pub fn draw(&self, shader: &mut Shader) {
        for rd in &self.data {
            rd.draw(shader);
        }
    }
}

#[derive(Debug, Default)]
pub struct Material {
    albedo: gl::GLuint,
    pub color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
}

type SceneNode = Rc<RefCell<RealSceneNode>>;

impl Default for RealSceneNode {
    fn default() -> Self {
        RealSceneNode {
            id: 0,
            transform: Similarity3::<f32>::identity(),
            children: Vec::new(),
            name: String::from("NULL"),
            parent: None,
            camera: None,
            light: None,
            mesh: None,
        }
    }
}

impl Drop for RenderData {
    fn drop(&mut self) {
        if have_gl() {
            unsafe {
                gl::DeleteVertexArrays(1, &mut self.vao);
                gl::DeleteBuffers(1, &mut self.vbo);
            }
        }
    }
}

impl RenderData {
    pub fn new() -> RenderData {
        let mut rd = RenderData {
            vao: 0,
            vbo: 0,
            mode: 0,
            material: Material::default(),
            buffer: Vec::new(),
            n_elements: 0,
        };
        unsafe {
            gl::GenVertexArrays(1, &mut rd.vao);
            gl::GenBuffers(1, &mut rd.vbo);
        };
        rd
    }
}

pub fn create_mesh(
    mesh: gltf::Mesh,
    buffers: &[gltf::buffer::Data],
    images: &[image::Data],
) -> Mesh {
    let name = String::from(mesh.name().unwrap_or("NULL"));
    let mut data = Vec::new();
    for primitive in mesh.primitives() {
        let mut rd = RenderData::new();
        rd.mode = primitive.mode().as_gl_enum();
        let model = primitive.material().pbr_metallic_roughness();
        let mut material = Material {
            albedo: 0,
            color: model.base_color_factor(),
            metallic: model.metallic_factor(),
            roughness: model.roughness_factor(),
        };
        unsafe {
            gl::GenTextures(1, &mut material.albedo);
            gl::BindTexture(gl::GL_TEXTURE_2D, material.albedo);
            gl::PixelStorei(gl::GL_PACK_ALIGNMENT, 1);
        }
        if let Some(texture) = model.base_color_texture() {
            if texture.tex_coord() != 0 {
                error!("Assuming tex_coord 0, but it's {}", texture.tex_coord());
            }
            let texture = texture.texture();
            assert_eq!(texture.index(), texture.source().index());
            let index = texture.index();
            let format = match images[index].format {
                image::Format::R8G8B8 => gl::GL_RGB,
                image::Format::R8G8B8A8 => gl::GL_RGBA,
                _ => unimplemented!(),
            };
            let sampler = texture.sampler();
            unsafe {
                gl::TexImage2D(
                    gl::GL_TEXTURE_2D,
                    0,
                    format as i32,
                    images[index].width as i32,
                    images[index].height as i32,
                    0,
                    format,
                    gl::GL_UNSIGNED_BYTE,
                    images[index].pixels.as_ptr() as *const c_void,
                );
                gl::GenerateMipmap(gl::GL_TEXTURE_2D);
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MIN_FILTER,
                    sampler
                        .min_filter()
                        .unwrap_or(gltf::texture::MinFilter::LinearMipmapLinear)
                        .as_gl_enum() as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MAG_FILTER,
                    sampler
                        .mag_filter()
                        .unwrap_or(gltf::texture::MagFilter::Linear)
                        .as_gl_enum() as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_S,
                    sampler.wrap_s().as_gl_enum() as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_T,
                    sampler.wrap_s().as_gl_enum() as i32,
                );
            }
        } else {
            let white: [u8; 4] = [255; 4];
            unsafe {
                gl::TexImage2D(
                    gl::GL_TEXTURE_2D,
                    0,
                    gl::GL_RGBA as i32,
                    1,
                    1,
                    0,
                    gl::GL_RGBA,
                    gl::GL_UNSIGNED_BYTE,
                    white.as_ptr() as *const c_void,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MIN_FILTER,
                    gl::GL_NEAREST as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_MAG_FILTER,
                    gl::GL_NEAREST as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_S,
                    gl::GL_CLAMP_TO_EDGE as i32,
                );
                gl::TexParameteri(
                    gl::GL_TEXTURE_2D,
                    gl::GL_TEXTURE_WRAP_T,
                    gl::GL_CLAMP_TO_EDGE as i32,
                );
            }
        }
        unsafe {
            gl::BindTexture(gl::GL_TEXTURE_2D, 0);
        }
        rd.material = material;
        let reader = primitive.reader(|x| {
            assert!(match x.source() {
                Source::Bin => true,
                _ => false,
            });
            Some(&buffers[x.index()])
        });
        let pos: Vec<_> = reader.read_positions().expect("No positions!").collect();
        let norm: Vec<_> = reader.read_normals().expect("No normals!").collect();
        let texcoord: Vec<_> = if let Some(tex) = reader.read_tex_coords(0) {
            tex.into_f32().collect()
        } else {
            vec![[0.0, 0.0]; pos.len()]
        };
        let ind: Vec<_> = match reader.read_indices() {
            Some(x) => x.into_u32().map(|y| y as usize).collect(),
            None => (0..pos.len()).collect(),
        };
        for i in ind {
            for j in 0..3 {
                rd.buffer.push(pos[i][j]);
            }
            for j in 0..3 {
                rd.buffer.push(norm[i][j]);
            }
            for j in 0..2 {
                rd.buffer.push(texcoord[i][j]);
            }
            rd.n_elements += 1;
        }
        unsafe {
            gl::BindVertexArray(rd.vao);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, rd.vbo);
            gl::BufferData(
                gl::GL_ARRAY_BUFFER,
                (rd.buffer.len() * size_of::<gl::GLfloat>()) as isize,
                rd.buffer.as_ptr() as *const c_void,
                gl::GL_STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                null(),
            );
            gl::VertexAttribPointer(
                1,
                3,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
            );
            gl::VertexAttribPointer(
                2,
                2,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                null::<c_void>().offset(6 * size_of::<gl::GLfloat>() as isize),
            );
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        data.push(rd);
    }
    Mesh { name, data }
}

pub fn import_scene(asset: &[u8], width: u32, height: u32) -> Scene {
    let aspect_ratio = width as f32 / height as f32;
    let (document, buffers, images) = gltf::import_slice(asset).expect("Cannot import asset!");
    let scene = document.scenes().nth(0).expect("No scenes in asset!");
    let mut root_node = RealSceneNode::default();
    let mut camera = None;
    let mut lights = Vec::new();
    fn construct_scene(
        parent: &mut SceneNode,
        node: gltf::Node,
        mut camera: &mut Option<SceneNode>,
        mut lights: &mut Vec<SceneNode>,
        ar: f32,
        buffers: &[gltf::buffer::Data],
        images: &[image::Data],
    ) {
        let mut scene_node = RealSceneNode::default();
        scene_node.id = node.index();
        scene_node.name = String::from(node.name().unwrap_or("NULL"));
        let (translation, rotation, scaling) = node.transform().decomposed();
        let translation = Translation3::<f32>::new(translation[0], translation[1], translation[2]);
        let rotation = Quaternion::<f32>::new(rotation[3], rotation[0], rotation[1], rotation[2]);
        let rotation = UnitQuaternion::<f32>::from_quaternion(rotation);
        if scaling[0] != scaling[1] || scaling[1] != scaling[2] {
            warn!("Non uniform scaling is not supported!");
        }
        let scaling = scaling[0];
        scene_node.transform = Similarity3::<f32>::from_parts(translation, rotation, scaling);
        let mut scene_node = Rc::new(RefCell::new(scene_node));
        if let Some(ccamera) = node.camera() {
            if camera.is_some() {
                error!("Only one camera allowed!");
            }
            *camera = Some(scene_node.clone());
            let proj = match ccamera.projection() {
                gltf::camera::Projection::Perspective(p) => Perspective3::<f32>::new(
                    p.aspect_ratio().unwrap_or(ar),
                    p.yfov(),
                    p.znear(),
                    p.zfar().unwrap_or(1e9),
                ),
                _ => unimplemented!(),
            };
            scene_node.borrow_mut().camera = Some(Camera {
                perspective: proj,
                name: String::from(ccamera.name().unwrap_or("NULL")),
            });
        }
        if let Some(light) = node.light() {
            scene_node.borrow_mut().light = Some(Light {
                color: light.color(),
                intensity: light.intensity(),
                name: String::from(light.name().unwrap_or("NULL")),
                directional: match light.kind() {
                    Kind::Directional => true,
                    _ => false,
                },
            });
            lights.push(scene_node.clone());
        }
        if let Some(mesh) = node.mesh() {
            scene_node.borrow_mut().mesh = Some(create_mesh(mesh, buffers, images));
        }
        for child in node.children() {
            construct_scene(
                &mut scene_node,
                child,
                &mut camera,
                &mut lights,
                ar,
                buffers,
                images,
            );
        }
        scene_node.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(scene_node);
    }
    root_node.name = String::from("ROOT_NODE");
    let mut root_node = Rc::new(RefCell::new(root_node));
    for node in scene.nodes() {
        construct_scene(
            &mut root_node,
            node,
            &mut camera,
            &mut lights,
            aspect_ratio,
            &buffers,
            &images,
        );
    }
    let mut animation_map: HashMap<usize, BTreeMap<u64, Similarity3<f32>>> = HashMap::new();
    let mut animation_time: BTreeSet<u64> = BTreeSet::new();
    for animation in document.animations() {
        for channel in animation.channels() {
            let target_node = channel.target().node().index();
            let reader = channel.reader(|x| {
                assert!(match x.source() {
                    Source::Bin => true,
                    _ => false,
                });
                Some(&buffers[x.index()])
            });
            let inputs = reader.read_inputs().expect("No animation input!");
            let outputs = reader.read_outputs().expect("No animation output!");
            animation_map.entry(target_node).or_insert(BTreeMap::new());
            let obj_animation = animation_map.get_mut(&target_node).unwrap();
            if let ReadOutputs::Scales(iter) = outputs {
                for anim in inputs.zip(iter) {
                    let tick = (anim.0 * 1000.0) as u64;
                    animation_time.insert(tick);
                    obj_animation
                        .entry(tick)
                        .or_insert(Similarity3::<f32>::identity());
                    if anim.1[0] != anim.1[1] || anim.1[1] != anim.1[2] {
                        error!("Non uniform scaling!");
                    }
                    obj_animation
                        .get_mut(&tick)
                        .unwrap()
                        .append_scaling_mut(anim.1[0]);
                }
            } else if let ReadOutputs::Translations(iter) = outputs {
                for anim in inputs.zip(iter) {
                    let tick = (anim.0 * 1000.0) as u64;
                    animation_time.insert(tick);
                    obj_animation
                        .entry(tick)
                        .or_insert(Similarity3::<f32>::identity());
                    let translation = Translation3::<f32>::new(anim.1[0], anim.1[1], anim.1[2]);
                    obj_animation
                        .get_mut(&tick)
                        .unwrap()
                        .append_translation_mut(&translation);
                }
            } else if let ReadOutputs::Rotations(iter) = outputs {
                let iter = iter.into_f32();
                for anim in inputs.zip(iter) {
                    let tick = (anim.0 * 1000.0) as u64;
                    animation_time.insert(tick);
                    obj_animation
                        .entry(tick)
                        .or_insert(Similarity3::<f32>::identity());
                    let rotation =
                        Quaternion::<f32>::new(anim.1[3], anim.1[0], anim.1[1], anim.1[2]);
                    let rotation = UnitQuaternion::<f32>::from_quaternion(rotation);
                    obj_animation
                        .get_mut(&tick)
                        .unwrap()
                        .append_rotation_mut(&rotation);
                }
            }
        }
    }
    let mut shdr = Shader::new();
    shdr.attach(include_str!("shaders/prepare.vert"), ShaderType::Vertex);
    shdr.attach(include_str!("shaders/prepare.frag"), ShaderType::Fragment);
    shdr.compile();
    let mut pbr = Shader::new();
    pbr.attach(include_str!("shaders/pbr.vert"), ShaderType::Vertex);
    pbr.attach(include_str!("shaders/pbr.frag"), ShaderType::Fragment);
    pbr.compile();
    let mut ssr = Shader::new();
    ssr.attach(include_str!("shaders/ssr.vert"), ShaderType::Vertex);
    ssr.attach(include_str!("shaders/ssr.frag"), ShaderType::Fragment);
    ssr.compile();
    let mut ssra = Shader::new();
    ssra.attach(include_str!("shaders/ssra.vert"), ShaderType::Vertex);
    ssra.attach(include_str!("shaders/ssra.frag"), ShaderType::Fragment);
    ssra.compile();
    let animation_time = animation_time.into_iter().collect();
    Scene {
        root: root_node,
        lights: lights,
        camera: camera.expect("There must be a camera in the scene!"),
        width,
        height,
        passes: RenderPasses::new(width as gl::GLsizei, height as gl::GLsizei),
        prepare_shader: shdr,
        pbr_shader: pbr,
        ssr_shader: ssr,
        ssr_apply_shader: ssra,
        fps: VecDeque::new(),
        fps_total: 0.0,
        last_frame_time: Instant::now(),
        animation: animation_map,
        animation_step: animation_time,
        frame_count: 0,
        start_time: Instant::now(),
    }
}

impl Scene {
    pub fn get_node(&self, name: &str) -> Option<SceneNode> {
        let mut queue = vec![self.root.clone()];
        while let Some(node) = queue.pop() {
            if name == node.borrow().name {
                return Some(node.clone());
            }
            for child in &node.borrow().children {
                queue.push(child.clone());
            }
        }
        None
    }

    pub fn draw(&mut self, frame: &str, realtime: bool) -> bool {
        fn get_animation(scene: &mut Scene, node: usize, realtime: bool) -> Similarity3<f32> {
            let mut animation = Similarity3::<f32>::identity();
            if !realtime {
                if let Some(animap) = scene.animation.get(&node) {
                    if let Some(elem) = animap
                        .range(0..=scene.animation_step[scene.frame_count])
                        .rev()
                        .nth(0)
                    {
                        animation *= elem.1;
                    }
                }
            } else {
                if scene.animation_step.len() > 0 {
                    let tick = (Instant::now().duration_since(scene.start_time).as_millis()
                        % *scene.animation_step.last().unwrap() as u128)
                        as u64;
                    if let Some(animap) = scene.animation.get(&node) {
                        if let Some(elem) = animap.range(0..=tick).rev().nth(0) {
                            animation *= elem.1;
                        }
                    }
                }
            }
            animation
        }
        const MAX_LIGHTS: usize = 32;
        self.passes.bind();
        self.prepare_shader.activate();
        let mut light_info = Vec::new();
        let mut queue = vec![(self.root.clone(), Similarity3::<f32>::identity())];
        let mut camstruct = None;
        while let Some(mut node) = queue.pop() {
            node.1 = node.1 * node.0.borrow().transform;
            node.1 *= get_animation(self, node.0.borrow().id, realtime);
            if let Some(camera) = &node.0.borrow().camera {
                let trans_matrix = node.1;
                let projection = camera.perspective.to_projective();
                camstruct = Some((
                    trans_matrix.transform_point(&Point3::<f32>::new(0.0, 0.0, 0.0)),
                    projection * trans_matrix.inverse(),
                ));
            }
            if let Some(light) = &node.0.borrow().light {
                let trans_matrix = node.1;
                let point = trans_matrix.transform_point(&Point3::<f32>::new(0.0, 0.0, 0.0));
                light_info.push((point, light.clone()));
            }
            for child in &node.0.borrow().children {
                queue.push((child.clone(), node.1));
            }
        }
        let camstruct = camstruct.unwrap();
        let cp = camstruct.0;
        let cm = camstruct.1;
        self.prepare_shader
            .uniformMat4f("camera", cm.to_homogeneous().into());
        let mut queue = vec![(self.root.clone(), Similarity3::<f32>::identity())];
        while let Some(mut node) = queue.pop() {
            node.1 = node.1 * node.0.borrow().transform;
            node.1 *= get_animation(self, node.0.borrow().id, realtime);
            if let Some(mesh) = &node.0.borrow().mesh {
                let trans_matrix = node.1.to_homogeneous().into();
                self.prepare_shader.uniformMat4f("world", trans_matrix);
                mesh.draw(&mut self.prepare_shader);
            }
            for child in &node.0.borrow().children {
                queue.push((child.clone(), node.1));
            }
        }
        // PBR PASS
        let shader = &mut self.pbr_shader;
        shader.activate();
        self.passes.bind_pbr(shader);
        shader.uniform3f("camera_pos", [cp[0], cp[1], cp[2]]);
        shader.uniform1ui("n_lights", light_info.len() as u32);
        for i in 0..light_info.len() {
            if i >= MAX_LIGHTS {
                error!("Too many lights: {}", light_info.len());
                break;
            }
            let light = &light_info[i];
            let post = &light.0;
            let pos: [f32; 4] = [
                post[0],
                post[1],
                post[2],
                if light.1.directional { 0.0 } else { 1.0 },
            ];
            shader.uniform4f(&format!("light[{}].position", i), pos);
            shader.uniform3f(&format!("light[{}].color", i), light.1.color);
            shader.uniform1f(&format!("light[{}].intensity", i), light.1.intensity);
        }
        self.passes.print_quad();
        // SSR PASS
        let shader = &mut self.ssr_shader;
        shader.activate();
        self.passes.bind_ssr(shader);
        shader.uniform3f("camera_pos", [cp[0], cp[1], cp[2]]);
        shader.uniformMat4f("camera", cm.to_homogeneous().into());
        self.passes.print_quad();
        // SSR-APPLY
        let shader = &mut self.ssr_apply_shader;
        shader.activate();
        self.passes.bind_ssr_apply(shader);
        self.passes.print_quad();
        // FINAL PASS
        self.passes.print_buffer(frame);
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f64();
        self.fps_total += elapsed;
        self.fps.push_back(elapsed);
        self.last_frame_time = now;
        if self.fps.len() > 127 {
            self.fps_total -= self.fps.pop_front().expect("cannot fail");
        }
        self.frame_count += 1;
        if realtime {
            if self.animation_step.len() == 0
                || Instant::now().duration_since(self.start_time).as_millis()
                    >= *self.animation_step.last().unwrap() as u128
            {
                true
            } else {
                false
            }
        } else {
            if self.animation_step.len() == 0 || self.frame_count == self.animation_step.len() {
                self.frame_count = 0;
                true
            } else {
                false
            }
        }
    }

    pub fn get_fps(&self) -> f64 {
        return if self.fps_total == 0.0 {
            0.0
        } else {
            self.fps.len() as f64 / self.fps_total
        };
    }
}

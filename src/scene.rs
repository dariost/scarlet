use crate::have_gl;
use glad_gles2::gl;
use gltf::accessor::DataType;
use gltf::buffer::Source;
use gltf::khr_lights_punctual::Kind;
use gltf::mesh::Semantic;
use na::geometry::{Perspective3, Quaternion, Similarity3, Translation3, UnitQuaternion};
use nalgebra as na;
use std::cell::RefCell;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Scene {
    root: SceneNode,
    camera: SceneNode,
    lights: Vec<SceneNode>,
}

#[derive(Debug)]
pub struct RealSceneNode {
    transform: Similarity3<f32>,
    children: Vec<SceneNode>,
    parent: Option<Weak<RefCell<RealSceneNode>>>,
    name: String,
    camera: Option<Camera>,
    light: Option<Light>,
    mesh: Option<Mesh>,
}

#[derive(Debug)]
pub struct Camera {
    perspective: Perspective3<f32>,
    name: String,
}

#[derive(Debug)]
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
    material: Material,
    buffer: Vec<f32>,
}

#[derive(Debug)]
pub struct Mesh {
    name: String,
    data: Vec<RenderData>,
}

#[derive(Debug, Default)]
pub struct Material {
    color: [f32; 4],
    metallic: f32,
    roughness: f32,
}

type SceneNode = Rc<RefCell<RealSceneNode>>;

impl Default for RealSceneNode {
    fn default() -> Self {
        RealSceneNode {
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
        };
        unsafe {
            gl::GenVertexArrays(1, &mut rd.vao);
            gl::GenBuffers(1, &mut rd.vbo);
        };
        rd
    }
}

pub fn create_mesh(mesh: gltf::Mesh, buffers: &Vec<gltf::buffer::Data>) -> Mesh {
    let name = String::from(mesh.name().unwrap_or("NULL"));
    let mut data = Vec::new();
    for primitive in mesh.primitives() {
        let mut rd = RenderData::new();
        rd.mode = primitive.mode().as_gl_enum();
        let model = primitive.material().pbr_metallic_roughness();
        let material = Material {
            color: model.base_color_factor(),
            metallic: model.metallic_factor(),
            roughness: model.roughness_factor(),
        };
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
                6 * size_of::<gl::GLfloat>() as i32,
                null(),
            );
            gl::VertexAttribPointer(
                1,
                3,
                gl::GL_FLOAT,
                gl::GL_FALSE,
                6 * size_of::<gl::GLfloat>() as i32,
                null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
            );
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        data.push(rd);
    }
    Mesh { name, data }
}

pub fn import_scene(asset: &[u8], aspect_ratio: f32) -> Scene {
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
        buffers: &Vec<gltf::buffer::Data>,
    ) {
        let mut scene_node = RealSceneNode::default();
        scene_node.name = String::from(node.name().unwrap_or("NULL"));
        let (translation, rotation, scaling) = node.transform().decomposed();
        let translation = Translation3::<f32>::new(translation[0], translation[1], translation[2]);
        let rotation = Quaternion::<f32>::new(rotation[0], rotation[1], rotation[2], rotation[3]);
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
                    p.zfar().unwrap_or(10e9),
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
            scene_node.borrow_mut().mesh = Some(create_mesh(mesh, buffers));
        }
        for child in node.children() {
            construct_scene(
                &mut scene_node,
                child,
                &mut camera,
                &mut lights,
                ar,
                buffers,
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
        );
    }
    Scene {
        root: root_node,
        lights: lights,
        camera: camera.expect("There must be a camera in the scene!"),
    }
}

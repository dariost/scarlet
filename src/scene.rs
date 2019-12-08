use gltf::khr_lights_punctual::Kind;
use na::geometry::{Perspective3, Quaternion, Similarity3, Translation3, UnitQuaternion};
use nalgebra as na;
use std::cell::RefCell;
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
        }
    }
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
        for child in node.children() {
            construct_scene(&mut scene_node, child, &mut camera, &mut lights, ar);
        }
        scene_node.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(scene_node);
    }
    root_node.name = String::from("ROOT_NODE");
    let mut root_node = Rc::new(RefCell::new(root_node));
    for node in scene.nodes() {
        construct_scene(&mut root_node, node, &mut camera, &mut lights, aspect_ratio);
    }
    Scene {
        root: root_node,
        lights: lights,
        camera: camera.expect("There must be a camera in the scene!"),
    }
}

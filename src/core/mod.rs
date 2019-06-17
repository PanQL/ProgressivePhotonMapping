pub mod progressive_photon_mapper;
pub mod path_tracer;
pub mod photon_tracer;

pub use progressive_photon_mapper::ProgressivePhotonTracer;
pub use path_tracer::RayTracer;
pub use photon_tracer::PhotonTracer;
use crate::scene::Scene;
use crate::util::*;
use crate::camera::Camera;
use std::vec::Vec;
use std::sync::{ Arc, Mutex, mpsc::channel };
use std::boxed::Box;
use std::thread::spawn;
use kdtree::kdtree::KdTree as Kd;
use kdtree::distance::squared_euclidean;
extern crate rand;
pub use rand::Rng;

struct RenderInner {
    //class : TraceType,  // 渲染算法类型
    camera : Arc<Camera>,   // 相机，只读
    width : usize,
    height : usize,
    scene: Arc<Scene>,  // 场景，只读
    hit_point_map : Arc<Mutex<Kd<f64, Box<ViewPoint>, [f64;3]>>>,
    points : Arc<Vec<Arc<Mutex<ViewPoint>>>>,
    photon_map : Arc<Mutex<Kd<f64, Photon, [f64;3]>>>,   // 光子图
    photon_tracer : Arc<PhotonTracer>,
    ray_tracer : RayTracer,
}

impl RenderInner {
    pub fn new(camera : Camera, scene : Scene) -> Self {
        let scene = Arc::new(scene);
        let width = camera.width;
        let height = camera.height;
        RenderInner{
            //class : TraceType::PPM,
            camera : Arc::new(camera) ,
            width,
            height,
            scene : scene.clone(),
            hit_point_map : Arc::new(Mutex::new(Kd::new(3))),
            points : Arc::new(Vec::new()),
            photon_map : Arc::new(Mutex::new(Kd::new(3))),
            photon_tracer : Arc::new(PhotonTracer::new(scene.clone())),
            ray_tracer : RayTracer::new(scene),
        }
    }

    // 光线追踪阶段
    pub fn ray_tarcing(&self, sampling : usize, picture : &mut Vec<Color>) {
        let f = | collider : &Collider| -> Color{
            let mut ret = Color::default();
            if let Ok(photon_map) = self.photon_map.lock() {
                let mut coord : [f64;3] = [0.0, 0.0, 0.0];
                coord[0] = collider.pos.x;
                coord[1] = collider.pos.y;
                coord[2] = collider.pos.z;
                let result = photon_map.within(&coord, 1.0, &squared_euclidean).unwrap();
                for (_, photon) in result.iter() {
                    ret += photon.power;
                }
                if !result.is_empty() { ret.div(result.len() as f64); }
            }
            ret
        };
        let sampling2 : f64 = (sampling * sampling) as f64;
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.camera.emitting(i, j);
                let dx = ray.d.get_vertical_vec();
                let dy = dx.cross(&ray.d);
                let mut a_vec = Vector3::default();
                for x in 0..sampling {
                    for y in 0..sampling {
                        a_vec = ray.d + dx.mult(x as f64 / sampling as f64 - 0.5).mult(0.001) 
                            + dy.mult(y as f64 / sampling as f64 - 0.5).mult(0.001);
                        let a_ray = Ray { o : ray.o, d : a_vec };
                        picture[j * self.width + i] += self.ray_tracer.trace_ray(&a_ray, 1.0, 0, f).div(sampling2);
                    }
                }
            }
        }
    }

    // 光子发射阶段
    pub fn photon_tracing(&self, photon_number : usize) {
        let f = |photon : Photon|{  // 光子发射阶段，当光子碰撞到漫反射面时，应该执行的操作。
            let mut coord : [f64;3] = [0.0, 0.0, 0.0];
            coord[0] = photon.ray.o.x;
            coord[1] = photon.ray.o.y;
            coord[2] = photon.ray.o.z;
            if let Ok(mut p_map) = self.photon_map.lock() {
                p_map.add(coord, photon).unwrap();
            }
        };
        let number = self.scene.get_light_num();
        for i in 0..number {    // 按照光源顺序不断发射光子
            let illumiant = self.scene.get_light(i);
            for _ in 0..photon_number {
                self.photon_tracer.photon_tracing(illumiant.gen_photon(), 0, f);
            }
        }
    }

}

pub struct Render {
    inner : Arc<RenderInner>,
    picture: Vec<Color>,    // 所有线程结束之后才会写回,无需互斥
    width : usize,
    height : usize,
}

impl Render {
    pub fn new(camera : Camera, scene : Scene) -> Self {
        let width = camera.width;
        let height = camera.height;
        Render { inner : Arc::new(RenderInner::new(camera, scene)), picture : Vec::new(), width, height}
    }
    
    pub fn run(&mut self, times : usize, photon_num : usize, thread_num : usize) {
        self.picture.resize_default(self.width * self.height);
        let mut handle_vec = Vec::new();
        for i in 0..thread_num {
            let inner = self.inner.clone();
            let (sender, receiver) = channel();
            spawn(move ||{
                for j in 0..times / thread_num {
                    info!("{} thread {} round", i, j);
                    inner.photon_tracing(photon_num);
                }
                sender.send(1).unwrap();
            });
            handle_vec.push(receiver);
        }
        for receiver in handle_vec.iter_mut() {
            receiver.recv().unwrap();
        }
        self.inner.ray_tarcing(4, &mut self.picture);
        self.gen_png();
    }

    fn gen_png(&self) {
        let buffer: &mut [u16] = &mut vec![0; 1024 * 768 * 3];
        //将结果写入png
        let width = self.width;
        let height = self.height;
        for i in 0..width {
            for j in 0..height {
                let (r, g, b) = self.picture[j * width + i].to_u16();
                buffer[(j * width + i) * 3] = r;
                buffer[(j * width + i) * 3 + 1] = g;
                buffer[(j * width + i) * 3 + 2] = b;
            }
        }
        unsafe {
            image::save_buffer("result.png", 
                               std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len() * 2),
                               width as u32, height as u32, image::RGB(16)).unwrap()
        }
    }
}

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
use std::cell::RefCell;
use std::sync::Arc;
use kdtree::kdtree::KdTree as Kd;
use kdtree::distance::squared_euclidean;
extern crate rand;
pub use rand::Rng;

pub struct Render {
    //class : TraceType,  // 渲染算法类型
    camera : Arc<Camera>,   // 相机，只读
    picture: Vec<Color>,    // 所有线程结束之后才会写回,无需互斥
    width : usize,
    height : usize,
    scene: Arc<Scene>,  // 场景，只读
    hit_point_map : Arc<RefCell<Kd<f64, Arc<RefCell<ViewPoint>>, [f64;3]>>>,
    points : Arc<RefCell<Vec<Arc<RefCell<ViewPoint>>>>>,
    photon_map : Arc<RefCell<Kd<f64, Photon, [f64;3]>>>,   // 光子图
    photon_tracer : PhotonTracer,
    ray_tracer : RayTracer,
}

impl Render {
    pub fn new(camera : Camera, scene : Scene) -> Self {
        let scene = Arc::new(scene);
        let width = camera.width;
        let height = camera.height;
        Render{
            //class : TraceType::PPM,
            camera : Arc::new(camera) ,
            picture: Vec::new(),
            width,
            height,
            scene : scene.clone(),
            hit_point_map : Arc::new(RefCell::new(Kd::new(3))),
            points : Arc::new(RefCell::new(Vec::new())),
            photon_map : Arc::new(RefCell::new(Kd::new(3))),
            photon_tracer : PhotonTracer::new(scene.clone()),
            ray_tracer : RayTracer::new(scene),
        }
    }

    // 光线追踪阶段
    pub fn ray_tarcing(&mut self, sampling : usize) {
        let photon_map = self.photon_map.borrow();
        let f = | collider : &Collider| -> Color{
            let mut coord : [f64;3] = [0.0, 0.0, 0.0];
            coord[0] = collider.pos.x;
            coord[1] = collider.pos.y;
            coord[2] = collider.pos.z;
            let result = photon_map.within(&coord, 1.0, &squared_euclidean).unwrap();
            let mut ret = Color::default();
            for (_, photon) in result.iter() {
                ret += photon.power;
            }
            if !result.is_empty() { ret.div(result.len() as f64); }
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
                        self.picture[j * self.width + i] += self.ray_tracer.trace_ray(&a_ray, 1.0, 0, f).div(sampling2);
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
            self.photon_map
                .borrow_mut()
                .add(coord, photon)
                .unwrap();
        };
        let number = self.scene.get_light_num();
        for i in 0..number {    // 按照光源顺序不断发射光子
            let illumiant = self.scene.get_light(i);
            for _ in 0..photon_number {
                self.photon_tracer.photon_tracing(illumiant.gen_photon(), 0, f);
            }
        }
    }

    pub fn run(&mut self, times : usize, photon_num : usize) {
        self.picture.resize_default(self.width * self.height);
        for i in 0..times {
            self.photon_tracing(photon_num);
            info!("{}th round", i); 
        }
        self.ray_tarcing(4);
        self.gen_png();
    }

    fn gen_png(&mut self) {
        let buffer: &mut [u16] = &mut vec![0; 1024 * 768 * 3];
        //将结果写入png
        let width = self.camera.width;
        let height = self.camera.height;
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

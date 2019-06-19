use crate::scene::Scene;
use crate::util::*;
use crate::camera::Camera;
use std::vec::Vec;
use std::cell::RefCell;
use std::sync::Arc;
use kdtree::kdtree::KdTree as Kd;
use kdtree::distance::squared_euclidean;
extern crate rand;

use rand::Rng;


enum TraceType {
    PM,PPM
}

pub struct ProgressivePhotonTracer {
    class : TraceType,  // 渲染算法类型
    camera : Arc<Camera>,   // 相机，只读
    picture: Vec<Color>,    // 所有线程结束之后才会写回,无需互斥
    width: usize,
    height: usize,
    scene: Arc<Scene>,  // 场景，只读
    hit_point_map : Kd<f64, Arc<RefCell<ViewPoint>>, [f64;3]>,
    points : Vec<Arc<RefCell<ViewPoint>>>,
    photon_map : Kd<f64, Photon, [f64;3]>,   // 光子图
    total_photon : f64, // 发射的总光子数量
    max_radius : f64,
}

impl ProgressivePhotonTracer {
    pub fn new(camera : Arc<Camera>, scene : Arc<Scene>) -> Self {
        ProgressivePhotonTracer{
            class : TraceType::PPM,
            camera,
            picture: Vec::new(),
            width: 0,
            height: 0,
            scene,
            hit_point_map : Kd::new(3),
            points : Vec::new(),
            photon_map : Kd::new(3),
            total_photon : 0.0,
            max_radius : 0.0,
        }
    }

    pub fn ray_tracing_pass(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.camera.emitting(i, j);
                self.trace_ray(&ray, j, i, 1.0);
            }
        }
    }

    pub fn trace_ray(&mut self, ray: &Ray, pixel_x: usize, pixel_y: usize, weight : f64) {
        if let Some(collider) = self.scene.intersect(ray) {
            if collider.material.is_diffuse() {
                let vp = Arc::new(RefCell::new(
                    ViewPoint::new(&collider,  pixel_x, pixel_y, weight * collider.material.diffuse)
                ));
                let mut coord : [f64;3] = [0.0, 0.0, 0.0];
                coord[0] = collider.pos.x;
                coord[1] = collider.pos.y;
                coord[2] = collider.pos.z;
                self.hit_point_map.add(coord, vp.clone()).unwrap();
                self.points.push(vp);
            }
            if collider.material.is_specular() {
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                self.trace_ray(&spec_ray, pixel_x, pixel_y, weight * collider.material.specular);
            }
        }
    }

    pub fn run(&mut self, times: usize) {
        self.width = self.camera.width;
        self.height = self.camera.height;
        self.picture.resize((self.width * self.height) as usize, Color::default());

        self.ray_tracing_pass(); // 从眼睛发射光线
        
        self.cal_hp_radius();
        self.class = TraceType::PPM;

        for i in 0..times {
            self.photon_tracing_pass(100_0000);
            self.total_photon += 100_0000.0;
            self.renew_hp_map();
            info!("{} rounds, {} photons ", i, self.total_photon);
        }
        

        self.gen_png();
    }

    fn photon_tracing(&mut self, mut photon : Photon, depth : u32) {
        if depth > 10 { return; }   // 最大递归深度
        if let Some(collider) = self.scene.intersect(&photon.ray) {
            photon.ray.o = collider.pos;
            if collider.material.is_diffuse() {    // 到达漫反射平面
                let mut new_photon = photon.clone();
                new_photon.ray.d = photon.ray.d.mult(-1.0); // 方向设置为指向光源的方向
                match &self.class {
                    TraceType::PM => {
                        let x : f64 = collider.pos.x.clone();
                        let y : f64 = collider.pos.y.clone();
                        let z : f64 = collider.pos.z.clone();
                        let coord : [f64;3] = [x, y, z];
                        self.photon_map.add(coord, new_photon).unwrap();
                    }
                    TraceType::PPM => {
                        self.insert_photon(&new_photon);    // 计算该光子对碰撞点的影响
                    }
                }
            }

            let mut prob = 1.0;
            if !self.photon_diffusion(&collider, photon.clone(), depth, &mut prob) {
                if !self.photon_reflection(&collider, photon.clone(), depth, &mut prob) {
                }
            }
        }
    }

    fn photon_reflection(&mut self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64) -> bool {
        let eta = collider.material.specular * collider.material.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(spec_ray) = collider.get_specular_ray() {
            photon.ray.d = spec_ray;
            photon.power = photon.power * collider.material.color.refresh_by_power();
            self.photon_tracing(photon, depth + 1);
        }
        return true;
    }

    fn photon_diffusion(&mut self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64) -> bool {
        let eta = collider.material.diffuse * collider.material.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(diff_ray) = collider.get_diffuse_ray() {
            photon.ray.d = diff_ray;
            photon.power = photon.power * collider.material.color.refresh_by_power();
            self.photon_tracing(photon, depth + 1);
        }
        return true;
    }

    fn photon_tracing_pass(&mut self, photon_number : usize) {
        let number = self.scene.get_light_num();
        for i in 0..number {
            let illumiant = self.scene.get_light(i);
            for _ in 0..photon_number {
                let photon = illumiant.gen_photon();
                self.photon_tracing(photon, 0);
            }
        }
    }

    fn cal_hp_radius(&mut self ) {  // TODO， 可以根据视点的分布范围来估计半径
        let mut max = Vector3::new(-1e20, -1e20, -1e20);
        let mut min = Vector3::new(1e20, 1e20, 1e20);
        for vp_ptr in self.points.iter() {
            let mut vp = vp_ptr.borrow_mut();
            if vp.pos.x > max.x { max.x = vp.pos.x } ;
            if vp.pos.y > max.y { max.y = vp.pos.y } ;
            if vp.pos.z > max.z { max.z = vp.pos.z } ;
            if vp.pos.x < min.x { min.x = vp.pos.x } ;
            if vp.pos.y < min.y { min.y = vp.pos.y } ;
            if vp.pos.z < min.z { min.z = vp.pos.z } ;
        }
        info!("{:?}, {:?}", max, min);
        let irad = (((max.x - min.x) + (max.y - min.y) + (max.z - min.z)) / 3.0) / ((self.width + self.height) as f64 / 2.0) * 2.0;
        for vp_ptr in self.points.iter() {
            let mut vp = vp_ptr.borrow_mut();
            vp.radius2 = irad * irad;
        }
        self.max_radius = irad * irad;
    }

    fn gen_png(&mut self) {
        let buffer: &mut [u16] = &mut vec![0; 1024 * 768 * 3];

        for vp_ptr in self.points.iter() {
            let vp = vp_ptr.borrow();
            let to_div = std::f64::consts::PI * self.total_photon * vp.radius2;
            self.picture[vp.x * self.width + vp.y] += vp.flux_color.div(to_div) * vp.color.mult(vp.wgt); //TODO !!!
        }

        //将结果写入png
        for i in 0..self.width {
            for j in 0..self.height {
                let (r, g, b) = self.picture[j * self.width + i].to_u16();
                buffer[(j * self.width + i) * 3] = r;
                buffer[(j * self.width + i) * 3 + 1] = g;
                buffer[(j * self.width + i) * 3 + 2] = b;
            }
        }
        unsafe {
            image::save_buffer("result.png", 
                               std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len() * 2),
                               self.width as u32, self.height as u32, image::RGB(16)).unwrap()
        }
    }

    fn renew_hp_map(&mut self) {
        let mut irad = 1e-20;
        for vp_ptr in self.points.iter() {
            let vp = vp_ptr.borrow_mut();
            if vp.radius2 > irad { irad = vp.radius2; }
        }
        self.max_radius = irad;
        info!("max radius2 is {}", irad);
    }

    fn insert_photon(&self, photon : &Photon) {
        let mut coord : [f64;3] = [0.0, 0.0, 0.0];
        coord[0] = photon.ray.o.x;
        coord[1] = photon.ray.o.y;
        coord[2] = photon.ray.o.z;
        let result = self.hit_point_map.within(&coord, self.max_radius, &squared_euclidean).unwrap();
        for (_, vp) in result.iter() {
            vp.borrow_mut().handle(photon);
        }
    }
}

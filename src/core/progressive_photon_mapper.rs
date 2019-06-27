use crate::scene::Scene;
use crate::util::*;
use crate::camera::Camera;
use crate::consts::EPS;
use std::vec::Vec;
use std::cell::RefCell;
use std::sync::{ Arc, mpsc::channel };
use std::path::Path;
use kdtree::kdtree::KdTree as Kd;
use kdtree::distance::squared_euclidean;
use rand::Rng;
use super::PhotonTracer;
use std::thread::spawn;
use spin::Mutex;


pub struct ProgressivePhotonTracer {
    camera : Arc<Camera>,   // 相机，只读
    picture: Vec<Color>,    // 所有线程结束之后才会写回,无需互斥
    width: usize,
    height: usize,
    scene: Arc<Scene>,  // 场景，只读
    hit_point_map : Arc<Kd<f64, Arc<Mutex<ViewPoint>>, [f64;3]>>,
    points : Vec<Arc<Mutex<ViewPoint>>>,
    photon_map : Kd<f64, Photon, [f64;3]>,   // 光子图
    total_photon : f64, // 发射的总光子数量
    max_radius : f64,
    hash_table : Vec<u64>,
    super_sampled : Vec<bool>,
}

impl ProgressivePhotonTracer {
    pub fn new(camera : Arc<Camera>, scene : Arc<Scene>) -> Self {
        ProgressivePhotonTracer{
            camera,
            picture: Vec::new(),
            width: 0,
            height: 0,
            scene,
            hit_point_map : Arc::new(Kd::new(3)),
            points : Vec::new(),
            photon_map : Kd::new(3),
            total_photon : 0.0,
            max_radius : 0.0,
            hash_table : Vec::new(),
            super_sampled : Vec::new(),
        }
    }

    pub fn ray_tracing_pass(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.camera.emitting(i, j);
                let mut hash = 0u64;
                let idx = j * self.width +  i;
                self.trace_ray(&ray, idx, 1.0, 0, false, &mut hash);
                self.hash_table[idx] = hash;
            }
        }
        for i in 0..self.width {
            for j in 0..self.height {
                if self.judge_hash(i, j) {
                    let idx = j * self.width + i;
                    let mut hash = 0u64;
                    self.super_sampled[idx] = true;
                    for ii in 0..9 {
                        let ray = self.camera.super_emitting(
                            i, j, (ii % 3) as f64 / 3.0 - 1.0 / 3.0, (ii / 3) as f64 / 3.0 - 1.0 / 3.0);
                        self.trace_ray(&ray, idx, 1.0, 0, false, &mut hash);
                    }
                }
            }
        }
    }

    fn judge_hash(&self, x : usize, y : usize) -> bool {
        if x != 0 && self.hash_table[y * self.width + x] != self.hash_table[y * self.width + x - 1] {
            return true;
        }
        if x != self.width - 1 && self.hash_table[y * self.width + x] != self.hash_table[y * self.width + x + 1] {
            return true;
        }
        if y != 0 && self.hash_table[y * self.width + x] != self.hash_table[(y - 1) * self.width + x] {
            return true;
        }
        if y != self.height - 1 && self.hash_table[y * self.width + x] != self.hash_table[(y + 1) * self.width + x] {
            return true;
        }
        false
    }

    pub fn trace_ray(&mut self, ray: &Ray, pixel_pos: usize, weight : f64, depth : u32, refracted : bool, hash : &mut u64) {
        if depth > 20 { return; }
        let lgt_collider = self.scene.intersect_light(ray);
        let obj_collider = self.scene.intersect(ray);
        if obj_collider.is_some() { // 与物体相交
            let collider = obj_collider.unwrap();
            if lgt_collider.is_some() {
                let lgt = lgt_collider.unwrap();
                if (collider.distance - lgt.dist) < EPS { // 光源的交点更近
                    self.picture[pixel_pos] = lgt.power.mult(0.7);
                    return;
                }
            }
            if collider.material.is_diffuse() {
                *hash = *hash * 13 + collider.get_hash();
                let vp = ViewPoint::new(&collider,  pixel_pos, weight * collider.material.diffuse);
                let vp_ptr = Arc::new(Mutex::new(vp));
                let mut coord : [f64;3] = [0.0, 0.0, 0.0];
                coord[0] = collider.pos.x;
                coord[1] = collider.pos.y;
                coord[2] = collider.pos.z;
                Arc::get_mut(&mut self.hit_point_map)
                    .unwrap()
                    .add(coord, vp_ptr.clone())
                    .unwrap();
                self.points.push(vp_ptr);
            }
            if collider.material.is_specular() {
                *hash = *hash * 17 + collider.get_hash();
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                self.trace_ray(&spec_ray, pixel_pos, weight * collider.material.specular, depth + 1, refracted, hash);
            }
            if collider.material.is_refractive() {
                *hash = *hash * 19 + collider.get_hash();
                if let Some(dir) = collider.material.cal_refractive_ray(&ray.d, &collider.norm_vec, refracted) {
                    let spec_ray = Ray::new(
                        collider.pos,
                        dir
                    );
                    self.trace_ray(&spec_ray, pixel_pos, weight * collider.material.refraction, depth + 1, !refracted, hash);
                }
            }
        } else if lgt_collider.is_some() {  // 只与光源相交
            let lgt = lgt_collider.unwrap();
            self.picture[pixel_pos] = lgt.power.mult(0.7);
        }
    }

    pub fn run(&mut self, times: usize, threads : usize) {
        self.width = self.camera.width;
        self.height = self.camera.height;
        self.picture.resize((self.width * self.height) as usize, Color::default());
        self.hash_table.resize((self.width * self.height) as usize, 0u64);
        self.super_sampled.resize((self.width * self.height) as usize, false);

        self.ray_tracing_pass(); // 从眼睛发射光线

        info!("sampling over!");
        
        self.cal_hp_radius();

        for i in 0..times {
            self.photon_tracing_pass(10_0000, threads);
            self.total_photon += 10_0000.0;
            self.renew_hp_map();
            info!("{} rounds, {} photons ", i, self.total_photon);
        }
        

        self.gen_png();
    }

    fn photon_tracing_pass(&mut self, photon_number : usize, threads : usize) {
        let mut handle_vec = Vec::new();
        for _ in 0..threads {
            let photon_tracer = PhotonTracer::new(
                self.scene.clone(), self.hit_point_map.clone(), self.max_radius);
            let (sender, receiver) = channel();
            spawn(move ||{
                photon_tracer.photon_tracing_pass(photon_number / threads, photon_number as f64);
                sender.send(1).unwrap();
            });
            handle_vec.push(receiver);
        }
        for receiver in handle_vec.iter_mut() {
            receiver.recv().unwrap();
        }
    }

    fn cal_hp_radius(&mut self ) {  // TODO， 可以根据视点的分布范围来估计半径
        let mut max = Vector3::new(-1e20, -1e20, -1e20);
        let mut min = Vector3::new(1e20, 1e20, 1e20);
        for vp_ptr in self.points.iter() {
            let vp = vp_ptr.lock();
            max.x = max.x.max(vp.pos.x);
            max.y = max.y.max(vp.pos.y);
            max.z = max.z.max(vp.pos.z);
            min.x = min.x.min(vp.pos.x);
            min.y = min.y.min(vp.pos.y);
            min.z = min.z.min(vp.pos.z);
        }
        info!("{:?}, {:?}", max, min);
        let irad = (((max.x - min.x) + (max.y - min.y) + (max.z - min.z)) / 3.0) / ((self.width + self.height) as f64 / 2.0) * 2.0;
        for vp_ptr in self.points.iter_mut() {
            //vp.radius2 = irad * irad;
            let mut vp =  vp_ptr.lock();
            vp.radius2 = irad * irad;
        }
        self.max_radius = irad * irad;
    }

    fn gen_png(&mut self) {
        let buffer: &mut [u8] = &mut vec![0; self.width * self.height * 3];
        for vp_ptr in self.points.iter() {
            let vp = vp_ptr.lock();
            let to_div = std::f64::consts::PI * self.total_photon * vp.radius2;
            self.picture[vp.px_pos] += vp.flux_color.div(to_div); 
        }

        //将结果写入png
        for i in 0..self.width {
            for j in 0..self.height {
                let idx = j * self.width + i;
                let mut res = self.picture[idx];
                if self.super_sampled[idx] {
                    res = res.mult(0.1);
                }
                let (r, g, b) = res.to_u8();
                buffer[idx * 3] = r;
                buffer[idx * 3 + 1] = g;
                buffer[idx * 3 + 2] = b;
            }
        }
        let path = &Path::new("result.png");
        if let Err(_e) = lodepng::encode_file(path, buffer, self.width, self.height, lodepng::ColorType::RGB, 8) {
            panic!("encode error! {} ", _e);
        }
    }

    fn renew_hp_map(&mut self) {
        let mut irad = 1e-20;
        for vp_ptr in self.points.iter_mut() {
            let mut vp = vp_ptr.lock();
            vp.renew();
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
        for (_, vp_ptr) in result.iter() {
            let mut vp = vp_ptr.lock();
            vp.handle(photon);
        }
    }
}

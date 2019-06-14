use super::scene::Scene;
use super::util::*;
use super::camera::Camera;
use std::vec::Vec;
extern crate rand;

use rand::Rng;

pub struct ProgressivePhotonTracer {
    camera : Camera,
    picture: Vec<Color>,
    width: usize,
    height: usize,
    scene: Scene,
    hit_point_map : KdTree,
    points : Vec<ViewPoint>,
    emitted_photon : f64,
}

impl ProgressivePhotonTracer {
    pub fn new(camera : Camera, scene : Scene) -> Self {
        ProgressivePhotonTracer{
            camera ,
            picture: Vec::new(),
            width: 0,
            height: 0,
            scene,
            hit_point_map : KdTree::new(),
            points : Vec::new(),
            emitted_photon : 0.0,
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
                self.points.push(
                    ViewPoint::new(&collider,  pixel_x, pixel_y, weight)   // TODO corrent weight
                );
            }
            if collider.material.is_specular() {
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                self.trace_ray(&spec_ray, pixel_x, pixel_y, weight); // TODO correct weight
            }
        }
    }

    pub fn run(&mut self, times: usize) {
        let buffer: &mut [u16] = &mut vec![0; 1024 * 768 * 3];
        self.width = self.camera.width;
        self.height = self.camera.height;
        self.picture.resize((self.width * self.height) as usize, Color::default());

        let mut total_photon : f64 = 0.0;
        self.ray_tracing_pass(); // 从眼睛发射光线
        self.hit_point_map.build(&mut self.points, 0); // 构建视点树
        for i in 0..times {
            self.photon_tracing_pass(10_0000);
            total_photon += 100000.0;
            self.hit_point_map.renew();
            info!("{} rounds", i);
        }
        self.hit_point_map.setup_pixel(&mut self.picture, self.width, total_photon);

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

    fn photon_tracing(&mut self, mut photon : Photon, depth : u32) {
        if depth > 10 { return; }   // 最大递归深度
        if let Some(collider) = self.scene.intersect(&photon.ray) {
            photon.ray.o = collider.pos;
            if collider.material.is_diffuse() && depth > 0{    // 到达漫反射平面
                let mut new_photon = photon.clone();
                new_photon.ray.d = photon.ray.d.mult(-1.0); // 方向设置为指向光源的方向
                self.hit_point_map.walk_photon(&new_photon);   // 计算该光子对碰撞点的影响
                return;
            }

            let mut prob = 2.0;
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
                self.photon_tracing(illumiant.gen_photon(), 0);
            }
        }
    }
}

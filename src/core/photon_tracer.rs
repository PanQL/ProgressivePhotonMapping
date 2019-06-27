use crate::scene::Scene;
use crate::util::*;
use std::sync::Arc;
use kdtree::kdtree::KdTree as Kd;
use kdtree::distance::squared_euclidean;
use spin::Mutex;

use rand::Rng;

pub struct PhotonTracer {
    scene : Arc<Scene>,
    hit_point_map : Arc<Kd<f64, Arc<Mutex<ViewPoint>>, [f64;3]>>,
    max_radius : f64,
}

impl PhotonTracer {
    fn photon_tracing(&self, mut photon : Photon, depth : u32, refracted : bool) {
        if depth > 10 || photon.power.power() < 1e-7 { return; }   // 最大递归深度
        if let Some(collider) = self.scene.intersect(&photon.ray) {
            photon.ray.o = collider.pos;
            if collider.material.is_diffuse() {    // 到达漫反射平面
                let mut new_photon = photon.clone();
                new_photon.ray.d = photon.ray.d.mult(-1.0); // 方向设置为指向光源的方向
                self.insert_photon(&new_photon);    // 计算该光子对碰撞点的影响
            }

            let mut prob = 1.0;
            if !self.photon_diffusion(&collider, photon.clone(), depth, &mut prob, refracted) {
                if !self.photon_reflection(&collider, photon.clone(), depth, &mut prob, refracted) {
                    self.photon_refraction(&collider, photon.clone(), depth, &mut prob, refracted);
                }
            }
        }
    }

    fn photon_reflection(&self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64, refracted : bool) -> bool {
        let eta = collider.material.specular * collider.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(spec_ray) = collider.get_specular_ray() {
            photon.ray.d = spec_ray;
            photon.power = photon.power * collider.color.refresh_by_power();
            self.photon_tracing(photon, depth + 1,refracted);
        }
        return true;
    }

    fn photon_diffusion(&self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64, refracted : bool) -> bool {
        let eta = collider.material.diffuse * collider.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(diff_ray) = collider.get_diffuse_ray() {
            photon.ray.d = diff_ray;
            photon.power = photon.power * collider.color.refresh_by_power();
            self.photon_tracing(photon, depth + 1,refracted);
        }
        return true;
    }

    fn photon_refraction(&self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64, refracted : bool) -> bool {
        let eta = collider.material.refraction * collider.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(refr_ray) = collider.get_refractive_ray(refracted) {
            photon.ray.d = refr_ray;
            photon.power = photon.power * collider.color.refresh_by_power();
            self.photon_tracing(photon, depth + 1, !refracted);
        }
        return true;
    }

    fn insert_photon(&self, photon : &Photon) {
        let mut coord : [f64;3] = [0.0, 0.0, 0.0];
        coord[0] = photon.ray.o.x;
        coord[1] = photon.ray.o.y;
        coord[2] = photon.ray.o.z;
        // TODO mutex
        let result = self.hit_point_map.within(&coord, self.max_radius, &squared_euclidean).unwrap();
        for (_, vp_ptr) in result.iter() {
            let mut vp = vp_ptr.lock();
            vp.handle(photon);
        }
    }

    pub fn photon_tracing_pass(&self, photon_number : usize, power : f64) {
        let number = self.scene.get_light_num();
        for i in 0..number {
            let illumiant = self.scene.get_light(i);
            for _ in 0..photon_number {
                let mut photon = illumiant.gen_photon();
                photon.power = photon.power.mult(power);
                self.photon_tracing(photon, 0, false);
            }
        }
    }

    pub fn new(scene : Arc<Scene>, hit_point_map : Arc<Kd<f64, Arc<Mutex<ViewPoint>>, [f64;3]>>, max_radius : f64) -> Self {
        PhotonTracer { scene, hit_point_map, max_radius }
    }
}

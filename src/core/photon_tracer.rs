use crate::scene::Scene;
use crate::util::*;
use std::sync::Arc;
extern crate rand;

use rand::Rng;

pub struct PhotonTracer {
    scene : Arc<Scene>,
}

impl PhotonTracer {
    pub fn photon_tracing<F : Copy>(&self, mut photon : Photon, depth : u32, mut func : F) where F : FnMut(Photon) {
        if depth > 10 { return; }   // 最大递归深度
        if let Some(collider) = self.scene.intersect(&photon.ray) {
            photon.ray.o = collider.pos;
            if collider.material.is_diffuse() {    // 到达漫反射平面
                let mut new_photon = photon.clone();
                new_photon.ray.d = photon.ray.d.mult(-1.0); // 方向设置为指向光源的方向
                func(new_photon);
            }

            let mut prob = 1.0;
            if self.photon_diffusion(&collider, photon.clone(), depth, &mut prob, func) {
                return;
            }
            if self.photon_reflection(&collider, photon.clone(), depth, &mut prob, func) {
                return;
            }
        }
    }

    fn photon_reflection<F : Copy>(&self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64, func : F) -> bool 
        where F : FnMut(Photon) {
        let eta = collider.material.specular * collider.material.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(spec_ray) = collider.get_specular_ray() {
            photon.ray.d = spec_ray;
            photon.power = photon.power * collider.material.color;
            self.photon_tracing(photon, depth + 1, func);
        }
        return true;
    }

    fn photon_diffusion<F : Copy>(&self, collider : &Collider, mut photon : Photon, depth : u32, prob : &mut f64, func : F) -> bool
        where F : FnMut(Photon) {
        let eta = collider.material.diffuse * collider.material.color.power();
        if eta < rand::thread_rng().gen_range(0.0, 1.0) * ( *prob) {
            *prob -= eta;
            return false;
        }

        if let Some(diff_ray) = collider.get_diffuse_ray() {
            photon.ray.d = diff_ray;
            photon.power = photon.power * collider.material.color.norm_max();
            self.photon_tracing(photon, depth + 1, func);
        }
        return true;
    }

    pub fn new(scene : Arc<Scene>) -> Self {
        PhotonTracer { scene }
    }
}

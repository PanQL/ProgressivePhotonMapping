use crate::scene::Scene;
use crate::util::*;
use std::sync::Arc;

pub struct PathTracer {
    scene : Arc<Scene>,
}

impl PathTracer {
    pub fn new(scene : Arc<Scene>) -> Self {
        PathTracer { scene ,}
    }

    pub fn trace_ray(&self, ray: &Ray, weight : f64, depth : u32) -> Color {
        let mut ret = Color::default();
        if depth > 20 { 
            return ret; 
        }
        let mut dist = 1e20;
        let mut light_power = Color::default();
        if let Some(collider) = self.scene.intersect_light(ray) {
            dist = collider.dist;
            light_power = collider.power.mult(weight);
        }
        if let Some(collider) = self.scene.intersect(ray) {
            if collider.distance > dist { 
                ret += light_power;
                return ret; 
            }
            if collider.material.is_diffuse() {
                let diff_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_diffuse_ray(&collider.norm_vec).unwrap()
                );
                ret += self.trace_ray(&diff_ray, weight * collider.material.diffuse, depth + 1) * collider.color; // TODO correct weight
            }
            if collider.material.is_specular() {
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                ret += self.trace_ray(&spec_ray, weight * collider.material.specular, depth + 1); // TODO correct weight
            }
        }
        ret
    }
}

pub struct RayTracer {
    scene : Arc<Scene>,
}

impl RayTracer {
    pub fn new(scene : Arc<Scene>) -> Self {
        RayTracer { scene ,}
    }

    pub fn trace_ray<F : Copy>(&self, ray: &Ray, weight : f64, depth : u32, mut func : F) -> Color where F : FnMut(&Collider) -> Color {
        let mut ret = Color::default();
        if let Some(collider) = self.scene.intersect(ray) {
            if collider.material.is_diffuse() {
                ret += func(&collider) * collider.color.mult(weight);
            }
            if collider.material.is_specular() {
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                ret += self.trace_ray(&spec_ray, weight * collider.material.specular, depth + 1, func); // TODO correct weight
            }
        }
        ret
    }
}

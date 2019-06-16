use crate::scene::Scene;
use crate::util::*;
use std::sync::Arc;

pub struct PathTracer {
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
                ret += func(&collider) * collider.material.color.mult(weight);
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

mod light;
pub mod material;
pub mod primitive;

use self::light::*;
use self::material::Material;
use self::primitive::*;
pub use super::util::*;
use std::boxed::Box;
use std::sync::Arc;

pub struct Scene {
    objects: Vec<Box<dyn Primitive + Send + Sync>>, // 代表场景中的各个物体
    illumiants: Vec<Arc<dyn Light + Send + Sync>>,  // 代表场景中的各个光源
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            illumiants: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.objects.push(Box::new(Plane::new(
            // Left
            0,
            Vector3::new(0.0, 1.0, 0.0),
            5500.0,
            Arc::new(Material::new(
                Color::new(0.75, 0.25, 0.25),
                1.0,
                0.0,
                0.0,
                2.0,
            )),
            None,
        )));
        self.objects.push(Box::new(Plane::new(
            // Right
            1,
            Vector3::new(0.0, 1.0, 0.0),
            4500.0,
            Arc::new(Material::new(
                Color::new(0.25, 0.25, 0.75),
                1.0,
                0.0,
                0.0,
                2.0,
            )),
            None,
        )));
        self.objects.push(Box::new(Plane::new(
            // Top
            2,
            Vector3::new(0.0, 0.0, 1.0),
            800.0,
            Arc::new(Material::new(
                Color::new(0.99, 0.99, 0.99),
                0.1,
                0.9,
                0.0,
                2.0,
            )),
            None,
        )));
        self.objects.push(Box::new(Plane::new(
            // Bottom
            3,
            Vector3::new(0.0, 0.0, 1.0),
            100.0,
            Arc::new(Material::new(
                Color::new(0.75, 0.75, 0.75),
                1.0,
                0.0,
                0.0,
                2.0,
            )),
            Some("floor.png"),
        )));
        self.objects.push(Box::new(Plane::new(
            //Back
            4,
            Vector3::new(1.0, 0.0, 0.0),
            4500.0,
            Arc::new(Material::new(
                Color::new(0.75, 0.75, 0.75),
                1.0,
                0.0,
                0.0,
                2.0,
            )),
            None,
        )));
        self.objects.push(Box::new(Plane::new(
            //Front
            5,
            Vector3::new(1.0, 0.0, 0.0),
            6100.0,
            Arc::new(Material::new(
                Color::new(0.75, 0.75, 0.75),
                1.0,
                0.0,
                0.0,
                2.0,
            )),
            None,
        )));
        //self.objects.push(Box::new(Sphere::new(
        //6,
        //120.0,
        //Vector3::new(5000.0, 5000.0, 300.0),
        //Arc::new(Material::new(Color::new(0.75, 0.25, 0.25), 1.0, 0.0, 0.0, 1.2)),
        //)));
        //self.objects.push(Box::new(Sphere::new(
        //7,
        //100.0,
        //Vector3::new(5100.0, 5300.0, 200.0),
        //Arc::new(Material::new(Color::new(0.99, 0.99, 0.99), 0.2, 0.0, 0.8, 1.3)),
        //)));
        //self.objects.push(Box::new(Sphere::new(
        //8,
        //100.0,
        //Vector3::new(5100.0, 4700.0, 200.0),
        //Arc::new(Material::new(Color::new(0.25, 0.75, 0.25), 0.2, 0.8, 0.0, 1.3)),
        //)));
        self.objects.push(Box::new(BazierCurve::new(
            9,
            Vector3::new(5000.0, 5000.0, 200.0),
            Arc::new(Material::new(
                Color::new(0.25, 0.75, 0.25),
                1.0,
                0.0,
                0.0,
                1.3,
            )),
        )));
        // 设置光源
        self.illumiants.push(Arc::new(AreaLight::new(
            Vector3::new(4900.0, 4900.0, 800.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Color::new(25.0, 25.0, 25.0),
            200.0,
            200.0,
        )));
    }

    // 求给定射线在场景中的碰撞点
    pub fn intersect(&self, ray: &Ray) -> Option<Collider> {
        let inf: f64 = 1e20;
        let mut t: f64 = 1e20;
        let mut id: usize = 0;
        for i in 0..self.objects.len() {
            if let Some(d) = self.objects[i].intersect(ray) {
                if d < t {
                    t = d;
                    id = i;
                }
            }
        }
        if t < inf {
            let position = ray.o + ray.d.mult(t);
            let mut norm_vec = self.objects[id].get_normal_vec(&position);
            if norm_vec.dot(&ray.d) > 0.0 {
                norm_vec = norm_vec.mult(-1.0);
            }
            return Some(Collider {
                pos: position,
                material: self.objects[id].get_material(),
                norm_vec,
                distance: t,
                in_direction: ray.d,
                hash_value: self.objects[id].get_hash(),
                color: self.objects[id].get_color(&position),
            });
        } else {
            return None;
        }
    }

    pub fn intersect_light(&self, ray: &Ray) -> Option<LightCollider> {
        let inf: f64 = 1e20;
        let mut t: f64 = 1e20;
        let mut id: usize = 0;
        for i in 0..self.illumiants.len() {
            if let Some(d) = self.illumiants[i].intersect(ray) {
                if d < t {
                    t = d;
                    id = i;
                }
            }
        }
        if t < inf {
            return Some(LightCollider {
                power: self.illumiants[id].get_power(),
                dist: t,
            });
        } else {
            return None;
        }
    }

    pub fn get_light_num(&self) -> usize {
        self.illumiants.len()
    }

    pub fn get_light(&self, idx: usize) -> Arc<dyn Light> {
        self.illumiants[idx].clone()
    }
}

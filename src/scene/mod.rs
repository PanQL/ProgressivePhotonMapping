mod primitive;
mod material;

use std::boxed::Box;
pub use super::util::*;
use self::primitive::*;
use self::material::Material;

pub struct Scene {
    objects : Vec<Box<dyn Primitive>>,  // 代表场景中的各个物体
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects : Vec::new() , }
    }

    pub fn init(&mut self) {
        self.objects.push(Box::new(Plane::new(   //Left
            Vector3::new(0.0, 1.0, 0.0), 
            0.0, 
            Material::new(Vector3::new(0.0, 0.0, 0.2), 1.0, 1.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   //Right
            Vector3::new(0.0, 1.0, 0.0), 
            10000.0, 
            Material::new(Vector3::new(0.0, 0.0, 0.2), 1.0, 1.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Bottom
            Vector3::new(0.0, 0.0, 1.0), 
            100.0, 
            Material::new(Vector3::new(0.5, 0.5, 0.5), 1.0, 1.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Top
            Vector3::new(0.0, 0.0, 1.0), 
            10000.0, 
            Material::new(Vector3::new(0.5, 0.5, 0.5), 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(  //Back
            Vector3::new(1.0, 0.0, 0.0), 
            100.0, 
            Material::new(Vector3::new(1.0, 0.0, 0.0), 1.0, 1.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Front
            Vector3::new(1.0, 0.0, 0.0), 
            250000.0, 
            Material::new(Vector3::new(1.0, 0.0, 0.0), 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Sphere::new(
            1500.0,
            Vector3::new(800.0, 7000.0, 2000.0), 
            Material::new(Vector3::new(0.1, 0.4, 0.3), 1.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Sphere::new(
            1000.0,
            Vector3::new(500.0, 5000.0, 6000.0), 
            Material::new(Vector3::new(0.0, 1.0, 0.0), 0.0, 1.0, 0.0)
        )));
    }

    // 求给定射线在场景中的碰撞点
    fn intersect(&self, r : &Ray, t : &mut f64, id : &mut usize) -> bool {
        let inf : f64 = 1e20;
        *t = 1e20;
        for i in 0..self.objects.len() {
            if let Some(d) = self.objects[i].intersect(r) {
                if d < *t {
                    *t = d;
                    *id = i;
                }
            }
        }
        *t < inf
    }
}

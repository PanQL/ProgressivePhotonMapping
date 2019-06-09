mod primitive;
mod light;
mod material;

use std::boxed::Box;
pub use super::util::*;
use self::primitive::*;
use self::material::Material;

pub struct Scene {
    objects : Vec<Box<dyn Primitive>>,  // 代表场景中的各个物体
    points: Vec<ViewPoint>,
    view_tree: KdTree,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: Vec::new(), points: Vec::new(), view_tree: KdTree::new() }
    }

    pub fn init(&mut self) {
        self.objects.push(Box::new(Plane::new(   //Left
            Vector3::new(0.0, 1.0, 0.0),
            0.0,
            Material::new(Color::new(0.0, 0.0, 0.2), 1.0, 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   //Right
            Vector3::new(0.0, 1.0, 0.0),
            10000.0,
            Material::new(Color::new(0.0, 0.0, 0.2), 1.0, 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Bottom
            Vector3::new(0.0, 0.0, 1.0),
            100.0,
            Material::new(Color::new(0.0, 0.3, 0.0), 1.0, 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Top
            Vector3::new(0.0, 0.0, 1.0),
            10000.0,
            Material::new(Color::new(0.5, 0.5, 0.5), 0.0, 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(  //Back
            Vector3::new(1.0, 0.0, 0.0),
            100.0,
            Material::new(Color::new(0.5, 0.0, 0.0), 1.0, 0.0, 0.0, 0.0)
        )));
        self.objects.push(Box::new(Plane::new(   // Front
            Vector3::new(1.0, 0.0, 0.0),
            20000.0,
            Material::new(Color::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0)
        )));
//        self.objects.push(Box::new(Sphere::new(
//            1500.0,
//            Vector3::new(800.0, 7000.0, 2000.0),
//            Material::new(Color::new(0.1, 0.4, 0.3), 1.0, 1.0, 0.0)
//        )));
        //self.objects.push(Box::new(Sphere::new(
            //2000.0,
            //Vector3::new(500.0, 5000.0, 2200.0),
            //Material::new(Color::new(0.0, 1.0, 0.0), 0.0, 1.0, 0.0, 0.0),
        //)));
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

    // 光线追踪阶段
    pub fn trace_ray(&mut self, ray: &Ray, pixel_x: usize, pixel_y: usize, strength : f64) {
        let mut id: usize = 0; // 用于存放发生碰撞的物体的编号
        let mut t: f64 = 0.0; // 用于存放相交点到光线原点的距离
        if !self.intersect(ray, &mut t, &mut id) { // 射线与任何物体都没有相交，递归终止
            return;
        }
        let x = ray.o + ray.d.mult(t);  // 得到交点所在位置
        let color = self.objects[id].get_color();   // 获取物体的颜色

        //if let Some(next) = self.objects[id].cal_specular_reflection(&x, &ray.d) {
            //let new_ray = Ray { o: x, d: next };
            //self.trace_ray(&new_ray, pixel_x, pixel_y);
        //} else {
            //self.points.push(ViewPoint::new(x, Vector3::default(), Vector3::default(), pixel_x, pixel_y, color.mult(t / 50000.0)));
        //}
        self.points.push(ViewPoint::new(x, Vector3::default(), Vector3::default(), pixel_x, pixel_y, color));
    }

    pub fn photon_tracing(&mut self, photon : &mut Photon) {
        let mut id: usize = 0; // 用于存放发生碰撞的物体的编号
        let mut t: f64 = 0.0; // 用于存放相交点到光线原点的距离
        if !self.intersect(&photon.ray, &mut t, &mut id) { // 光子与任何物体都没有碰撞，递归终止
            return;
        }
        let x = photon.ray.o + photon.ray.d.mult(t);  // 得到交点所在位置
        self.view_tree.walk_photon(&x, photon.strength);
    }

    pub fn build_view_tree(&mut self) {
        self.view_tree.build(&mut self.points, 0);
        info!("count of view points : {}", self.points.len());
    }

    pub fn draw_picture(&mut self, pic: &mut Vec<Color>) {
        self.view_tree.setup_pixel(pic);
    }
}

mod primitive;
mod light;
pub mod material;

use std::boxed::Box;
use std::sync::Arc;
pub use super::util::*;
use self::primitive::*;
use self::material::Material;
use self::light::*;

pub struct Scene {
    objects : Vec<Box<dyn Primitive>>,  // 代表场景中的各个物体
    illumiants : Vec<Box<dyn Light>>,   // 代表场景中的各个光源
    points: Vec<ViewPoint>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: Vec::new(), illumiants : Vec::new(), points: Vec::new() }
    }

    pub fn init(&mut self) {
        self.objects.push(Box::new(Plane::new(   //Left
            Vector3::new(0.0, 1.0, 0.0),
            0.0,
            Arc::new(Material::new(Color::new(0.0, 0.0, 0.2), 1.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Plane::new(   //Right
            Vector3::new(0.0, 1.0, 0.0),
            10000.0,
            Arc::new(Material::new(Color::new(0.0, 0.0, 0.2), 1.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Plane::new(   // Bottom
            Vector3::new(0.0, 0.0, 1.0),
            100.0,
            Arc::new(Material::new(Color::new(0.0, 0.3, 0.0), 1.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Plane::new(   // Top
            Vector3::new(0.0, 0.0, 1.0),
            10000.0,
            Arc::new(Material::new(Color::new(0.5, 0.5, 0.5), 0.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Plane::new(  //Back
            Vector3::new(1.0, 0.0, 0.0),
            100.0,
            Arc::new(Material::new(Color::new(0.5, 0.0, 0.0), 1.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Plane::new(   // Front
            Vector3::new(1.0, 0.0, 0.0),
            20000.0,
            Arc::new(Material::new(Color::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0))
        )));
        self.objects.push(Box::new(Sphere::new(
            2000.0,
            Vector3::new(500.0, 5000.0, 2200.0),
            Arc::new(Material::new(Color::new(0.0, 1.0, 0.0), 0.0, 1.0, 0.0, 0.0)),
        )));
        // 设置光源
        self.illumiants.push(Box::new(DotLight::new(
            Vector3::new(9000.0, 9000.0, 9000.0), 10000
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

    // 光线追踪阶段
    /*
     * ray : 要追踪的射线
     * pixel_x , pixel_y : 对应像素在图片中的位置
     * weight : 该点在整个像素的显示中占据的比重
     */
    pub fn trace_ray(&mut self, ray: &Ray, pixel_x: usize, pixel_y: usize, weight : f64) {
        let mut id: usize = 0; // 用于存放发生碰撞的物体的编号
        let mut t: f64 = 0.0; // 用于存放相交点到光线原点的距离
        if !self.intersect(ray, &mut t, &mut id) { // 射线与任何物体都没有相交，递归终止
            return;
        }
        let x = ray.o + ray.d.mult(t);  // 得到交点所在位置
        let color = self.objects[id].get_color();   // 获取物体的颜色
        let n = self.objects[id].get_normal_vec(&x);
        let material = self.objects[id].get_material();

        self.points.push(
            ViewPoint::new(x, ray.d.mult(-1.0), n, pixel_x, pixel_y, color, material)
        );
    }

    fn photon_tracing(&self, photon : &mut Photon, tree : &mut KdTree) {
        let mut id: usize = 0; // 用于存放发生碰撞的物体的编号
        let mut t: f64 = 0.0; // 用于存放相交点到光线原点的距离
        if !self.intersect(&photon.ray, &mut t, &mut id) { // 光子与任何物体都没有碰撞，递归终止
            return;
        }
        photon.ray.o = photon.ray.o + photon.ray.d.mult(t);  // 得到交点所在位置
        tree.walk_photon(photon);
    }

    pub fn pm_round(&mut self, tree : &mut KdTree) {
        let length = self.illumiants.len();
        for idx in 0..length {
            while let Some(mut photon) = self.illumiants[idx].gen_photon() {
                self.photon_tracing(&mut photon, tree);
            }
        }
    }

    // 获取所有视点，用于构建视点树
    pub fn get_hit_points(&mut self) -> &mut Vec<ViewPoint> {
        &mut self.points
    }
}

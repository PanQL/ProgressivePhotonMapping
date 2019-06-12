use super::{Ray, Vector3, color::Color};
use crate::scene::material::Material;
use std::sync::Arc;

pub struct Photon {
    pub ray: Ray,
    pub power: Color,
}

#[derive(Clone)]
pub struct ViewPoint {
    pub pos: Vector3,   // 位置
    pub norm: Vector3,  // 该处的法向量
    pub dire: Vector3,  // 击中该处的视线射线方向
    pub x: usize,   // 在图片中对应的行位置
    pub y: usize,   // 在图片中对应的列位置
    pub color: Color, // 本身颜色值
    pub radius: f64,
    pub count: f64, // 已经被统计到该视点名下的光子数量
    pub flux_color: Color, // 光子累积的通量,初始化为(0,0,0)
    pub material : Arc<Material>, // 关于该视点所在位置的材质信息
    pub wgt : f64,  // 在像素点中的权重
}

impl ViewPoint {
    pub fn new(pos: Vector3, norm: Vector3, dire: Vector3, x: usize, y: usize, color: Color, material : Arc<Material>, wgt : f64) -> Self {
        ViewPoint { pos, norm, dire, x, y, color, radius: 500.0, count: 0.0, flux_color: Color::default(), material, wgt }
    }

    pub fn influenced(&self, photon : &Photon) -> bool {
        self.pos.distance(&photon.ray.o) < self.radius
    }

    pub fn handle(&mut self, photon : &Photon) {
        self.flux_color += photon.power.mult(self.material.brdf(&self.dire, &self.norm, &photon.ray.d));
    }
}

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
    pub color: Color, // 最终反馈的颜色值
    pub radius: f64,
    pub count: u32, // 已经被统计到该视点名下的光子数量
    pub flux_color: Color, // 光子累积的通量
    pub material : Arc<Material>, // 关于该视点所在位置的材质信息
}

impl ViewPoint {
    pub fn new(pos: Vector3, norm: Vector3, dire: Vector3, x: usize, y: usize, color: Color, material : Arc<Material>) -> Self {
        ViewPoint { pos, norm, dire, x, y, color, radius: 1000.0, count: 0, flux_color: Color::default(), material }
    }

    pub fn influenced(&self, photon : &Photon) -> bool {
        self.pos.distance(&photon.ray.o) < self.radius
    }
}

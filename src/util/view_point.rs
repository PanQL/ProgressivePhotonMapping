use super::{Ray, Vector3, MyVector3, color::Color, collision::Collider};
use crate::scene::material::Material;
use crate::consts::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Photon {
    pub ray: Ray,
    pub power: Color,
}

#[derive(Clone)]
pub struct ViewPoint {
    pub pos: Vector3,   // 位置
    pub norm: Vector3,  // 该处的法向量
    pub dire: Vector3,  // 击中该处的视线射线方向
    pub px_pos : usize, // 在图片中对应的像素位置
    pub color: Color, // 本身颜色值
    pub radius2: f64,
    pub count: f64, // 已经被统计到该视点名下的光子数量
    pub delta: f64, // 当前这轮被统计到该视点名下的光子数量
    pub flux_color: Color, // 光子累积的通量,初始化为(0,0,0)
    pub material : Arc<Material>, // 关于该视点所在位置的材质信息
}

impl ViewPoint {
    pub fn new(collider : &Collider, px_pos: usize,  wgt : f64) -> Self {
        ViewPoint { 
            pos : collider.pos, 
            norm : collider.norm_vec, 
            dire : collider.in_direction.mult(-1.0), 
            px_pos,
            color : collider.color.mult(wgt), 
            radius2: MAX_PH_RADIUS2, 
            count : 0.0, 
            delta : 0.0,
            flux_color: Color::default(), 
            material : collider.material.clone(), 
        }
    }

    pub fn influenced(&self, photon : &Photon) -> bool {
        self.pos.distance2(&photon.ray.o) < self.radius2
    }

    pub fn handle(&mut self, photon : &Photon) {
        let dist = self.pos.distance2(&photon.ray.o);
        if dist < self.radius2 {
            self.delta += 1.0;
            self.flux_color = self.flux_color + self.color * photon.power
                .mult(self.material.brdf(&photon.ray.d, &self.norm, &self.dire))
                .mult(1.0 - dist / self.radius2);
        }
    }

    pub fn renew(&mut self) {
        if self.delta > 1e-8 {
            let k = ( self.count as f64 + self.delta * 0.7) / ( self.count as f64 + self.delta);
            self.radius2 *= k;
            self.flux_color = self.flux_color.mult(k);
            self.count += self.delta * 0.7;
            self.delta = 0.0;
        }
    }
}

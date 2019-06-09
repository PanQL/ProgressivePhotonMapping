use super::{Ray, Vector3, color::Color};

pub struct Photon {
    pub ray: Ray,
    pub radius: f64,
    pub color: Color,
    pub strength : f64,
}

#[derive(Clone)]
pub struct ViewPoint {
    pub pos: Vector3,   // 位置
    pub norm: Vector3,  // 该处的法向量
    pub dire: Vector3,  // 击中该处的视线射线方向
    pub x: usize,   // 在图片中对应的行位置
    pub y: usize,   // 在图片中对应的列位置
    pub color: Color, // 最终反馈的颜色值
    pub strength : f64, // 最终反馈的亮度
    radius: f64,
    count: u32,
    flux_color: Color, // accumulated reflected flux
}

impl ViewPoint {
    pub fn new(pos: Vector3, norm: Vector3, dire: Vector3, x: usize, y: usize, color: Color) -> Self {
        ViewPoint { pos, norm, dire, x, y, color, strength : 0.0, radius: 8000.0, count: 0, flux_color: Color::default() }
    }

    pub fn cmp(&self, index: usize, other: &ViewPoint) -> Option<bool> {
        match index {
            0 => { Some(self.pos.x < self.pos.x) }
            1 => { Some(self.pos.y < self.pos.y) }
            2 => { Some(self.pos.z < self.pos.z) }
            _ => {
                error!("not a correct index for viewpoint cmp !");
                None
            }
        }
    }

    pub fn influenced(&self, ph : &Vector3) -> bool {
        self.pos.distance(ph) < self.radius
    }
}

use super::{Vector3, color::Color};

pub struct Photon {
    pub pos: Vector3,
    pub radius: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct ViewPoint {
    pub pos: Vector3,
    // hit position
    pub norm: Vector3,
    // normal vector
    pub dire: Vector3,
    // ray direction
    x: usize,
    y: usize,
    color: Color,
    radius: f64,
    // current photon radius
    count: u32,
    // accumulated photon count
    flux_color: Color, // accumulated reflected flux
}

impl ViewPoint {
    pub fn new(pos: Vector3, norm: Vector3, dire: Vector3, x: usize, y: usize, color: Color) -> Self {
        ViewPoint { pos, norm, dire, x, y, color, radius: 0.0, count: 0, flux_color: Color::default() }
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
}
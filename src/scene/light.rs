use crate::util::*;

pub trait Light {
    fn gen_photon(&self) -> Photon;
}

pub struct DotLight {
    pos: Vector3,
}

impl Light for DotLight {
    fn gen_photon(&self) -> Photon {
        Photon { 
            ray : Ray { o : self.pos, d : Vector3::random(), }, 
            power : Color::new(1.0, 1.0, 1.0), 
        }
    }
}

impl DotLight {
    pub fn new(pos : Vector3) -> Self {
        DotLight { pos }
    }
}

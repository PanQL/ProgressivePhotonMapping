use crate::util::*;
extern crate rand;
use rand::Rng;

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

pub struct AreaLight {
    pos : Vector3,
    dx : Vector3,
    dy : Vector3,
    dir : Vector3,
    color : Color,
}

impl Light for AreaLight {
    fn gen_photon(&self) -> Photon {
        let mut rng = rand::thread_rng();
        Photon { 
            ray : Ray { 
                o : self.pos + self.dx.mult(rng.gen_range(-30.0,30.0)) + self.dy.mult(rng.gen_range(-30.0,30.0)), 
                d : (self.dir + self.dx.mult(rng.gen_range(-1.0,1.0)) + self.dy.mult(rng.gen_range(-1.0,1.0))).normalize(), 
            }, 
            power : self.color.div(self.color.power()), 
        }
    }
}

impl AreaLight {
    pub fn new(pos : Vector3, dx : Vector3, dy : Vector3, dir : Vector3, color : Color ) -> Self {
        AreaLight { pos, dx, dy, dir, color }
    }
}

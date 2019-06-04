use crate::util::*;

pub trait Light {
    fn gen_ray(&self) -> Ray;
}

pub struct DotLight {
    pos: Vector3,
}

impl Light for DotLight {
    fn gen_ray(&self) -> Ray {
        Ray { o: self.pos, d: Vector3::random() }
    }
}
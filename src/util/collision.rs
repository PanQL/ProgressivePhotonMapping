use super::*;
use crate::scene::material::Material;
use std::sync::Arc;

pub struct Collider {
    pub pos : Vector3,
    pub material : Arc<Material>,
    pub norm_vec : Vector3,
    pub distance : f64,
    pub in_direction : Vector3,
    pub hash_value : u64,
    pub color : Color,
}

impl Collider {
    pub fn get_diffuse_ray(&self) -> Option<Vector3> {
        self.material.cal_diffuse_ray(&self.norm_vec)
    }

    pub fn get_specular_ray(&self) -> Option<Vector3> {
        self.material.cal_specular_ray(&self.in_direction, &self.norm_vec)
    }

    pub fn get_refractive_ray(&self, refracted : bool) -> Option<Vector3> {
        self.material.cal_refractive_ray(&self.in_direction, &self.norm_vec, refracted)
    }

    pub fn get_hash(&self) -> u64 {
        self.hash_value
    }
}

pub struct LightCollider {
    pub power : Color,
    pub dist : f64,
}

use super::*;
use crate::scene::material::Material;
use std::sync::Arc;

pub struct Collider {
    pub pos : Vector3,
    pub material : Arc<Material>,
    pub norm_vec : Vector3,
    pub distance : f64,
    pub in_direction : Vector3,
}

impl Collider {
}


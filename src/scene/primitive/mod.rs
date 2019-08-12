mod sphere;
mod plane;
mod bazier;

pub use super::material::*;
pub use crate::util::*;
pub use std::sync::Arc;
pub use crate::consts::EPS;
pub use rgb::*;
pub use std::f64::consts::PI;

pub use sphere::Sphere;
pub use plane::Plane;
pub use bazier::BazierCurve;


pub trait Primitive {
    fn intersect(&self, r : &Ray) -> Option<f64>;
    fn get_normal_vec(&self, pos : &Vector3) -> Vector3;
    fn get_color(&self, pos : &Vector3) -> Color;
    fn get_material(&self) -> Arc<Material>;
    fn get_hash(&self) -> u64;
}

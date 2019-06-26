use rand::Rng;
use nalgebra::Vector3 as V3;
pub use num_traits::identities::Zero;
use nalgebra::Rotation3;
use nalgebra::Unit;

pub type Vector3 = V3<f64>;

pub trait MyVector3 {
    fn random() -> Self;
    fn mult(&self, b: f64) -> Vector3;
    fn distance(&self, other : &Vector3) -> f64;
    fn distance2(&self, other : &Vector3) -> f64;
    fn get_vertical_vec(&self) -> Vector3;
    fn rotate(&self, axis : &Vector3, theta : f64) -> Vector3;
}

impl MyVector3 for Vector3 {
    fn random() -> Self {   // generate a vec randomly
        Vector3::new(
            rand::thread_rng().gen_range(-10.0, 10.0),
            rand::thread_rng().gen_range(-10.0, 10.0),
            rand::thread_rng().gen_range(-10.0, 10.0)
        ).normalize()
    }

    fn mult(&self, b: f64) -> Vector3 {    // multi a number on this vec
        Vector3::new(self.x * b,self.y * b,self.z * b)
    }

    fn distance(&self, other : &Vector3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
    
    fn distance2(&self, other : &Vector3) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }
    fn get_vertical_vec(&self) -> Vector3 {
        let ret = self.cross(&Vector3::new(0.0, 0.0, 1.0));
        if !ret.is_zero() { return ret.normalize(); }
        Vector3::new(1.0, 0.0, 0.0)
    }

    fn rotate(&self, axis : &Vector3, theta : f64) -> Vector3 {
        let b = Rotation3::from_axis_angle(&Unit::new_normalize(axis.clone()), theta);
        b.transform_vector(&self)
    }
}


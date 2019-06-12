use super::*;
use super::material::*;
use std::sync::Arc;
use crate::consts::EPS;

pub trait Primitive {
    fn intersect(&self, r : &Ray) -> Option<f64>;
    fn get_normal_vec(&self, pos : &Vector3) -> Vector3;
    fn get_color(&self) -> Color;
    fn get_material(&self) -> Arc<Material>;
}

pub struct Sphere {
    pub radius : f64,
    pub position : Vector3,
    pub material : Arc<Material>,
}

impl Primitive for Sphere {
    fn intersect(&self, r : &Ray) -> Option<f64> {  // 给定一条射线，判断其与本物体是否相交
        let op = self.position - r.o;   // 射线源点到球心的向量
        let eps : f64 = 1e-4;
        let b : f64 = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.radius * self.radius;
        if det < EPS {
            return None;
        } else {
            det = det.sqrt();
        }
        if b - det  > eps {
            return Some(b - det);
        } else if b + det > eps {
            return Some(b + det);
        }
        None
    }

    fn get_normal_vec(&self, pos : &Vector3) -> Vector3 {
        let ret = *pos - self.position;
        if !ret.is_zero_vec() { return ret.normalize(); }
        ret
    }

    fn get_color(&self) -> Color {
        self.material.color
    }

    fn get_material(&self) -> Arc<Material>{
        self.material.clone()
    }
}

impl Sphere {
    pub fn new(radius : f64, position : Vector3, material : Arc<Material>) -> Self {
        Sphere{ 
            radius, 
            position, 
            material,
        }
    }
}

pub struct Plane {
    direction : Vector3,
    distance : f64,
    material : Arc<Material>,
}

impl Primitive for Plane {
    fn intersect(&self, r : &Ray) -> Option<f64> {  // 给定一条射线，判断其与本物体是否相交
        let o_projection = r.o.dot(&self.direction);
        let d_projection = r.d.dot(&self.direction);
        if o_projection > 0.0 {
            if ( o_projection - self.distance) * d_projection < 0.0 {
                return Some((o_projection - self.distance).abs() / d_projection.abs());
            }
        } else if d_projection > 0.0 {
            return Some( ( self.distance - o_projection ) / d_projection );
        }
        return None;
    }

    fn get_normal_vec(&self, _ : &Vector3) -> Vector3 {
        self.direction
    }

    fn get_color(&self) -> Color {
        self.material.color
    }

    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }
}

impl Plane {
    pub fn new(direction : Vector3, distance : f64, material : Arc<Material>) -> Self {
        Plane { direction : direction.normalize(), distance , material, }
    }
}

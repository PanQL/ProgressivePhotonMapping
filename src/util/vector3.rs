use std::ops::*;

extern crate rand;

use rand::Rng;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3 {
    // use for 3D position
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

impl Vector3 {
    pub fn random() -> Self {   // generate a vec randomly
        Vector3 {
            x: rand::thread_rng().gen_range(-10.0, 10.0),
            y: rand::thread_rng().gen_range(-10.0, 10.0),
            z: rand::thread_rng().gen_range(-10.0, 10.0),
        }.normalize()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {     // generate a vec from x, y, z
        Vector3 { x, y, z }
    }

    pub fn dot(&self, b: &Vector3) -> f64 {   // dot another vec with self
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn mult(&self, b: f64) -> Vector3 {    // multi a number on this vec
        Vector3 { x: self.x * b, y: self.y * b, z: self.z * b }
    }

    pub fn normalize(&self) -> Vector3 {    // normalize this vec
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len == 0.0 {
            error!("vector to normalize is {:?}", self);
        }
        Vector3 { x: self.x / len, y: self.y / len, z: self.z / len }
    }

    pub fn is_zero_vec(&self) -> bool {     // test if it is 0 vec
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn negate(&self) -> Vector3 {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn distance(&self, other : &Vector3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
    
    pub fn distance2(&self, other : &Vector3) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }

    pub fn by_coordiante(&self, coord : usize) -> f64 {
        return match coord {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unimplemented!()
        }
    }

    pub fn get_vertical_vec(&self) -> Vector3 {
        let ret = self.cross(&Vector3::new(0.0, 0.0, 1.0));
        if !ret.is_zero_vec() { return ret.normalize(); }
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn cross(&self, other : &Vector3) ->  Vector3 {
        Vector3 { 
            x : self.y * other.z - self.z * other.y,
            y : self.z * other.x - self.x * other.z,
            z : self.x * other.y - self.y * other.x
        }
    }

    pub fn rotate(&self, axis : &Vector3, theta : f64) -> Vector3 {
        let mut x : f64 = 0.0;
        let mut y : f64 = 0.0;
        let mut z : f64 = 0.0;
        let cost = theta.cos();
        let sint = theta.sin();
        x += self.x * ( axis.x.powi(2) + ( 1.0 - axis.x.powi(2) ) * cost );
        x += self.y * ( axis.x * axis.y * ( 1.0 - cost ) - axis.z * sint );
        x += self.z * ( axis.x * axis.z * ( 1.0 - cost ) + axis.y * sint );
        y += self.x * ( axis.y * axis.x * ( 1.0 - cost ) + axis.z * sint );
        y += self.y * ( axis.y.powi(2) + ( 1.0 - axis.y.powi(2) ) * cost );
        y += self.z * ( axis.y * axis.z * ( 1.0 - cost ) - axis.x * sint );
        z += self.x * ( axis.z * axis.x + ( 1.0 - cost ) - axis.y * sint );
        z += self.y * ( axis.z * axis.y * ( 1.0 - cost ) + axis.x * sint );
        z += self.z * ( axis.z.powi(2) + ( 1.0 - axis.z.powi(2)) * cost );
        Vector3 { x, y, z}
    }

}


use std::ops::{ Add, AddAssign, Mul };
use crate::consts::*;

#[derive(Clone, Default, Debug, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn normalize(&self) -> Color {    // normalize this vec
        let len = (self.r * self.r + self.g * self.g + self.b * self.b).sqrt();
        if len == 0.0 {
            error!("color to normalize is {:?}", self);
        }
        Color { r: self.r / len, g: self.g / len, b: self.b / len }
    }

    pub fn is_zero_vec(&self) -> bool {     // test if it is 0 vec
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    pub fn to_int(&self) -> (u8, u8, u8) {
        let r = if self.r > EPS { 
            ( self.r.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u8 
        } else { 
            0u8 
        };
        let g = if self.g > EPS { 
            ( self.g.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u8 
        } else { 
            0u8 
        };
        let b = if self.b > EPS { 
            ( self.b.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u8 
        } else { 
            0u8 
        };
        info!("befor r is {} g is {} b is {}", self.r, self.g, self.b);
        info!("after r is {} g is {} b is {}", r, g, b);
        (r, g, b)
    }

    pub fn mult(&self, b: f64) -> Color {    // multi a number on this vec
        Color { r: self.r * b, g: self.g * b, b: self.b * b }
    }

    pub fn div(&self, b: f64) -> Color {    // multi a number on this vec
        if b < 1e-10 { 
            error!("color div a zero !");
            return Color::default(); 
        }
        Color { r: self.r / b, g: self.g / b, b: self.b / b }
    }

    pub fn to_u16(&self) -> (u16, u16, u16) {
        let r = if self.r > EPS { 
            ( self.r.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u16 
        } else { 
            0u16 
        };
        let g = if self.g > EPS { 
            ( self.g.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u16 
        } else { 
            0u16 
        };
        let b = if self.b > EPS { 
            ( self.b.clamp(0.0, 1.0).powf(1.0 / 2.2) * 65535.0 + 0.5 ) as u16 
        } else { 
            0u16 
        };
        (r, g, b)
    }

    pub fn power(&self) -> f64 {
        (self.r + self.g + self.b) / 3.0
    }

    pub fn refresh_by_power(&self) -> Color {
        let power = (self.r + self.g + self.b) / 3.0;
        if power < EPS { return Color::new(0.0, 0.0, 0.0); }
        Color { r : self.r / power, g : self.g / power, b : self.b / power }
    }

    pub fn norm_max(&self) -> Color {
        let m = self.r.max(self.g).max(self.b);
        return Color { r : self.r / m, g : self.g / m, b : self.b / m }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

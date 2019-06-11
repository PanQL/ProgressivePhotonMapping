use std::ops::{ Add, AddAssign, Mul };

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
        if !self.is_zero_vec() {
            let temp = self.normalize();
            return ((temp.r * 255.0) as u8,
                    (temp.g * 255.0) as u8,
                    (temp.b * 255.0) as u8);
        }
        (0u8, 0u8, 0u8)
    }

    pub fn mult(&self, num: f64) -> Color {
        Color { r: self.r * num, g: self.g * num, b: self.b * num }
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

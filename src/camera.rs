use super::util::*;

pub struct Camera {
    position: Vector3,
    direction: Vector3,
    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
            width: 0,
            height: 0,
        }
    }

    pub fn set_pos(&mut self, new_pos: &Vector3) {
        self.position = *new_pos;
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.height = height;
        self.width = width;
    }

    pub fn set_dir(&mut self, direction: Vector3) {
        self.direction = direction;
    }

    pub fn emitting(&self, i : usize, j : usize) -> Ray {
        let cx = self.direction.get_vertical_vec().mult(self.width as f64 / self.height as f64);
        let cy = cx.cross(&self.direction).mult(self.height as f64 / self.width as f64).mult(-1.0);
        let d = cx.mult(i as f64 / self.width as f64 - 0.5)
            + cy.mult(j as f64 / self.height as f64 - 0.5) + self.direction;
        Ray {
            o: self.position + d.mult(100.0),
            d: d.normalize(),
        }
    }
}

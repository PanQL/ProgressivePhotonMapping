use std::vec::Vec;
use std::boxed::Box;
use super::*;
use crate::consts::MAX_PH_RADIUS2;

pub struct KdTree {
    left: Option<Box<KdTree>>,
    // children
    right: Option<Box<KdTree>>,
    // common
    split: usize,
    value: Option<ViewPoint>,
    capacity: usize,
    photon_number : f64,
}

impl KdTree {
    pub fn new() -> Self {
        KdTree { left: None, right: None, split: 0, value: None, capacity: 1, photon_number: 0.0 }
    }

    pub fn build(&mut self, values: &mut Vec<ViewPoint>, split: usize) -> bool {
        if values.is_empty() {
            return false;
        }

        self.capacity = values.len();
        self.split = split;
        if self.capacity == 1 {
            self.value = Some(values.get(0).unwrap().clone());
            return true;
        }
        match split {
            0 => {
                values.sort_by(|a, b| {
                    a.pos.x.partial_cmp(&b.pos.x).unwrap()
                });
            }
            1 => {
                values.sort_by(|a, b| {
                    a.pos.y.partial_cmp(&b.pos.y).unwrap()
                });
            }
            2 => {
                values.sort_by(|a, b| {
                    a.pos.z.partial_cmp(&b.pos.z).unwrap()
                });
            }
            _ => {
                error!("only 3 demensions !");
                return false;
            }
        }

        let at = self.capacity / 2;
        let mut values1 = values.split_off(at);
        self.value = Some(values1.get(0).unwrap().clone());
        values1.remove(0);

        self.left = Some(Box::new(KdTree::new()));
        self.right = Some(Box::new(KdTree::new()));
        if !self.left.as_mut().unwrap().build(values, (self.split + 1) % 3) {
            self.left = None;
        }
        if !self.right.as_mut().unwrap().build(&mut values1, (self.split + 1) % 3) {
            self.right = None;
        }
        return true;
    }

    pub fn walk_photon(&mut self, photon: &Photon) {
        let point = self.value.as_mut().unwrap();
        if point.influenced(photon) {   // 统计光子的通量和数目
            // TODO 过于粗糙的估计
            point.handle(photon);
            self.photon_number += 1.0;
        }
        let judge = photon.ray.o.by_coordiante(self.split) - point.pos.by_coordiante(self.split);
        let judge2 = judge * judge;
        if judge <= 0.0 || judge2 <= MAX_PH_RADIUS2 {
            if let Some(left) = self.left.as_mut() {
                left.walk_photon(photon);
            }
        }
        if judge >= 0.0 || judge2 <= MAX_PH_RADIUS2 {
            if let Some(right) = self.right.as_mut() {
                right.walk_photon(photon);
            }
        }
    }

    pub fn setup_pixel(&mut self, pic: &mut Vec<Color>, width : usize, n_emitted : f64) {
        let point = self.value.as_ref().unwrap();
        // TODO 计算的是落在视点半径范围内的光子的平均色彩。
        let to_div = std::f64::consts::PI * n_emitted * point.radius2;
        pic[point.x * width + point.y] += point.flux_color.div(to_div) * point.color; //TODO !!!
        if let Some(left) = self.left.as_mut() {
            left.setup_pixel(pic, width, n_emitted);
        }
        if let Some(right) = self.right.as_mut() {
            right.setup_pixel(pic, width, n_emitted);
        }
    }

    pub fn renew(&mut self) {
        let point = self.value.as_mut().unwrap();
        if point.count > 1e-8 {
            let k = ( point.count as f64 + self.photon_number * 0.7) / ( point.count as f64 + self.photon_number);
            let new_radius2 = point.radius2 * k;
            point.radius2 = new_radius2;
            point.flux_color = point.flux_color.mult(k);
        }
        point.count += self.photon_number;
        self.photon_number = 0.0;
        if let Some(left) = self.left.as_mut() {
            left.renew();
        }
        if let Some(right) = self.right.as_mut() {
            right.renew();
        }
    }
}

use crate::util::*;
extern crate rand;
use rand::Rng;

pub trait Light {
    fn gen_photon(&self) -> Photon;
    fn intersect(&self, ray : &Ray) -> bool;
}

pub struct DotLight {
    pos: Vector3,
}

impl Light for DotLight {
    fn gen_photon(&self) -> Photon {
        Photon { 
            ray : Ray { o : self.pos, d : Vector3::random(), }, 
            power : Color::new(1.0, 1.0, 1.0), 
        }
    }

    fn intersect(&self, _ : &Ray) -> bool {
        // TODO
        return false;
    }
}

impl DotLight {
    pub fn new(pos : Vector3) -> Self {
        DotLight { pos }
    }
}

pub struct AreaLight {
    pos : Vector3,  // 横纵坐标最小的定点所在位置。
    dx : Vector3,   // 横向向量分量
    dy : Vector3,   // 纵向向量分量
    dir : Vector3, // 法向量
    color : Color,  // 颜色
    width : f64,    // 横向宽度
    height : f64,   // 纵向长度
}

impl Light for AreaLight {
    fn gen_photon(&self) -> Photon {
        let mut rng = rand::thread_rng();
        Photon { 
            ray : Ray { 
                o : self.pos + self.dx.mult(rng.gen_range(0.0,self.width)) + self.dy.mult(rng.gen_range(0.0,self.height)), 
                d : (self.dir + self.dx.mult(rng.gen_range(-1.0,1.0)) + self.dy.mult(rng.gen_range(-1.0,1.0))).normalize(), 
            }, 
            power : self.color.div(self.color.power()), 
        }
    }

    fn intersect(&self, ray : &Ray) -> bool {
        // 计算ray的方向向量在平面法向量的投影
        let projection = ray.d.dot(&self.dir);
        // 计算射线原点到矩形位置的向量
        let vec1 = self.pos - ray.o;
        // 查看在矩形法向量方向上，射线与向量vec1是否同向
        let mid_res = vec1.dot(&self.dir) * ray.d.dot(&self.dir);
        if mid_res < 1e-10 { return false } // 反向，必然不相交
        let projection_pos = ray.o + ray.d.mult(vec1.dot(&self.dir).abs() / projection.abs());  // 得到直线与平面的交点
        let new_vec = projection_pos - self.pos;
        let dx_proj = new_vec.dot(&self.dx);
        let dy_proj = new_vec.dot(&self.dy);
        if 0.0 < dx_proj && dx_proj < self.width && 0.0 < dy_proj && dy_proj < self.height {
            return true;
        }
        false
    }
}

impl AreaLight {
    pub fn new(pos : Vector3, dx : Vector3, dy : Vector3, dir : Vector3, color : Color, width : f64, height : f64 ) -> Self {
        AreaLight { pos, dx, dy, dir, color, width, height }
    }
}

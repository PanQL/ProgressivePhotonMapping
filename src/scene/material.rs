use super::*;
use crate::consts::EPS;
extern crate rand;
use rand::Rng;
use std::f64::consts::PI;

pub struct Material {
    color: Color,
    pub diffuse : f64,
    pub specular : f64,
    pub refraction : f64,
    pub rindex : f64,
}

impl Material {
    pub fn new(color: Color, diffuse: f64, specular: f64, refraction: f64, rindex: f64) -> Self {
        Material { color, diffuse, specular, refraction, rindex }
    }

    /*
     * 计算漫反射的单位方向
     * ray_x : 入射的射线
     * ray_n : 法向量
     */
     pub fn cal_diffuse_ray(&self, vec_n : &Vector3) -> Option<Vector3> {
         if self.diffuse > EPS {
             let mut rng = rand::thread_rng();
             let vert = vec_n.get_vertical_vec();
             let theta = rng.gen_range(0.0, 2.0) * PI;
             let phi = rng.gen_range(0.0, 2.0) * PI;
             return Some(vec_n.rotate(&vert, theta).rotate(vec_n, phi).normalize());
         }
         None
     }

     //计算镜面反射的单位方向
     //ray_x : 入射的射线
     //ray_n : 法向量
     pub fn cal_specular_ray(&self, vec_x : &Vector3, vec_n : &Vector3) -> Option<Vector3> {
         if self.specular > EPS {
             if vec_x.dot(vec_n) < 0.0 {
                 return Some(*vec_x - vec_n.mult(2.0 * vec_n.dot(vec_x)));
             } else {
                 return Some(*vec_x + vec_n.mult(2.0 * vec_n.dot(vec_x)));
             }
         }
         None
     }

     //计算折射的单位方向
     //ray_x : 入射的射线
     //ray_n : 法向量
     //refracted : 是否已经折射过奇数次
    pub fn cal_refractive_ray(&self, vec_x: &Vector3, vec_n: &Vector3, refracted : bool) -> Option<Vector3> {
        if self.refraction > EPS {
            let mut n = self.rindex;
            if !refracted {
                n = 1.0 / n;
            }
            let cos_i = -vec_n.dot(vec_x);
            let cos2_t = 1.0 - ( n * n) * ( 1.0 - cos_i * cos_i);
            if cos2_t > EPS {
                return Some(vec_x.mult(n) + vec_n.mult(n * cos_i - cos2_t.sqrt()));
            }
        }
        None
    }

    pub fn is_diffuse(&self) -> bool {
        self.diffuse > EPS
    }

    pub fn is_specular(&self) -> bool {
        self.specular > EPS
    }

    pub fn is_refractive(&self) -> bool {
        self.refraction > EPS
    }
    
    // TODO 正确计算漫反射分量
    pub fn brdf(&self, ray_r : &Vector3, vec_n : &Vector3, ray_i : &Vector3) -> f64 {
        let mut ret = 0.0;
        let test = ray_r.dot(vec_n);
        if self.is_diffuse() && test > EPS { // 存在漫反射分量
            ret += self.diffuse * test;
        }
        if let Some(refl) = self.cal_specular_ray(&ray_i.mult(-1.0), vec_n) {   // 存在镜面反射
            let projection = refl.dot(ray_r);
            if projection > EPS {
                ret += self.specular * projection.powi(10);
            }
        }
        ret.clamp(0.0, 1.0)
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

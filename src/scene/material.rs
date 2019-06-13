use super::*;
use crate::consts::EPS;

pub struct Material {
    pub color: Color,
    pub diffuse : f64,
    pub specular : f64,
    pub refraction : f64,
    pub refraction_index: f64,
}

impl Material {
    pub fn new(color: Color, diffuse: f64, specular: f64, refraction: f64, index: f64) -> Self {
        Material { color, diffuse, specular, refraction, refraction_index: index }
    }

    /*
     * 计算漫反射的单位方向
     * ray_x : 入射的射线
     * ray_n : 法向量
     */
     //pub fn cal_diffuse_ray(&self, vec_x : &Vector3, vec_n : &Vector3) -> Option<Vector3> {
         //if self.diffuse > EPS {
             //return Some(Vector3::random());
         //}
         //None
     //}

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
    //pub fn cal_refractive_ray(&self, vec_x: &Vector3, vec_n: &Vector3) -> Option<Vector3> {
        //if self.refraction > EPS {}
        //None
    //}

    pub fn is_diffuse(&self) -> bool {
        self.diffuse > EPS
    }

    pub fn is_specular(&self) -> bool {
        self.specular > EPS
    }
    
    pub fn brdf(&self, ray_r : &Vector3, vec_n : &Vector3, ray_i : &Vector3) -> f64 {
        let mut ret = 0.0;
        let p = ray_r.dot(vec_n);
        if self.is_diffuse() && p > EPS{ // 存在漫反射分量
            ret += self.diffuse * p;
        }
        if let Some(refl) = self.cal_specular_ray(&ray_i.mult(-1.0), vec_n) {   // 存在镜面反射
            let projection = refl.dot(ray_r);
            if projection > EPS {
                ret += self.specular * projection.powi(10);
            }
        }
        ret
    }
}

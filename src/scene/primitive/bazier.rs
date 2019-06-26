use super::*;
use nalgebra::base::Matrix3;
use nalgebra::base::Vector3 as V3;

pub struct BazierCurve {
    hash_value : u64,
    px : Vec<f64>,
    py : Vec<f64>,
}

impl BazierCurve {
    pub fn new() -> Self{
        BazierCurve {
            hash_value : 0u64,
            px : vec![10.0f64, 2.0f64, 3.0f64, 4.0f64],
            py : vec![10.0f64, 2.0f64, 3.0f64, 4.0f64],
        }
    }

    pub fn test(&self) {
        let matrix = Matrix3::new(1.0f64, 1.0f64, 1.0f64,
                                  1.0f64, 1.0f64, 1.0f64,
                                  1.0f64, 1.0f64, 1.0f64);
        //info!("{:?}", matrix.pseudo_inverse(1e-20));
        let v3 = V3::new(0.1f64, 0.2f64, 0.3f64);
        let v31 = V3::new(0.1f64, 0.2f64, 0.3f64);
        info!("{:?} {} {} {}", matrix * v3, v3.x, v3.y, v3.z);
        info!("{} {} {}", v3 + v31, v3 - v31, v3);
        let p = vec![10.0f64, 2.0f64, 3.0f64, 4.0f64];
        info!("result : {}", self.getdP(&p, 0.0));
    }

    // Bezier曲线给入参数求值。
    fn getP(&self, p : &Vec<f64>, t : f64) -> f64 {
        1.0*p[0]*(1.0-t)*(1.0-t)*(1.0-t) +
            3.0*p[1]*t*(1.0-t)*(1.0-t) +
            3.0*p[2]*t*t*(1.0-t) +
            1.0*p[3]*t*t*t
    }

    // Bezier曲线给入参数求切线导数值。
    fn getdP(&self, p : &Vec<f64>, t : f64) -> f64 {
        -3.0*p[0]*(1.0-t)*(1.0-t) +
            3.0*p[1]*(1.0-t)*(1.0-t) +
            -6.0*p[1]*t*(1.0-t) +
            6.0*p[2]*(1.0-t)*t +
            -3.0*p[2]*t*t +
            3.0*p[3]*t*t
    }

    // 给定t计算射线上的点
    fn getC(&self, ray : &Ray, t : f64) -> Vector3 {
        if t < 0.0 { error!("t should not be 0.0"); }
        ray.o + ray.d.mult(t)
    }

    // 给定u和theta计算旋转体方程的值
    fn getS(&self, u : f64, theta : f64) -> Vector3 {
        Vector3::new(self.getP(&self.py, u), theta.sin() * self.getP(&self.px, u), theta.cos() * self.getP(&self.px, u))
    }

    fn getF(&self, t : f64, u : f64, theta : f64, ray : &Ray) -> Vector3 {
        self.getC(ray, t) - self.getS(u, theta)
    }

    fn getdF(&self, u : f64, theta : f64, ray : &Ray) -> Matrix3<f64> {
        Matrix3::new(ray.d.x, -self.getdP(&self.py, u), 0.0,
                    ray.d.y, -theta.sin() * self.getdP(&self.px, u), -theta.cos() * self.getP(&self.px, u),
                    ray.d.z, -theta.cos() * self.getdP(&self.px, u), theta.sin() * self.getP(&self.px, u)
                    )
    }
}


use super::*;
use nalgebra::base::Matrix3;
use nalgebra::base::Vector3 as V3;
use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct BazierCurve {
    position : Vector3, // 位置
    hash_value : u64,
    material : Arc<Material>,   // 材质
    n_vec : Rc<RefCell<Vector3>>,    // 最近一次碰撞处的法向量
    px : Vec<f64>,
    py : Vec<f64>,
    maxX : f64,
    minX : f64,
    maxR : f64,
    minR : f64,
}

impl BazierCurve {
    // TODO
    pub fn new(position : Vector3, material : Arc<Material>) -> Self{
        BazierCurve {
            position,
            hash_value : 0u64,
            material,
            n_vec : Rc::new(RefCell::new(Vector3::zero())),
            px : vec![10.0f64, 2.0f64, 3.0f64, 4.0f64],
            py : vec![10.0f64, 2.0f64, 3.0f64, 4.0f64],
            maxX : 1e-20,
            minX : 1e20,
            maxR : 1e-20,
            minR : 1e20,
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
        info!("result : {}", self.getd_p(&p, 0.0));

        let vec = Vector3::new(1.0, 0.0, 0.0);
        let axis = Vector3::new(0.0, 0.0, 1.0);
        info!("{:?}", vec.rotate(&axis, 0.5 * std::f64::consts::PI));
    }

    // Bezier曲线给入参数求值。
    fn get_p(&self, p : &Vec<f64>, t : f64) -> f64 {
        1.0*p[0]*(1.0-t)*(1.0-t)*(1.0-t) +
            3.0*p[1]*t*(1.0-t)*(1.0-t) +
            3.0*p[2]*t*t*(1.0-t) +
            1.0*p[3]*t*t*t
    }

    // Bezier曲线给入参数求切线导数值。
    fn getd_p(&self, p : &Vec<f64>, t : f64) -> f64 {
        -3.0*p[0]*(1.0-t)*(1.0-t) +
            3.0*p[1]*(1.0-t)*(1.0-t) +
            -6.0*p[1]*t*(1.0-t) +
            6.0*p[2]*(1.0-t)*t +
            -3.0*p[2]*t*t +
            3.0*p[3]*t*t
    }

    // 给定t计算射线上的点
    fn get_c(&self, ray : &Ray, t : f64) -> Vector3 {
        if t < 0.0 { error!("t should not be 0.0"); }
        ray.o + ray.d.mult(t)
    }

    // 给定u和theta计算旋转体方程的值
    fn get_s(&self, u : f64, theta : f64) -> Vector3 {
        Vector3::new(self.get_p(&self.py, u), theta.sin() * self.get_p(&self.px, u), theta.cos() * self.get_p(&self.px, u))
    }

    fn get_f(&self, t : f64, u : f64, theta : f64, ray : &Ray) -> Vector3 {
        self.get_c(ray, t) - self.get_s(u, theta)
    }

    fn getd_f(&self, u : f64, theta : f64, ray : &Ray) -> Matrix3<f64> {
        Matrix3::new(ray.d.x, -self.getd_p(&self.py, u), 0.0,
                    ray.d.y, -theta.sin() * self.getd_p(&self.px, u), -theta.cos() * self.get_p(&self.px, u),
                    ray.d.z, -theta.cos() * self.getd_p(&self.px, u), theta.sin() * self.get_p(&self.px, u)
                    )
    }

    /*
     * args : ( t, u, theta) 代表曲面求交方程的三个自变量
     */
    fn init_args(&self, dir : &Vector3, args : &mut Vector3) {
        let mut rng = rand::thread_rng();

        let u = rng.gen_range(0.0, 1.0);    // 曲线参数初始化时随机在[0.0,1.0]取值
        let theta = rng.gen_range(0.0, 2.0 * PI); // 旋转角度也是随机取值，取值范围[0, 2pi]

        let h = rng.gen_range(self.minX, self.maxX);
        let t = ((self.position.x + h) - dir.x) / dir.x;
        args.x = t;
        args.y = u;
        args.z = theta;
    }
}

impl Primitive for BazierCurve {
    fn intersect(&self, r : &Ray) -> Option<f64> {
        let mut dist = 1e100;
        let lr : f64 = 0.7; // 衰减系数
        for cnt in 0..35 {
            // 初始化各项参数
            let mut args = Vector3::new(0.0, 0.0, 0.0);
            self.init_args(&r.d, &mut args);
            let mut flag : bool = false;
            for i in 0..20 {
                let t = args.x;
                let u = args.y;
                let theta = args.z;
                if u < 0.5 || u > 1.5 { break; }
                let F = self.get_f(t, u, theta, r); // 得到向量
                let dF = self.getd_f(u, theta, r);   // 得到矩阵

                let (_, max) = F.argmax();
                if max < 1e-7 {
                    flag = true;
                    break;
                }
                if let Ok(inverse_df) = dF.pseudo_inverse(1e-20) {
                    args = args - ( inverse_df * F) * lr;
                } else {
                    break;
                }
            }
            if !flag { continue; }
            if args.x < 0.0 { continue; }
            if args.y < 0.0 || args.y > 1.0 { continue; }
            if args.x > dist { continue; }
            dist = args.x;  // 将t作为距离
            // TODO 更新法向量

            return Some(dist);
        }
        None
    }

    fn get_normal_vec(&self, _ : &Vector3) -> Vector3 {
        self.n_vec.borrow().clone()
    }

    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }

    fn get_hash(&self) -> u64 {
        self.hash_value
    }

    fn get_color(&self, _pos : &Vector3) -> Color {
        self.material.color()
    }
}

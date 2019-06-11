use super::*;
use super::material::*;
use std::sync::Arc;

pub trait Primitive {
    fn intersect(&self, r : &Ray) -> Option<f64>;
    fn get_normal_vec(&self, pos : &Vector3) -> Vector3;
    fn get_color(&self) -> Color;
    fn get_material(&self) -> Arc<Material>;
    //fn cal_diffuse_reflection(&self, pos : &Vector3, ray : &Vector3) -> Option<Vector3>;    // 给定光线ray及照射位置pos，返回漫反射得到的射线
    //fn cal_specular_reflection(&self, pos : &Vector3, ray : &Vector3) -> Option<Vector3>;    // 给定光线ray及照射位置pos，返回镜面反射得到的射线
}

pub struct Sphere {
    pub radius : f64,
    pub position : Vector3,
    pub material : Arc<Material>,
}

impl Primitive for Sphere {
    fn intersect(&self, r : &Ray) -> Option<f64> {  // 给定一条射线，判断其与本物体是否相交
        let op = self.position - r.o;   // 射线源点到球心的向量
        let eps : f64 = 1e-4;
        let b : f64 = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.radius * self.radius;
        if det < 0.0 {
            return None;
        } else {
            det = det.sqrt();
        }
        if b - det  > eps {
            return Some(b - det);
        } else if b + det > eps {
            return Some(b + det);
        }
        None
    }

    fn get_normal_vec(&self, pos : &Vector3) -> Vector3 {
        let ret = *pos - self.position;
        if !ret.is_zero_vec() { return ret.normalize(); }
        ret
    }

    fn get_color(&self) -> Color {
        self.material.color
    }

    //fn cal_diffuse_reflection(&self, pos : &Vector3, dir : &Vector3) -> Option<Vector3> {
        //if self.material.is_diffuse() {
            //let normal_vec = self.get_normal_vec(pos);    // 根据物体形状信息获得在该点的法向量
            //return self.material
                       //.cal_diffuse_ray(dir, &normal_vec);  // 最终将法向量以及射入射线委托material进行计算，即根据材质计算漫反射结果。
        //}
        //None
    //}

    //fn cal_specular_reflection(&self, pos : &Vector3, dir : &Vector3) -> Option<Vector3> {
        //if self.material.is_specular() {
            //let normal_vec = self.get_normal_vec(pos);    // 根据物体形状信息获得在该点的法向量
            //info!("{:?} {:?}", dir, normal_vec);
            //return self.material
                       //.cal_specular_ray(dir, &normal_vec);  // 最终将法向量以及射入射线委托material进行计算，即根据材质计算镜面反射结果。
        //}
        //None
    //}

    fn get_material(&self) -> Arc<Material>{
        self.material.clone()
    }
}

impl Sphere {
    pub fn new(radius : f64, position : Vector3, material : Arc<Material>) -> Self {
        Sphere{ 
            radius, 
            position, 
            material,
        }
    }
}

pub struct Plane {
    direction : Vector3,
    distance : f64,
    material : Arc<Material>,
}

impl Primitive for Plane {
    fn intersect(&self, r : &Ray) -> Option<f64> {  // 给定一条射线，判断其与本物体是否相交
        let o_projection = r.o.dot(&self.direction);
        let d_projection = r.d.dot(&self.direction);
        if o_projection > 0.0 {
            if ( o_projection - self.distance) * d_projection < 0.0 {
                return Some((o_projection - self.distance).abs() / d_projection.abs());
            }
        } else if d_projection > 0.0 {
            return Some( ( self.distance - o_projection ) / d_projection );
        }
        return None;
    }

    fn get_normal_vec(&self, _ : &Vector3) -> Vector3 {
        self.direction
    }

    fn get_color(&self) -> Color {
        self.material.color
    }

    //fn cal_diffuse_reflection(&self, pos : &Vector3, dir : &Vector3) -> Option<Vector3> {
        //if self.material.is_diffuse() {
            //let normal_vec = self.get_normal_vec(pos);    // 根据物体形状信息获得在该点的法向量
            //return self.material
                       //.cal_diffuse_ray(dir, &normal_vec);  // 最终将法向量以及射入射线委托material进行计算，即根据材质计算漫反射结果。
        //}
        //None
    //}
    
    //fn cal_specular_reflection(&self, pos : &Vector3, dir : &Vector3) -> Option<Vector3> {
        //if self.material.is_specular() {
            //let normal_vec = self.get_normal_vec(pos);    // 根据物体形状信息获得在该点的法向量
            //return self.material
                       //.cal_specular_ray(dir, &normal_vec);  // 最终将法向量以及射入射线委托material进行计算，即根据材质计算漫反射结果。
        //}
        //None
    //}
    
    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }
}

impl Plane {
    pub fn new(direction : Vector3, distance : f64, material : Arc<Material>) -> Self {
        Plane { direction : direction.normalize(), distance , material, }
    }
}

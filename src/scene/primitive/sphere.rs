use super::*;

pub struct Sphere {
    pub radius: f64,
    pub position: Vector3,
    pub material: Arc<Material>,
    hash_value: u64,
}

impl Primitive for Sphere {
    fn intersect(&self, r: &Ray) -> Option<f64> {
        // 给定一条射线，判断其与本物体是否相交
        let op = self.position - r.o; // 射线源点到球心的向量
        let eps: f64 = 1e-4;
        let b: f64 = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.radius * self.radius;
        if det < EPS {
            return None;
        } else {
            det = det.sqrt();
        }
        if b - det > eps {
            return Some(b - det);
        } else if b + det > eps {
            return Some(b + det);
        }
        None
    }

    fn get_normal_vec(&self, pos: &Vector3) -> Vector3 {
        let ret = *pos - self.position;
        if !ret.is_zero() {
            return ret.normalize();
        }
        ret
    }

    fn get_color(&self, _pos: &Vector3) -> Color {
        // TODO 纹理贴图
        self.material.color()
    }

    fn get_material(&self) -> Arc<Material> {
        self.material.clone()
    }

    fn get_hash(&self) -> u64 {
        self.hash_value
    }
}

impl Sphere {
    pub fn new(id: u32, radius: f64, position: Vector3, material: Arc<Material>) -> Self {
        Sphere {
            radius,
            position,
            material,
            hash_value: calculate_hash(&id),
        }
    }
}

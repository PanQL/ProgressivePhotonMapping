
pub struct PathTracer {
}

pub struct RayTracer {
    camera : Arc<Camera>,
    scene : Arc<Scene>,
}

impl RayTracer {
    pub fn new(scene : Arc<Scene>) -> Self {
        RayTracer { scene }
    }

    pub fn trace_ray(&self, ray: &Ray, pixel_x: usize, pixel_y: usize, weight : f64, depth : u32) -> Color {
        let mut ret = Color::default();
        if let Some(collider) = self.scene.intersect(ray) {
            if collider.material.is_diffuse() {
                ret += collider.material.color;
            }
        }
        ret
    }
}

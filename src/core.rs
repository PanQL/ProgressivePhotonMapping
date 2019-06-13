use super::scene::Scene;
use super::util::*;
use super::camera::Camera;
use std::vec::Vec;

pub struct ProgressivePhotonTracer {
    camera : Camera,
    picture: Vec<Color>,
    width: usize,
    height: usize,
    scene: Scene,
    hit_point_map : KdTree,
    points : Vec<ViewPoint>,
}

impl ProgressivePhotonTracer {
    pub fn new(camera : Camera, scene : Scene) -> Self {
        ProgressivePhotonTracer{
            camera ,
            picture: Vec::new(),
            width: 0,
            height: 0,
            scene,
            hit_point_map : KdTree::new(),
            points : Vec::new(),
        }
    }

    pub fn ray_tracing_pass(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.camera.emitting(i, j);
                self.trace_ray(&ray, j, i, 1.0);
            }
        }
    }

    pub fn trace_ray(&mut self, ray: &Ray, pixel_x: usize, pixel_y: usize, weight : f64) {
        if let Some(collider) = self.scene.intersect(ray) {
            if collider.material.is_diffuse() {
                let vec_n = if collider.norm_vec.dot(&ray.d) > 0.0 {    // 这里需要的是和视线夹角为钝角的法向量
                    collider.norm_vec.mult(-1.0)
                } else {
                    collider.norm_vec
                };
                self.points.push(
                    ViewPoint::new(collider.pos, ray.d.mult(-1.0), vec_n, pixel_x, pixel_y, collider.material.color, 
                                   collider.material.clone(), weight)   // TODO corrent weight
                );
            }
            if collider.material.is_specular() {
                let spec_ray = Ray::new(
                    collider.pos,
                    collider.material.cal_specular_ray(&ray.d, &collider.norm_vec).unwrap()
                );
                self.trace_ray(&spec_ray, pixel_x, pixel_y, weight); // TODO correct weight
            }
        }
    }

    pub fn run(&mut self, times: usize) {
        let buffer: &mut [u16] = &mut vec![0; 1024 * 768 * 3];
        self.width = self.camera.width;
        self.height = self.camera.height;
        self.picture.resize((self.width * self.height) as usize, Color::default());

        let mut total_photon : f64 = 0.0;
        self.ray_tracing_pass(); // 从眼睛发射光线
        self.hit_point_map.build(&mut self.points, 0); // 构建视点树
        for i in 0..times {
            self.photon_tracing_pass(10_0000);
            total_photon += 100000.0;
            self.hit_point_map.renew();
            info!("{} rounds", i);
        }
        self.hit_point_map.setup_pixel(&mut self.picture, self.width, total_photon);

        //将结果写入png
        for i in 0..self.width {
            for j in 0..self.height {
                let (r, g, b) = self.picture[j * self.width + i].to_u16();
                buffer[(j * self.width + i) * 3] = r;
                buffer[(j * self.width + i) * 3 + 1] = g;
                buffer[(j * self.width + i) * 3 + 2] = b;
            }
        }
        unsafe {
            image::save_buffer("result.png", 
                               std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len() * 2),
                               self.width as u32, self.height as u32, image::RGB(16)).unwrap()
        }
    }

    fn photon_tracing(&mut self, photon : &mut Photon) {
        if let Some(collider) = self.scene.intersect(&photon.ray) {
            if collider.material.is_diffuse() {  // 漫反射属性
                photon.ray.o = collider.pos;
                photon.ray.d = photon.ray.d.mult(-1.0); // 方向设置为指向光源的方向
                self.hit_point_map.walk_photon(photon);   // 计算该光子对碰撞点的影响
            }
            if collider.material.is_specular() { // 镜面反射属性
                let mut new_photon = Photon {
                    ray : Ray { o : photon.ray.o, d : material.cal_specular_ray(&origin_d, &n) }, power : ,
                };
                self.photon_tracing(&mut new_photon, tree);
            }
        }
    }

    fn photon_tracing_pass(&mut self, photon_number : usize) {
        let number = self.scene.get_light_num();
        for i in 0..number {
            let illumiant = self.scene.get_light(i);
            for _ in 0..photon_number {
                let mut photon = illumiant.gen_photon();
                self.photon_tracing(&mut photon);
            }
        }
    }
}

#[macro_use] extern crate log;
extern crate env_logger;

use env_logger::Env;
use ppm::util::*;
use ppm::camera::Camera;
use ppm::scene::Scene;
use ppm::core::Render;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("ppm")).init();

    info!("Hello, world!");

    let mut scene = Scene::new();
    scene.init();
    let mut camera = Camera::new();
    camera.set_size(1024, 768);
    camera.set_pos(&Vector3::new(6000.0, 5000.0, 400.0));
    camera.set_dir(Vector3::new(-0.8, 0.6, 0.0));
    let mut render = Render::new(camera, scene);
    //render.run(50, 500_0000,5);
    //render.run_pt(10, 5);
    render.run_ppm(10);
}

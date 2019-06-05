#[macro_use] extern crate log;
extern crate env_logger;

use env_logger::Env;
use ppm::util::*;
use ppm::camera::Camera;
use ppm::scene::Scene;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("ppm")).init();

    info!("Hello, world!");

//    let mut vector: Vec<ViewPoint> = vec![];
//    let mut tree = KdTree::new();
//    vector.push(ViewPoint::new(
//        Vector3::new(0.0, 0.0, 0.0),
//        Vector3::new(0.0, 0.0, 0.0),
//        Vector3::new(0.0, 0.0, 0.0),
//        0,0,
//        Color::default(),
//    ));
//    tree.build(&mut vector, 0);
    let mut scene = Scene::new();
    scene.init();
    let mut camera = Camera::new(scene);
    camera.set_size(1024, 768);
    camera.set_pos(&Vector3::new(20000.0, 5000.0, 5000.0));
    camera.set_dir(Vector3::new(-1.0, 0.0, 0.0));
    camera.run(1);
}

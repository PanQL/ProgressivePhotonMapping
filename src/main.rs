#![allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Env;
use ppm::camera::Camera;
use ppm::core::ProgressivePhotonTracer;
use ppm::scene::Scene;
use ppm::util::*;
use std::sync::Arc;

use ppm::scene::primitive::BazierCurve;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("ppm")).init();

    info!("Hello, world!");

    let mut scene = Scene::new();
    scene.init();
    let mut camera = Camera::new();
    camera.set_size(512, 384);
    camera.set_pos(&Vector3::new(6000.0, 5000.0, 400.0));
    camera.set_dir(Vector3::new(-1.0, 0.0, 0.0));
    let mut ppm = ProgressivePhotonTracer::new(Arc::new(camera), Arc::new(scene));
    ppm.run(1, 4);
}

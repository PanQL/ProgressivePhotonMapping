#[macro_use] extern crate log;
extern crate env_logger;

use env_logger::Env;

use ppm::util::*;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("ppm")).init();

    info!("Hello, world!");
}

#![feature(clamp)]
#![allow(dead_code)]
#![feature(vec_resize_default)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate lodepng;
extern crate rgb;
extern crate nalgebra;
extern crate rand;
pub mod util;
pub mod scene;
pub mod camera;
mod consts;
pub mod core;

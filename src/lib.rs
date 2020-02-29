#![feature(clamp)]
#![allow(dead_code)]
#![feature(vec_resize_default)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate lodepng;
extern crate nalgebra;
extern crate rand;
extern crate rgb;
pub mod camera;
mod consts;
pub mod core;
pub mod scene;
pub mod util;

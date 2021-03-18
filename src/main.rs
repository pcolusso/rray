#[macro_use] extern crate log;

mod ray;
mod sphere;
mod hitable;
mod camera;
mod renderer;
mod material;

use material::{Lambertian, Metal, Dielectric};
use minifb::{Window, WindowOptions};
use anyhow::Result;
use nalgebra::Vector3;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::env;

use crate::sphere::Sphere;
use crate::hitable::{HitableList};
use crate::camera::Camera;

const W: usize = 400;
const H: usize = 200;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "info") }
    pretty_env_logger::init();
    
    let camera = Camera {
        lower_left: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0),
    };

    
    let world = HitableList{
        list: vec!(
            Box::new(Sphere {
                centre: Vector3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Arc::new(Lambertian { albedo: Vector3::new(0.8, 0.8, 0.0) })
            }),
            Box::new(Sphere {
                centre: Vector3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Lambertian { albedo: Vector3::new(0.8, 0.3, 0.3) })
            }),
            Box::new(Sphere {
                centre: Vector3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Metal { albedo: Vector3::new(0.8, 0.6, 0.2), fuzz: 0.3 })
            }),
            Box::new(Sphere {
                centre: Vector3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Metal { albedo: Vector3::new(0.8, 0.8, 0.8), fuzz: 0.8 })
            }),
            Box::new(Sphere {
                centre: Vector3::new(0.2, -0.07, -0.2),
                radius: 0.05,
                material: Arc::new(Dielectric { refractive_index: 1.5 })
            })
        )
    };
    
    let start = Instant::now();
    let buf = renderer::render(W, H, camera, world);
    info!("Took {:?} to render", start.elapsed());

    let mut window = Window::new("rray", W * 2, H * 2, WindowOptions::default())?;
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    while window.is_open() {
        window.update_with_buffer(&buf, W, H)?;
    }
    Ok(())
}

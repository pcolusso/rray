#[macro_use]
extern crate log;
extern crate nalgebra_glm as glm;

mod camera;
mod hitable;
mod material;
mod ray;
mod renderer;
mod sphere;

use anyhow::Result;
use glm::vec3;
use material::{Dielectric, Lambertian, Metal};
use minifb::{Window, WindowOptions};
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::sphere::Sphere;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "trace")
    }
    pretty_env_logger::init();

    
    let r: f32 = (std::f32::consts::PI / 4.0).cos();
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const width: usize = 500;
    let height: usize = (width as f32 / ASPECT_RATIO).floor() as usize;
    let scale = 2;

    let camera = Camera::new(vec3(-2.0, 2.0, 1.0), vec3(0.0,0.0, -1.0), vec3(0.0, 1.0, 0.0), 20.0, ASPECT_RATIO);

    let mat_ground = Arc::new(Lambertian { albedo: vec3(0.8, 0.8, 0.0)});
    let mat_centre = Arc::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5)});
    let mat_left = Arc::new( Dielectric { refractive_index: 1.5 });
    let mat_right = Arc::new( Metal { albedo: vec3(0.8, 0.6, 0.2), fuzz: 0.0});

    let world = HitableList {
        list: vec![
            // Floor
            Box::new(Sphere { centre: vec3(0.0, -100.5, -1.0), radius: 100.0, material: mat_ground }),
            Box::new(Sphere { centre: vec3(0.0, 0.0, -1.0), radius: 0.5, material: mat_centre }),
            Box::new(Sphere { centre: vec3(-1.0, 0.0, -1.0), radius: 0.5, material: mat_left.clone()}),
            Box::new(Sphere { centre: vec3(-1.0, 0.0, -1.0), radius: 0.45, material: mat_left.clone()}),
            Box::new(Sphere { centre: vec3(1.0, 0.0, -1.0), radius: 0.5, material: mat_right}),
        ],
    };

    let start = Instant::now();
    let buf = renderer::render(width, height, camera, world);
    info!("Took {:?} to render", start.elapsed());

    let mut window = Window::new("rray", width * scale, height * scale, WindowOptions::default())?;
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    while window.is_open() {
        window.update_with_buffer(&buf, width, height)?;
    }
    Ok(())
}

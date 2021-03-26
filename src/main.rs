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
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio).floor() as usize;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 0.5;

    let origin = vec3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);
    let camera = Camera {
        origin,
        horizontal,
        vertical,
        lower_left,
    };

    let world = HitableList {
        list: vec![
            // Floor
            Box::new(Sphere {
                centre: vec3(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Arc::new(Lambertian {
                    albedo: vec3(0.8, 0.8, 0.0),
                }),
            }),
            // Middle
            Box::new(Sphere {
                centre: vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }),
            }),
            // Right
            Box::new(Sphere {
                centre: vec3(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Metal {
                    albedo: vec3(0.8, 0.6, 0.2),
                    fuzz: 1.0,
                }),
            }),
            // Left
            Box::new(Sphere {
                centre: vec3(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Arc::new(Dielectric {
                    refractive_index: 1.5,
                }),
            }),
            // Box::new(Sphere {
            //     centre: vec3(-1.0, -0.35, -0.4),
            //     radius: 0.15,
            //     material: Arc::new(Dielectric {
            //         refractive_index: 1.5,
            //     }),
            // }),
            // Box::new(Sphere {
            //     centre: vec3(1.0, -0.35, -0.4),
            //     radius: -0.15,
            //     material: Arc::new(Dielectric {
            //         refractive_index: 0.5,
            //     }),
            // }),
        ],
    };

    let start = Instant::now();
    let buf = renderer::render(width, height, camera, world);
    info!("Took {:?} to render", start.elapsed());

    let mut window = Window::new("rray", width * 2, height * 2, WindowOptions::default())?;
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    while window.is_open() {
        window.update_with_buffer(&buf, width, height)?;
    }
    Ok(())
}

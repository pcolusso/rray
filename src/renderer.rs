use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList};
use crate::ray::Ray;
use glm::{vec3, Vec3};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

#[cfg(debug_assertions)]
const NUM_SAMPLES: u32 = 64;
#[cfg(not(debug_assertions))]
const NUM_SAMPLES: u32 = 256;
const MAX_DEPTH: u32 = 16;

pub fn vec_squared_length(vec: &Vec3) -> f32 {
    vec.x * vec.x + vec.y * vec.y + vec.z * vec.z
}

pub fn vec_length(vec: &Vec3) -> f32 {
    vec_squared_length(vec).sqrt()
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut v;
    loop {
        v = 2.0 * vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - vec3(1.0, 1.0, 1.0);
        if !(vec_squared_length(&v) >= 1.0) {
            break;
        }
    }
    v
}

pub fn colour<T: Hitable>(ray: &Ray, world: &T, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, f32::MAX) {
        if depth < MAX_DEPTH {
            let scattered = rec.material.scatter(&ray, &rec, rng);
            scattered
                .atten
                .component_mul(&colour(&scattered.ray, world, rng, depth + 1))
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    } else {
        let direction = ray.direction.normalize();
        let time = 0.5 * (direction.y + 1.0);
        (1.0 - time) * vec3(1.0, 1.0, 1.0) + time * vec3(0.5, 0.7, 1.0)
    }
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

pub fn render(width: usize, height: usize, camera: Camera, world: HitableList) -> Vec<u32> {
    let pb = ProgressBar::new((width * height) as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}, {percent}%] [{bar:40.cyan/blue}] {count}/{total_count} eta: {eta}, {per_sec} pixels/sec")
        .progress_chars("#>-"));

    (0..width * height)
        .into_par_iter()
        .progress_with(pb)
        .map_init(
            || thread_rng(),
            |mut rng, screen_pos| {
                let mut c = vec3(0.0, 0.0, 0.0);
                let i = height - 1 - screen_pos / width;
                let j = screen_pos % width;
                for _ in 0..NUM_SAMPLES {
                    let u = ((j as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = ((i as f32) + rng.gen::<f32>()) / (height as f32);
                    let r = camera.get_ray(&mut rng, u, v);
                    c += colour(&r, &world, &mut rng, 0);
                }
                c = (1.0 / NUM_SAMPLES as f32) * c;
                let ir = (255.99 * c.x.sqrt()) as u32;
                let ig = (255.99 * c.y.sqrt()) as u32;
                let ib = (255.99 * c.z.sqrt()) as u32;

                to_bgra(ir, ig, ib)
            },
        )
        .collect()
}

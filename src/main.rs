#[macro_use] extern crate log;

mod ray;
mod sphere;
mod hitable;
mod camera;

use minifb::{Window, WindowOptions};
use anyhow::Result;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hitable::{HitableList, Hitable};
use crate::camera::Camera;

const W: usize = 800;
const H: usize = 400;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut buf: Vec<u32> = vec![0; W * H];
    let mut window = Window::new("rray", W, H, WindowOptions::default())?;

    render(&mut buf);

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        window.update_with_buffer(&buf, W, H)?;
    }
    Ok(())
}

pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

fn random_in_unit_sphere() -> Vector3<f32> {

}

fn colour<T: Hitable>(ray: &Ray, world: &T) -> Vector3<f32> {
    if let Some(rec) = world.hit(ray, 0.0, f32::MAX) {
        0.5 * Vector3::new(rec.n.x + 1.0, rec.n.y + 1.0, rec.n.z + 1.0)
    } else {
        let direction = ray.direction.normalize();
        let time = 0.5 * (direction.y + 1.0);
        (1.0 - time) * Vector3::new(1.0, 1.0, 1.0) + time * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn render(buf: &mut Vec<u32>) {
    let camera = Camera {
        lower_left: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0),
    };
    let world = HitableList{
        list: vec!(
            Box::new(Sphere { centre: Vector3::new(0.0, 0.0, -1.0), radius: 0.5 }),
            Box::new(Sphere { centre: Vector3::new(0.0, -100.5, -1.0), radius: 100.0 }),
        )
    };
    let mut rng = rand::thread_rng();

    let mut index = 0;
    for i in buf.iter_mut() {
        let (x, y) = div_rem(index, W);
        let (v, u) = ((x as f32 + rng.gen::<f32>()) / H as f32, (y as f32 + rng.gen::<f32>()) / W as f32);
        let ray = camera.get_ray(u, v);
        let p = ray.point_at_parameter(2.0);
        let c = colour(&ray, &world) * 255.99;

        let c_packed = 255 << 24 | (c.x.floor() as u32) << 16 | (c.y.floor() as u32) << 8 | (c.z.floor() as u32);

        *i = c_packed;

        //info!("Pixel #{}, x: {}, y: {}, c: {:?}", index, x, y, c);

        index += 1;
    }
}

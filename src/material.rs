use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::renderer::random_in_unit_sphere; //TODO: Move?
use nalgebra::{Unit, Vector3};
use rand::prelude::*;


pub trait Material {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vector3<f32>)>;
}

// Diffuse
pub struct Lambertian {
    pub albedo: Vector3<f32>
}

impl Material for Lambertian {
    fn scatter(self: &Self, _: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vector3<f32>)> {
        let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
        let scattered = Ray { origin: hit_record.position, direction: target - hit_record.position };
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>
}

impl Material for Metal {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, _: &mut ThreadRng) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        let scattered = Ray { origin: hit_record.position, direction: reflected };
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some((scattered, self.albedo))
        }
        None
    }
}

fn reflect(n: Vector3<f32>, v: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}
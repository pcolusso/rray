use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::renderer::random_in_unit_sphere; //TODO: Move?
use nalgebra::Vector3;
use rand::prelude::*;

type V3 = Vector3<f32>;

pub trait Material {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, V3)>;
}

// Diffuse
pub struct Lambertian {
    pub albedo: V3
}

impl Material for Lambertian {
    fn scatter(self: &Self, _: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, V3)> {
        let target = hit_record.p + hit_record.n + random_in_unit_sphere(rng);
        let scattered = Ray { origin: hit_record.p, direction: target - hit_record.p };
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: V3
}

impl Material for Metal {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, _: &mut ThreadRng) -> Option<(Ray, V3)> {
        let reflected = reflect(ray.direction, hit_record.n);
        let scattered = Ray { origin: hit_record.p, direction: reflected };
        if scattered.direction.dot(&hit_record.n) > 0.0 {
            return Some((scattered, self.albedo))
        }
        None
    }
}

fn reflect(n: V3, v: V3) -> V3 {
    v - 2.0 * v.dot(&n) * n
}
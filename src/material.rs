use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::renderer::random_in_unit_sphere; //TODO: Move?
use nalgebra::{Unit, Vector3};
use rand::prelude::*;

pub struct Scatter {
    pub ray: Ray,
    pub atten: Vector3<f32>
}


pub trait Material {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter;
}

// Diffuse
pub struct Lambertian {
    pub albedo: Vector3<f32>
}

impl Material for Lambertian {
    fn scatter(self: &Self, _: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
        let scattered = Ray { origin: hit_record.position, direction: target - hit_record.position };
        Scatter { ray: scattered, atten: self.albedo }
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let reflected = reflect(ray.direction, hit_record.normal);
        let attenuation = self.albedo;
        let scattered = Ray { origin: hit_record.position, direction: reflected + self.fuzz * random_in_unit_sphere(rng) };
        Scatter { ray: scattered, atten: attenuation }
    }
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}
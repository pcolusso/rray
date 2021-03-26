use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::renderer::*; //TODO: Move?
use glm::{vec3, Vec3};
use rand::prelude::*;

pub struct Scatter {
    pub ray: Ray,
    pub atten: Vec3,
}

pub trait Material {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter;
}

// Diffuse
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(self: &Self, _: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
        let scattered = Ray {
            origin: hit_record.position,
            direction: target - hit_record.position,
        };
        Scatter {
            ray: scattered,
            atten: self.albedo,
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let reflected = reflect(ray.direction, hit_record.normal);
        let attenuation = self.albedo;
        let scattered = Ray {
            origin: hit_record.position,
            direction: reflected + self.fuzz * random_in_unit_sphere(rng),
        };
        Scatter {
            ray: scattered,
            atten: attenuation,
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let x = glm::dot(&-uv, &n);
    let cos_theta = if x < 1.0 { x } else { 1.0 };
    let r_out_perp = etai_over_etat * (uv - cos_theta * n);
    let r_out_para = -(1.0 - glm::length2(&r_out_perp).abs()).sqrt() * n;
    r_out_perp + r_out_para
}

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray.direction.normalize();
        let refracted = refract(unit_direction, hit_record.normal, refraction_ratio);
        Scatter {
            ray: Ray {
                origin: hit_record.position,
                direction: refracted,
            },
            atten: attenuation,
        }
    }
}

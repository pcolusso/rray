use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::renderer::*; //TODO: Move?
use glm::{Vec3, vec3};
use rand::prelude::*;

pub struct Scatter {
    pub ray: Ray,
    pub atten: Vec3
}


pub trait Material {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter;
}

// Diffuse
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(self: &Self, _: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
        let scattered = Ray { origin: hit_record.position, direction: target - hit_record.position };
        Scatter { ray: scattered, atten: self.albedo }
    }
}

pub struct Metal {
    pub albedo: Vec3,
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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: Vec3, n: Vec3, etai_over_etat: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = glm::dot(&uv, &n);
    let d  = 1.0 - etai_over_etat * etai_over_etat * (1.0 - dt * dt);
    if d > 0.0 {
        Some(etai_over_etat * (uv - n * dt) - n * d.sqrt())
    } else {
        None
    }
}

pub struct Dielectric {
    pub refractive_index: f32
}

impl Material for Dielectric {
    fn scatter(self: &Self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let reflected = reflect(ray.direction, hit_record.normal);
        let atten = vec3(1.0, 1.0, 0.0);

        if ray.direction.dot(&hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * ray.direction.dot(&hit_record.normal) / vec_length(&ray.direction);
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray.direction.dot(&hit_record.normal) / vec_length(&ray.direction);
        }

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            Scatter { ray: Ray { origin: hit_record.position, direction: refracted }, atten: atten }
        } else {
            if rng.gen::<f32>() < 0.5 {
                Scatter { ray: Ray { origin: hit_record.position, direction: reflected }, atten: atten }
            } else {
                Scatter { ray: Ray { origin: hit_record.position, direction: reflected }, atten: atten }
            }
            
        }
    }
}
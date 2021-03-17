use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::renderer::random_in_unit_sphere; //TODO: Move?
use nalgebra::Vector3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, atten: &Vector3<f32>) -> Option<(Ray, Vector3<f32>)>;
}

// Diffuse
pub struct Lambertian {
    pub albedo: Vector3<f32>
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, atten: &Vector3<f32>) -> Option<(Ray, Vector3<f32>)> {
        let target = hit_record.p + hit_record.n + random_in_unit_sphere();
        let scattered = Ray { origin: hit_record.p, direction: target - hit_record.p };
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>
}

impl Metal {
    pub fn reflect(n: Vector3<f32>, v: Vector3<f32>) -> Vector3<f32> {
        v - 2.0 * v.dot(&n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, atten: &Vector3<f32>) -> Option<(Ray, Vector3<f32>)> {
        let reflected = Metal::reflect(ray.direction, hit_record.n);
        let scattered = Ray { origin: hit_record.p, direction: reflected };
        if scattered.direction.dot(&hit_record.n) > 0.0 {
            return Some((scattered, self.albedo))
        }
        None
    }
}

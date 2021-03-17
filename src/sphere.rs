use nalgebra::{Vector3};
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Lambertian;

pub struct Sphere {
    pub centre: Vector3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> f32 {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            -1.0
        } else {
            (-b - d.sqrt()) / (2.0 * a)
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                let normal = (1.0 / self.radius) * (hit_point - self.centre);
                return Some(HitRecord{
                    t: temp,
                    p: hit_point,
                    n: normal,
                    m: Box::new(Lambertian { albedo: Vector3::new(1.0, 1.0, 1.0) })
                })
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                let normal = (1.0 / self.radius) * (hit_point - self.centre);
                return Some(HitRecord{
                    t: temp,
                    p: hit_point,
                    n: normal,
                    m: Box::new(Lambertian { albedo: Vector3::new(1.0, 1.0, 1.0) })
                })
            }
        }
        None
    }
}
use glm::Vec3;
use std::sync::Arc;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material + Sync + Send>
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
                    time: temp,
                    position: hit_point,
                    normal,
                    material: self.material.clone() // Here's where that arc comes in handy. The material will be put on the heap, and this will just increase the refcount.
                })
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                let normal = (1.0 / self.radius) * (hit_point - self.centre);
                return Some(HitRecord{
                    time: temp,
                    position: hit_point,
                    normal,
                    material: self.material.clone()
                })
            }
        }
        None
    }
}
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
                let outward_normal = (hit_point - self.centre) / self.radius;
                let mut rec = HitRecord::new(temp, hit_point, normal, self.material.clone());
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec)
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                let normal = (1.0 / self.radius) * (hit_point - self.centre);
                let mut rec = HitRecord::new(temp, hit_point, normal, self.material.clone());
                let outward_normal = (hit_point - self.centre) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec)
            }
        }
        None
    }
}
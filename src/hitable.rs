use crate::ray::Ray;
use crate::material::Material;

use glm::{vec3, Vec3};
use std::sync::Arc;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub time: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Sync>,
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable + Sync>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = None;
        for h in self.list.iter() {
            if let Some(rec)= h.hit(ray, t_min, closest_so_far) {
                hit = true;
                closest_so_far = rec.time;
                temp_rec = Some(rec);
            }
        }
        if hit {
            temp_rec
        } else {
            None
        }
    }
}
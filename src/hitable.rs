use crate::ray::Ray;
use crate::material::Material;
use nalgebra::{Vector3};
use std::sync::Arc;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub n: Vector3<f32>,
    pub m: Arc<dyn Material + Sync>,
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
                closest_so_far = rec.t;
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
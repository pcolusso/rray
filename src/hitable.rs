use crate::material::Material;
use crate::ray::Ray;

use glm::Vec3;
use std::sync::Arc;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub time: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Sync>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        time: f32,
        position: Vec3,
        normal: Vec3,
        material: Arc<dyn Material + Sync>,
    ) -> Self {
        let front_face= true;
        Self {
            time,
            position,
            normal,
            material,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = glm::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
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
            if let Some(rec) = h.hit(ray, t_min, closest_so_far) {
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

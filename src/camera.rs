use glm::{Vec3, vec3};
use rand::prelude::*;
use crate::ray::Ray;

pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera {
    pub fn new(from: Vec3, at: Vec3, up: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov * (std::f32::consts::PI / 180.0);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = (from - at).normalize();
        let u = glm::cross(&up, &w).normalize();
        let v = glm::cross(&w, &u);
        
        let origin = from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(&self, _: &mut ThreadRng, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
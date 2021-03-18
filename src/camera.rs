use nalgebra::Vector3;
use rand::prelude::*;
use crate::ray::Ray;

pub struct Camera {
    pub lower_left: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub origin: Vector3<f32>
}

impl Camera {
    pub fn get_ray(&self, _: &mut ThreadRng, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
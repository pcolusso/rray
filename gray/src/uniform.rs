use crate::camera::Camera;
// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub window_size: [u32; 2],
    pub seed: f32, 
    pub camera_lens_radius: f32,
    pub camera_origin: [f32; 3],
    pub camera_lower_left: [f32; 3],
    pub camera_horizontal: [f32; 3],
    pub camera_vertical: [f32; 3],
}

impl Uniforms {
    pub fn new() -> Self {
        let window_size = [600, 600];
        let seed = 42.0;
        let camera_origin = [0.0, 0.0, 0.0];
        let camera_lower_left = [0.0, 0.0, 0.0];
        let camera_horizontal = [1.0, 0.0, 0.0];
        let camera_vertical = [0.0, 1.0, 0.0];
        let camera_lens_radius = 0.0001;

        Uniforms {
            window_size, seed, camera_origin, camera_lower_left, camera_horizontal, camera_vertical, camera_lens_radius
        }
    }

    pub fn update_camera(&mut self, camera: &Camera) {
        self.camera_origin = camera.origin.into();
        self.camera_lower_left = camera.lower_left.into();
        self.camera_horizontal = camera.horizontal.into();
        self.camera_vertical = camera.vertical.into();
    }

    pub fn update_window_size(&mut self, width: &u32, height: &u32) {
        self.window_size = [*width, *height];
    }
}
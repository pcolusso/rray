use nalgebra_glm::Vec3;
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

        let w = (from - at).normalize();
        let u = nalgebra_glm::cross(&up, &w).normalize();
        let v = nalgebra_glm::cross(&w, &u);
        
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
}
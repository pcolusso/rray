use nalgebra_glm::Vec3;
pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f32,
    pub w: Vec3,
    pub u: Vec3,
    pub v: Vec3,
}

impl Camera {
    pub fn new(from: Vec3, at: Vec3, up: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * (std::f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let origin = from;

        let w = (from - at).normalize();
        let u = nalgebra_glm::cross(&up, &w).normalize();
        let v = nalgebra_glm::cross(&w, &u);
        
        let horizontal = (2.0 * half_width * focus_dist) * u;
        let vertical = (2.0 * half_width * focus_dist) * v;
        let lower_left =  origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
            w,
            u,
            v,
            lens_radius
        }
    }
}
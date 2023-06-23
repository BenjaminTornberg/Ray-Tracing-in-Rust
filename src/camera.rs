use super::vector::*;
use super::ray::*;

#[derive(Debug, Default)]
pub struct Camera{
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3
}
impl Camera{
    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray{ orig: self.origin, dir: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin }
    }
}
pub fn basic_camera() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

    Camera{
        aspect_ratio,
        viewport_height,
        viewport_width,
        focal_length,
        
        origin,
        horizontal,
        vertical,
        lower_left_corner
    }
    
}

use crate::utils::deg_to_rad;

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
    pub lower_left_corner: Vec3,

    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Point3

}
impl Camera{
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Point3,
        vfov: f64,
        aspect:f64
    ) -> Camera{
        let theta = deg_to_rad(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let focal_length = 1.0;

        let w = unit_vector(look_from-look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width*u;
        let vertical = viewport_height*v;
        let lower_left_corner =  origin - horizontal/2.0 - vertical/2.0 - w; 

        Camera { 
            aspect_ratio: aspect,
             viewport_height,
              viewport_width,
               focal_length,
                origin,
                 horizontal,
                  vertical,
                   lower_left_corner,
                    vfov,
                     look_from,
                      look_at,
                        vup }


        }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray{ orig: self.origin,
            dir: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin }
    }
}


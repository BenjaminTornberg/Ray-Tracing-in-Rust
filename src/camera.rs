use crate::utils::deg_to_rad;

use super::vector::*;
use super::ray::*;


#[derive(Debug, Default, Clone, Copy)]
pub struct Camera{
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}
impl Camera{
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Point3,
        vfov: f64,
        aspect:f64,
        focus_dist: f64,
        aperture: f64
    ) -> Camera{
        let theta = deg_to_rad(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        //let focal_length = 1.0;

        let w = unit_vector(look_from-look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal =  viewport_width*u * focus_dist;
        let vertical =  viewport_height*v * focus_dist;
        let lower_left_corner =  origin - horizontal/2.0 - vertical/2.0 - w*focus_dist; 

        let lens_radius = aperture / 2.0;


        Camera { 
                origin,
                 horizontal,
                  vertical,
                   lower_left_corner,
                    u,
                     v,
                      w,
                       lens_radius
                    }


        }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray{ orig: self.origin +offset, 
            dir: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset}
    }
}



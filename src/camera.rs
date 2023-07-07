use crate::utils::deg_to_rad;
use crate::utils::random_double_range;

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
    lens_radius: f64,
    time0: f64,
    time1: f64,
}
impl Camera{
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Point3,
        vfov: f64,
        aspect:f64,
        focus_dist: f64,
        aperture: f64,
        time0: f64,
        time1: f64,
    ) -> Camera{
        let theta = deg_to_rad(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

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
                       lens_radius,
                       time0,
                       time1
                    }


        }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new( self.origin +offset, 
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
            random_double_range(self.time0, self.time1)
        
        )

    }
}



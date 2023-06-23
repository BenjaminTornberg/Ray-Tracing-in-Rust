use super::vector::*;
use super::hittable::*;
use super::ray::*;
use std::fmt::Debug;


pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;
    
} 

impl Debug for dyn Material{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DO NOT KNOW WHY THE FUCK I GOTTA WRITE THIS SHIT")
    }
}

#[derive(Debug, Default)]
pub struct Lambertian{
    pub albedo: Color
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord,  attenuation: &mut Color,  scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.zero_near(){
            scatter_direction = rec.normal;
        }

        *scattered = ray(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
#[derive(Debug, Default)]
pub struct Metal{
    pub albedo: Color
}
impl Material for Metal{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord,  attenuation: &mut Color,  scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction()), rec.normal);
        
        *scattered = ray(rec.p, reflected);
        *attenuation = self.albedo;

        dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Debug, Default)]
pub struct BlankMaterial{
    pub albedo: Color
}
impl Material for BlankMaterial{
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered:  &mut Ray) -> bool {
        false
    }
}
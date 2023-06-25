use super::vector::*;
use super::hittable::*;
use super::ray::*;
use super::utils::*;
use std::fmt::Debug;


pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)>;
    
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.zero_near(){
            scatter_direction = -rec.normal;
        }

        Some((
            self.albedo,
            Some(ray(rec.p, scatter_direction)),
        ))
    }
}
#[derive(Debug, Default)]
pub struct Metal{
    pub albedo: Color,
    pub fuzz: f64
}
impl Material for Metal{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let reflected = reflect(unit_vector(ray_in.direction()), rec.normal);
        
        let scattered = ray(rec.p, reflected + self.fuzz*random_in_unit_sphere());


        if dot(scattered.direction(), rec.normal) > 0.0 {
            return Some((self.albedo, Some(scattered)));
        }else{
            return None;
        }
    }
}

#[derive(Debug, Default)]
pub struct Dielectric{
    pub ir: f64
}
//some shit is wrong with this
//almost like i'm seeing the reflections from the back and flipped
//sky is beeing fracted in incorrect places
//distorted refractions
impl Material for Dielectric{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let refraction_ratio = if !rec.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        if cannot_reflect || reflectance(cos_theta, refraction_ratio) > random_double(){
            let reflected = reflect(unit_direction, rec.normal);
            let scattered = ray(rec.p, reflected);
            Some((Vec3(1.0, 1.0, 1.0), Some(scattered)))
        }
        else{
            let direction = refract(unit_direction, rec.normal, refraction_ratio);
            let scattered = ray(rec.p, direction);
            Some((Vec3(1.0, 1.0, 1.0), Some(scattered)))
        }
    }
}

#[test]
fn test_refract() {
    let uv = Vec3(1.0, 1.0, 0.0);
    let n = Vec3(-1.0, 0.0, 0.0);
    let etai_over_etat = 1.0;
    let expected = Vec3(0.0, 1.0, 0.0);
    let actual = refract(uv, n, etai_over_etat);
    assert_eq!(actual, expected);
}
  
#[test]
fn test_reflectance() {
    let cosine = 0.0;
    let ref_idx = 1.5;
    let expected = 1.0;
    let actual = reflectance(cosine, ref_idx);
    assert_eq!(actual, expected);
}


#[derive(Debug, Default)]
pub struct BlankMaterial{
    pub albedo: Color
}
impl Material for BlankMaterial{
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
    eprintln!("ERROR: BlankMaterial");
        None
    }
}
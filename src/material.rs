use rand::Rng;

use super::vector::*;
use super::hittable::*;
use super::ray::*;
use std::fmt::Debug;



#[derive(Debug, Clone)]
pub enum Material {
    BlankMaterial(BlankMaterial),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
} 

impl Scatterable for Material{
        fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
            match *self{
                Material::BlankMaterial(_) => None,
                Material::Lambertian(b) => b.scatter(ray_in, rec),
                Material::Metal(c) => c.scatter(ray_in, rec),
                Material::Dielectric(d) => d.scatter(ray_in, rec),
            }
        }
}


pub trait Scatterable{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian{
    pub albedo: Color
}
unsafe impl Send for Lambertian{ }

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.zero_near(){
            scatter_direction = rec.normal;
        }
        //et albedo = color(ray_in)

        Some((
            self.albedo,
            Some(ray(rec.p, scatter_direction)),
        ))
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Metal{
    pub albedo: Color,
    pub fuzz: f64
}
impl Scatterable for Metal{
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Dielectric{
    pub ir: f64
}
//some shit is wrong with this
//the bug is in schlicks approximation
//main issue is occuring in the leftside of the sphere
impl Scatterable for Dielectric{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let mut rng = rand::thread_rng();
        let normal = unit_vector(rec.normal);
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot(-unit_direction, normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        
        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        //this is the problem
        // || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        //the issue with schlicks approximations is that ray that are supposed to refract are given the ability to reflect
        //now the problem is that the total internal refraction is causing the rays to scramble
        //eprintln!("{}",reflectance(cos_theta, refraction_ratio));
        if cannot_reflect || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>(){
            let reflected = reflect(ray_in.direction(), normal);
            let scattered = ray(rec.p, reflected);
            Some((Vec3(1.0, 1.0, 1.0), Some(scattered)))
        }
        else{
            let direction = refract(&unit_direction, &normal, refraction_ratio);
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
    let actual = refract(&uv, &n, etai_over_etat);
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
#[derive(Debug, Default, Clone, Copy)]
pub struct BlankMaterial{
    pub albedo: Color
}
impl Scatterable for BlankMaterial{
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
    eprintln!("ERROR: BlankMaterial");
        None
    }
}
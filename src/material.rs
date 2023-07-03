use rand::Rng;

use crate::texture::SolidColor;
use crate::texture::Tex;
use crate::texture::Texture;

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
    DiffuseLight(DiffuseLight),
} 

impl Scatterable for Material{
        fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
            match self{
                Material::BlankMaterial(_) => None,
                Material::Lambertian(b) => b.scatter(ray_in, rec),
                Material::Metal(c) => c.scatter(ray_in, rec),
                Material::Dielectric(d) => d.scatter(ray_in, rec),
                Material::DiffuseLight(e) => e.scatter(ray_in, rec),
            }
        }
        fn emmited(&self, u: f64, v: f64, p: &Point3) -> Color {
            match self{
                Material::BlankMaterial(a) => a.emmited(u, v, p),
                Material::Lambertian(b) => b.emmited(u, v, p),
                Material::Metal(c) => c.emmited(u, v, p),
                Material::Dielectric(d) => d.emmited(u, v, p),
                Material::DiffuseLight(e) => e.emmited(u, v, p),
            }
        }
}


pub trait Scatterable{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)>;
    fn emmited(&self, _u: f64, _v: f64, _p: &Point3) -> Color { Vec3::color(0.0, 0.0, 0.0) }
}

#[derive(Debug, Clone)]
pub struct Lambertian{
    pub albedo: Texture
}

impl Lambertian{
    pub fn new(tex: Texture) -> Lambertian {Lambertian { albedo: tex }}
    pub fn new_rgb(a: Color ) -> Lambertian {Lambertian { albedo: Texture::SolidColor(SolidColor::new(a.r(), a.g(), a.b()))}}
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere();

        if scatter_direction.zero_near(){
            scatter_direction = rec.normal;
        }
        let target = rec.p + scatter_direction;
        let scattered = Ray::new(rec.p, target - rec.p, ray_in.time);

        let albedo = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((
            albedo,
            Some(scattered),
        ))
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Metal{
    pub albedo: Color,
    pub fuzz: f64
}
impl Metal{
    pub fn new(albedo: Color, fuzz: f64) -> Metal {Metal { albedo, fuzz }}
}
impl Scatterable for Metal{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let reflected = reflect(ray_in.direction(), rec.normal);
        
        let scattered = Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere(), ray_in.time);

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
impl Dielectric{
    pub fn new(ir: f64) -> Dielectric {Dielectric { ir }}
}

impl Scatterable for Dielectric{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        let mut rng = rand::thread_rng();
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>(){
            let reflected = reflect(ray_in.direction(), rec.normal);
            let scattered = Ray::new(rec.p, reflected, ray_in.time);
            Some((Vec3(1.0, 1.0, 1.0), Some(scattered)))
        }
        else{
            let direction = refract(&unit_direction, &rec.normal, refraction_ratio);
            

            let scattered = Ray::new(rec.p, direction, ray_in.time);
            Some((Vec3(1.0, 1.0, 1.0), Some(scattered)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight{
    pub emit: Texture
}
impl DiffuseLight{
    pub fn new(texture: Texture) -> DiffuseLight { DiffuseLight { emit: texture }}
    pub fn new_color(color: Color) -> DiffuseLight { 
        DiffuseLight { emit: Texture::SolidColor(SolidColor::new(color.r(), color.g(), color.b()))}
    }
}
impl Scatterable for DiffuseLight{
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
        None
    }
    fn emmited(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
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
pub struct BlankMaterial(pub f64);
impl Scatterable for BlankMaterial{
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Option<Ray>)> {
    eprintln!("ERROR: BlankMaterial");
        None
    }
}
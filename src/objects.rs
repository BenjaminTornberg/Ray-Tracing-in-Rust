#[cfg(test)]
use crate::material::Dielectric;

use super::vector::*;
use super::hittable::*;
use super::ray::*;
use super::material::Material;

#[derive(Debug, Clone)]
pub enum Object{
    Sphere(Sphere)
}
impl Hittable for Object{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord> {
        match self{
            Object::Sphere(s) => s.hit(r, t_min, t_max),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
    pub material: Material
}

impl Sphere{
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere{ Self { center, radius, material}}
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.orig - self.center;
        let a = r.direction().sqrlen();
        let half_b = dot(oc, r.dir);
        let c = oc.sqrlen() - self.radius*self.radius;
        let discriminant = (half_b * half_b) - (a * c);


        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            let root = if root_a < t_max && root_a > t_min {
                    root_a
                } else if root_b < t_max && root_b > t_min {
                    root_b
                } else {
                    return None;
                };

            let p = r.at(root);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = dot(r.direction(), outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else { 
                -outward_normal
            };

            return Some(HitRecord {
                p,
                normal,
                material: &self.material,
                t: root,
                front_face
            });
        }
        None
    }
}

//fail
#[test]
fn test_sphere_tangent_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, material: Material::Dielectric(Dielectric{ir: 1.5})};
    let ray = ray(Vec3(0.0, 0.0, -1.0), Vec3(1.0, 0.0, 0.0));
    let hit = sphere.hit(&ray, 0.0001, f64::INFINITY).unwrap();
    let expected_normal = Vec3(1.0, 0.0, 0.0);
    let expected_point = Vec3(1.0, 0.0, 0.0);
    assert_eq!(hit.normal, expected_normal);
    assert_eq!(hit.p, expected_point);
}

//pass
#[test]
fn test_sphere_outside_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, material: Material::Dielectric(Dielectric{ir: 1.5})};
    let ray = ray(Vec3(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY).unwrap();
    let expected_normal = Vec3(0.0, 0.0, -1.0);
    let expected_point = Vec3(0.0, 0.0, -1.0);
    eprintln!("{:?} {:?}", hit.normal, -hit.normal);
    assert_eq!(hit.normal, expected_normal);
    assert_eq!(hit.p, expected_point);
}

//fail
#[test]
fn test_sphere_inside_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, material: Material::Dielectric(Dielectric{ir: 1.5})};
    let ray = ray(Vec3(0.0, 0.0, 0.5), Vec3(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0001, f64::INFINITY).unwrap();
    let expected_normal = Vec3(0.0, 0.0, -1.0);
    let expected_point = Vec3(0.0, 0.0, -1.0);
    eprintln!("{:?} {:?}", hit.normal, hit.p);
    assert_eq!(hit.normal, expected_normal);
    assert_eq!(hit.p, expected_point);
}

//fail
#[test]
fn test_sphere_origin_inside_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, material: Material::Dielectric(Dielectric{ir: 1.5})};
    let ray = ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY).unwrap();
    let expected_normal = Vec3(0.0, 0.0, -1.0);
    let expected_point = Vec3(0.0, 0.0, -1.0);
    eprintln!("{:?} {:?}", hit.normal, hit.p);
    assert_eq!(hit.normal, expected_normal);
    assert_eq!(hit.p, expected_point);
}

//pass
#[test]
fn test_sphere_behind_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, material: Material::Dielectric(Dielectric{ir: 1.5})};
    let ray = ray(Vec3(0.0, 0.0, 5.0), Vec3(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY).unwrap();
    let expected_normal = Vec3(0.0, 0.0, 1.0);
    let expected_point = Vec3(0.0, 0.0, 1.0);
    assert_eq!(hit.normal, expected_normal);
    assert_eq!(hit.p, expected_point);
}
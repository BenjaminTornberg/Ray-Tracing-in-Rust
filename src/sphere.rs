#[cfg(test)]
use std::rc::Rc;
#[cfg(test)]
use crate::material::Dielectric;

use super::vector::*;
use super::hittable::*;
use super::ray::*;

#[derive(Debug)]
pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: MatPtr
}
impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.origin() - self.center;
        let a = r.direction().sqrlen();
        let half_b = dot(oc, r.direction());
        let c = oc.sqrlen() - self.radius*self.radius;

        let discriminant = (half_b * half_b) - (a *c);

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = r.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = dot(r.dir,normal) < 0.0;

                    return Some(HitRecord {
                        t: *root,
                        normal: if front_face { normal } else { -normal },
                        front_face,
                        p,
                        mat_ptr: self.mat_ptr.clone(),
                    });
                }
            }
        }
        None
    }
}

#[test]
fn test_sphere_hit() {
    let center = Vec3(0.0, 0.0, 0.0);
    let sphere = Sphere{center, radius: 1.0, mat_ptr: MatPtr(Rc::new(Dielectric{ir: 1.5}))};
    let ray = ray(Vec3(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert_eq!(hit.unwrap().t, 4.0);
}
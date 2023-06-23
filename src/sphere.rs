use super::vector::*;
use super::hittable::*;
use super::ray::*;

#[derive(Debug, Default)]
pub struct Sphere{
    pub center: Point3,
    pub radius: f64
}
impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64,mut rec: &mut HitRecord) -> bool{
        let oc = r.origin() - self.center;
        let a = r.direction().sqrlen();
        let half_b = dot(oc, r.direction());
        let c = oc.sqrlen() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 { return false } 
        let sqrtd = discriminant.sqrt();

        // this finds the nearest root that lies in the range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root{
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }
}

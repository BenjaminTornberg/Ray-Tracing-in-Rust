//#[cfg(test)]
//use crate::material::Dielectric;

use super::vector::*;
use super::hittable::*;
use super::ray::*;
use super::material::Material;
use crate::aabb::Aabb;
use crate::utils::PI;

#[derive(Debug, Clone)]
pub enum Object{
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    XyRect(XyRect),
    XzRect(XzRect),
    YzRect(YzRect),
}
impl Hittable for Object{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord> {
        match self{
            Object::Sphere(s) => s.hit(r, t_min, t_max),
            Object::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Object::XyRect(xyrec) => xyrec.hit(r, t_min, t_max),
            Object::XzRect(xzrec) => xzrec.hit(r, t_min, t_max),
            Object::YzRect(yzrec) => yzrec.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self{
            Object::Sphere(s) => s.bounding_box(time0, time1),
            Object::MovingSphere(ms) => ms.bounding_box(time0, time1),
            Object::XyRect(xyrec) => xyrec.bounding_box(time0, time1),
            Object::XzRect(xzrec) => xzrec.bounding_box(time0, time1),
            Object::YzRect(yzrec) => yzrec.bounding_box(time0, time1),
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

            //sphere texture calculations
            let theta = (-outward_normal.y()).acos();
            let phi = (-outward_normal.z()).atan2(outward_normal.x()) + PI;
            let u = phi / (2.0*PI);
            let v = theta / PI;

            return Some(HitRecord {
                p,
                normal,
                material: &self.material,
                t: root,
                u,
                v,
                front_face
            });
        }
        None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                self.center - Vec3(self.radius, self.radius, self.radius),
                self.center + Vec3(self.radius, self.radius, self.radius)
            )
        )
    }
}

#[derive(Debug, Clone)]
pub struct MovingSphere{
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,

}

impl MovingSphere{
    pub fn new(center0: Point3, center1: Point3, time0: f64, time1: f64, radius: f64, material: Material) -> MovingSphere
    { Self { center0, center1, time0, time1, radius, material}}

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0) *(self.center1 - self.center0))
    }
}

impl Hittable for MovingSphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.orig - self.center(r.time);
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
            let outward_normal = (p - self.center(r.time)) / self.radius;
            let front_face = dot(r.direction(), outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else { 
                -outward_normal
            };

            //sphere texture calculations
            let theta = (-outward_normal.y()).acos();
            let phi = (-outward_normal.z()).atan2(outward_normal.x()) + PI;
            let u = phi / (2.0*PI);
            let v = theta / PI;

            return Some(HitRecord {
                p,
                normal,
                material: &self.material,
                t: root,
                u,
                v,
                front_face
            });
        }
        None
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3(self.radius, self.radius, self.radius));
        
        let box1 = Aabb::new(
            self.center(time1) - Vec3(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3(self.radius, self.radius, self.radius));

        Some( Aabb::surrounding_box(&box0, &box1) )
    }
}

#[derive(Debug, Clone)]
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64, 
    material: Material,
}
impl XyRect{
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> XyRect{ XyRect { x0, x1, y0, y1, k, material}}
}
impl Hittable for XyRect{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let out_box = Aabb::new(Vec3(self.x0, self.y0, self.k-0.0001), Vec3(self.x1, self.y1, self.k+0.0001));
        Some(out_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max{
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1-self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3(0.0, 0.0, 1.0);
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else { 
            -outward_normal
        };
        let p = r.at(t);
        Some(HitRecord { p, normal, material: &self.material, t, u, v, front_face })
    }
}

#[derive(Debug, Clone)]
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}
impl XzRect{
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Material ) -> XzRect { XzRect { x0, x1, z0, z1, k, material }}
}
impl Hittable for XzRect{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let out_box = Aabb::new(Vec3(self.x0, self.z0, self.k-0.0001), Vec3(self.x1, self.z1, self.k+0.0001));
        Some(out_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max{
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1-self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3(0.0, 1.0, 0.0);
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else { 
            -outward_normal
        };
        let p = r.at(t);
        Some(HitRecord { p, normal, material: &self.material, t, u, v, front_face })
    }
}

#[derive(Debug, Clone)]
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}
impl YzRect{
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Material ) -> YzRect { YzRect { y0, y1, z0, z1, k, material }
    }
}
impl Hittable for YzRect{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let out_box = Aabb::new(Vec3(self.y0, self.z0, self.k-0.0001), Vec3(self.y1, self.z1, self.k+0.0001));
        Some(out_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max{
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1-self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3(1.0, 0.0, 0.0);
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else { 
            -outward_normal
        };
        let p = r.at(t);
        Some(HitRecord { p, normal, material: &self.material, t, u, v, front_face })
    }
}
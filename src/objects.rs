//#[cfg(test)]
//use crate::material::Dielectric;

use super::vector::*;
use super::hittable::*;
use super::ray::*;
use super::material::Material;
use crate::aabb::Aabb;
use crate::bvh::Hittables;
use crate::material::Isotropic;
use crate::texture::Texture;
use crate::utils::PI;
use crate::utils::deg_to_rad;
use crate::utils::random_double;

#[derive(Debug, Clone)]
pub enum Object{
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    XyRect(XyRect),
    XzRect(XzRect),
    YzRect(YzRect),
    BoxObject(BoxObject),
    Translate(Translate),
    RotateY(RotateY),
    ConstantMedium(ConstantMedium),
}
impl Hittable for Object{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord> {
        match self{
            Object::Sphere(s) => s.hit(r, t_min, t_max),
            Object::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Object::XyRect(xyrec) => xyrec.hit(r, t_min, t_max),
            Object::XzRect(xzrec) => xzrec.hit(r, t_min, t_max),
            Object::YzRect(yzrec) => yzrec.hit(r, t_min, t_max),
            Object::BoxObject(b) => b.hit(r, t_min, t_max),
            Object::Translate(t) => t.hit(r, t_min, t_max),
            Object::RotateY(ry) => ry.hit(r, t_min, t_max),
            Object::ConstantMedium(cm) => cm.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self{
            Object::Sphere(s) => s.bounding_box(time0, time1),
            Object::MovingSphere(ms) => ms.bounding_box(time0, time1),
            Object::XyRect(xyrec) => xyrec.bounding_box(time0, time1),
            Object::XzRect(xzrec) => xzrec.bounding_box(time0, time1),
            Object::YzRect(yzrec) => yzrec.bounding_box(time0, time1),
            Object::BoxObject(b) => b.bounding_box(time0, time1),
            Object::Translate(t) => t.bounding_box(time0, time1),
            Object::RotateY(ry) => ry.bounding_box(time0, time1),
            Object::ConstantMedium(cm) => cm.bounding_box(time0, time1), 
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

//TODO: add ability to make them one sided
//add option that allows for rays to pass through one side of the rect
//one_sided: Option<f64> 
//to determine if the ray passes through dot the normal with the ray and if it is the same sign as one sided return None
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

#[derive(Debug, Clone)]
pub struct BoxObject{
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}
impl BoxObject{
    pub fn new(p0: Point3, p1: Point3, material: Material) -> BoxObject { 
        let mut sides = HittableList::default();

        sides.add_obj(Object::XyRect(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone())));
        sides.add_obj(Object::XyRect(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone())));

        sides.add_obj(Object::XzRect(XzRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone())));
        sides.add_obj(Object::XzRect(XzRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone())));

        sides.add_obj(Object::YzRect(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone())));
        sides.add_obj(Object::YzRect(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone())));

        BoxObject { sides, box_min: p0, box_max: p1 } 
    }
}

impl Hittable for BoxObject{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}

#[derive(Debug, Clone)]
pub struct Translate{
    obj: Box<Hittables>,
    offset: Vec3
}
impl Translate{
    pub fn new_obj(obj: Object, displacement: Vec3) -> Translate {Translate::new(Hittables::Object(obj), displacement) }
    pub fn new(obj: Hittables, displacement: Vec3) -> Translate { Translate { obj: Box::new(obj), offset: displacement }}
}
impl Hittable for Translate{
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(bounding_box) = self.obj.bounding_box(time0, time1){
            return Some(Aabb::new(
                bounding_box.minimum + self.offset,
                bounding_box.maximum + self.offset
            ));
        }
        None
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(rec) = self.obj.hit(&moved_ray, t_min, t_max){
            let p = rec.p + self.offset;
            let front_face = dot(moved_ray.direction(), rec.normal) < 0.0;
            let normal = if front_face {
                rec.normal
            } else { 
                -rec.normal
            };
            return Some(
                HitRecord{
                    p,
                    normal,
                    material: rec.material,
                    t: rec.t,
                    u: rec.u,
                    v: rec.v,
                    front_face,
                }
            )
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct RotateY {
    obj: Box<Hittables>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>
}
impl RotateY{
    pub fn new_obj(obj: Object, angle: f64) -> RotateY{ RotateY::new(Hittables::Object(obj), angle)}
    pub fn new(obj: Hittables, angle: f64) -> RotateY {
        let radians = deg_to_rad(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bound_box = None;

        if let Some(bbox) = obj.bounding_box(0.0, 1.0){
            let mut max = [f64::MAX, f64::MAX, f64::MAX];
            let mut min = [f64::MIN, f64::MIN, f64::MIN];
            for i in 0..2{
                for j in 0..2{
                    for k in 0..2{
                        let x = bbox.maximum.x() + (1-i) as f64 * bbox.minimum.x();
                        let y = bbox.maximum.y() + (1-j) as f64 * bbox.minimum.y();
                        let z = bbox.maximum.z() + (1-k) as f64 * bbox.minimum.z();

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3(new_x, y, new_z);
                        for c in 0..3 {
                            max[c] = min[c].min(tester.to_array()[c]);
                            min[c] = max[c].max(tester.to_array()[c]);
                        }

                    }
                }
            }
            bound_box = Some(Aabb::new(Vec3(min[0], min[1], min[2]), Vec3(max[0], max[1], max[2])));
        }
        RotateY { obj: Box::new(obj), sin_theta, cos_theta, bbox: bound_box}
    }
}

impl Hittable for RotateY{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.0 = self.cos_theta*r.origin().0 - self.sin_theta*r.origin().2;
        origin.2 = self.sin_theta*r.origin().0 + self.cos_theta*r.origin().2;

        direction.0 = self.cos_theta*r.direction().0 - self.sin_theta*r.direction().2;
        direction.2 = self.sin_theta*r.direction().0 + self.cos_theta*r.direction().2;

        let rotated_r = Ray::new(origin, direction, r.time());

        let mut hr = None;

        if let Some(rec) = self.obj.hit(&rotated_r, t_min, t_max){
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.0 = self.cos_theta*rec.p.0 + self.sin_theta*rec.p.2;
            p.2 = -self.sin_theta*rec.p.0 + self.cos_theta*rec.p.2;

            normal.0 = self.cos_theta*rec.normal.0 + self.sin_theta*rec.normal.2;
            normal.2 = -self.sin_theta*rec.normal.0 + self.cos_theta*rec.normal.2;

            let front_face = dot(rotated_r.direction(), normal) < 0.0;
            normal = if front_face {
                normal
            } else { 
                -normal
            };

            hr = Some(HitRecord{
                p,
                normal,
                material: rec.material,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face
            })
        }
        hr
    }
}

#[derive(Debug, Clone)]
pub struct ConstantMedium{
    boundary: Box<Object>,
    phase_function: Material,
    neg_inv_density: f64
}
impl ConstantMedium{
    pub fn new(boundary: Object, density: f64, texture:Texture) -> ConstantMedium {
        ConstantMedium { boundary: Box::new(boundary), phase_function: Material::Isotropic(Isotropic::new(texture)), neg_inv_density: -1.0/density }
    }
    pub fn new_color(boundary: Object, density: f64, color: Color) -> ConstantMedium {
        ConstantMedium { boundary: Box::new(boundary), phase_function: Material::Isotropic(Isotropic::new_color(color)), neg_inv_density: -1.0/density }
    }
}
impl Hittable for ConstantMedium{
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, f64::MIN, f64::MAX){
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t+0.0001, f64::MAX){
                if rec1.t < t_min { rec1.t = t_min } 
                if rec2.t > t_max { rec2.t = t_max }
                if rec1.t >= rec2.t { return None }
                if rec1.t < 0.0 { rec1.t = 0.0}

                let ray_length= r.direction().len();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_double().log10();

                if hit_distance > distance_inside_boundary{
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);
                
                return Some(HitRecord{
                    normal: Vec3(1.0, 0.0, 0.0),
                    material: &self.phase_function,
                    u: rec1.u,
                    v: rec1.v,
                    front_face: true,
                    p,
                    t,
                });
            }
        }
        None
    }
}

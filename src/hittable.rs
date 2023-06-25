use super::vector::*;
use super::ray::*;
use std::fmt::Debug;
use std::rc::Rc;
use super::material::*;


#[derive(Debug, Default)]
pub struct HittableList{
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList{
    pub fn clear(&mut self){ self.objects.clear() }
    pub fn add(&mut self, object: Rc<dyn Hittable>){ self.objects.push(object) }
}

impl Hittable for HittableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
       
        for object in self.objects.iter(){

            if let Some(hit) = object.hit(r, t_min, closest_so_far){
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}


#[derive(Debug, Default, Clone)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: MatPtr,
    pub t: f64,
    pub front_face: bool
}


impl HitRecord{
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3){
        self.front_face = dot(self.normal, r.direction()) < 0.0;
        self.normal = if self.front_face { unit_vector(outward_normal) } else { - unit_vector(outward_normal) };
    }

}

pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord>;
}

impl Debug for dyn Hittable{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DO NOT KNOW WHY THE FUCK I GOTTA WRITE THIS SHIT")
    }
}


#[derive(Clone)]
pub struct MatPtr(pub Rc<dyn Material>);

impl Default for MatPtr{
    fn default() -> Self {
        MatPtr(Rc::new(BlankMaterial{..Default::default()}))
    }
}

impl Debug for MatPtr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GHAHAHAHA I HATE THIS SHIT")
    }
}
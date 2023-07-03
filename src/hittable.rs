use crate::aabb::Aabb;
use crate::bvh::Hittables;
use crate::objects::Object;
use super::vector::*;
use super::ray::*;
use std::fmt::Debug;
use super::material::*;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct HittableList{
    pub objects: Vec<Arc<Hittables>>
}

impl HittableList{
    pub fn clear(&mut self){ self.objects.clear() }
    pub fn add(&mut self, hittable: Hittables){ self.objects.push(Arc::new(hittable)) }
    pub fn add_obj(&mut self, object: Object) { self.objects.push(Arc::new(Hittables::Object(object)))}

    pub fn new(objects: Vec<Arc<Hittables>>) -> HittableList { HittableList { objects }}
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty(){ return None }

        let mut overall_box = self.objects[0].bounding_box(time0, time1).unwrap();

        for object in &self.objects[1..] {
            if let Some(object_box) = object.bounding_box(time0, time1) {
                overall_box = Aabb::surrounding_box(&overall_box, &object_box);
            }
        }
        return Some(overall_box);

    }
}


#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'material>{
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'material Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool
}



pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)->  Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}


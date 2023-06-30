use crate::objects::Object;
use super::vector::*;
use super::ray::*;
use std::fmt::Debug;
use super::material::*;




#[derive(Debug, Default, Clone)]
pub struct HittableList{
    pub objects: Vec<Object>
}

impl HittableList{
    pub fn clear(&mut self){ self.objects.clear() }
    pub fn add(&mut self, object: Object){ self.objects.push(object) }
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


#[derive(Debug, Clone)]
pub struct HitRecord<'material>{
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'material Material,
    pub t: f64,
    pub front_face: bool
}



pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64)-> Option<HitRecord>;
}


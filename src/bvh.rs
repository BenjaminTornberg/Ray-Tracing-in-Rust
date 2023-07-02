use std::cmp::Ordering;
use std::sync::Arc;

use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::aabb::Aabb;
use crate::objects::Object;
use crate::ray::Ray;


#[derive(Debug, Clone)]
pub enum Hittables{
    Object(Object),
    BvhNode(BvhNode),
}

impl Hittable for Hittables{
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self{
            Hittables::Object(obj) => {obj.bounding_box(time0, time1)},
            Hittables::BvhNode(bvh) => {bvh.bounding_box(time0, time1)},
        }
    }
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64)-> Option<crate::hittable::HitRecord> {
        match self{
            Hittables::Object(obj) => {obj.hit(r, t_min, t_max)},
            Hittables::BvhNode(bvh) => {bvh.hit(r, t_min, t_max)},
        }
    }
}

#[derive(Debug, Clone)]
pub struct BvhNode{
    left: Option<Arc<Hittables>>,
    right: Option<Arc<Hittables>>,
    pub bounding_box: Aabb,
}

impl BvhNode{
    pub fn new(src_objects: HittableList, time0: f64, time1: f64) -> BvhNode{
        let bounding_box = src_objects.bounding_box(time0, time1);
        let mut objects = src_objects.objects;

        let (left, right) = if objects.len() == 1 {
            (Some(objects[0].clone()), None)
        } else if objects.len() == 2 {
            (
                Some(objects[0].clone()),
                Some(objects[1].clone()),
            )
        }else{
            //TODO: cache the bounding boxes instead of redundently calculating for every object before sorting
            objects.sort_by(|a, b| {
                BvhNode::box_compare(&a.bounding_box(time0, time1).unwrap(), &b.bounding_box(time0, time1).unwrap(), 0)
            });

            let mid = objects.len() / 2;
            let (objects_left, objects_right) = objects.split_at(mid);
            (
                Some(Arc::new(Hittables::BvhNode(BvhNode::new( HittableList::new(objects_left.to_vec()), time0, time1)))),
                Some(Arc::new(Hittables::BvhNode(BvhNode::new( HittableList::new(objects_right.to_vec()), time0, time1)))),
            )
        };
        BvhNode {
            left,
            right,
            bounding_box: bounding_box.unwrap(),
        }

    }
    fn box_compare(a: &Aabb, b: &Aabb, axis: usize) -> Ordering {

        let a_min = a.minimum.to_array()[axis];
        let a_max = a.maximum.to_array()[axis];
        let b_min = b.minimum.to_array()[axis];
        let b_max = b.maximum.to_array()[axis];
    
        if a_min < b_min {
            Ordering::Less
        } else if a_min > b_min {
            Ordering::Greater
        } else if a_max < b_max {
            Ordering::Less
        } else if a_max > b_max {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

}

impl Hittable for BvhNode{
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.as_ref().and_then(|obj| obj.hit(r, t_min, t_max));

    
        if let Some(left) = hit_left {
            let t_max = left.t;
            let hit_right = self.right.as_ref().and_then(|obj| obj.hit(r, t_min, t_max));
            return hit_right.or(Some(left));
        }
        self.right.as_ref().and_then(|obj| obj.hit(r, t_min, t_max)) 
    }
}

use crate::{vector::{Point3, Vec3}, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Aabb{
    pub minimum: Point3,
    pub maximum:Point3,
}

impl Aabb{
    pub fn new(a: Point3, b: Point3) -> Aabb { Aabb{minimum: a, maximum: b}}

    #[allow(non_snake_case)]
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool{
        for a in 0..3 {
            let invD = 1.0 / r.direction().to_array()[a];

            let mut t0 = (self.minimum.to_array()[a] - r.origin().to_array()[a]) * invD;
        
            let mut t1 = (self.maximum.to_array()[a] - r.origin().to_array()[a]) * invD;

            if invD < 0.0 {
               std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min{
                return false;
            }
        }
        true
        
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Vec3(box0.minimum.x().min(box1.minimum.x()),
                            box0.minimum.y().min(box1.minimum.y()),
                            box0.minimum.z().min(box1.minimum.z()));

        let big = Vec3(box0.maximum.x().max(box1.maximum.x()),
                            box0.maximum.y().max(box1.maximum.y()),
                            box0.maximum.z().max(box1.maximum.z()));

        Aabb::new(small, big)
    }
}
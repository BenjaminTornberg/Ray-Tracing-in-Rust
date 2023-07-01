use super::vector::*;

#[derive(Debug, Default)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray{
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Ray{
        Ray { orig, dir, time }
    }
    pub fn origin(&self) -> Point3{
        self.orig
    }
    pub fn direction(&self) -> Vec3{
        self.dir
    }
    pub fn time(&self) -> f64 { self.time }
    pub fn at(&self, t: f64) -> Point3{
        self.origin() + self.direction() * t
    }
}


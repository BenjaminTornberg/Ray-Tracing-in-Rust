use super::vector::*;

#[derive(Debug, Default)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3
}

impl Ray{
    pub fn origin(&self) -> Point3{
        self.orig
    }
    pub fn direction(&self) -> Vec3{
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3{
        self.origin() + self.direction() * t
    }
}

pub fn ray(orig: Point3, dir: Vec3) -> Ray{
    Ray { orig, dir }
}

use super::utils::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3{
    pub fn x(&self) -> f64{ self.0 }
    pub fn y(&self) -> f64{ self.1 }
    pub fn z(&self) -> f64{ self. 2}

    pub fn r(&self) -> f64{ self.0 }
    pub fn g(&self) -> f64{ self.1 }
    pub fn b(&self) -> f64{ self. 2}

    pub fn len(&self) -> f64{
        self.sqrlen().sqrt()
    }

    pub fn sqrlen(&self) -> f64{
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }
}

impl std::ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.0)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3{
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2
    }
}

impl std::ops::MulAssign<Vec3> for Vec3{
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3{
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl std::ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1./rhs) * self
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64{
    u.0*v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: Vec3, v:Vec3) -> Vec3{
    Vec3(u.1*v.2 - u.2*v.1, u.2*v.0 - u.0*v.2, u.0*v.1 - u.1*v.0)
}

pub fn unit_vector(v: Vec3)-> Vec3{
    let l = v.len();
    v / l
}

pub fn write_point(p: Point3,){
    println!("{} {} {}", p.x(), p.y(), p.z())
}

pub fn write_colour(pixel_color: Color, samples_per_pixel: i32){
    
    //divide the color by the number of smples
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel_color.r() * scale;
    let g = pixel_color.g() * scale;
    let b = pixel_color.b() * scale;

    let r = (255.0 * clamp(r, 0.0, 0.999)) as i32;
    let g = (255.0 * clamp(g, 0.0, 0.999)) as i32;
    let b = (255.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{r} {g} {b}")
}
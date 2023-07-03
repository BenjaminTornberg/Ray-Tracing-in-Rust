use super::utils::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3{
    pub fn to_array(&self) -> [f64;3] { [self.0, self.1, self.2] }

    pub fn x(&self) -> f64{ self.0 }
    pub fn y(&self) -> f64{ self.1 }
    pub fn z(&self) -> f64{ self.2 }

    pub fn r(&self) -> f64{ self.0 }
    pub fn g(&self) -> f64{ self.1 }
    pub fn b(&self) -> f64{ self.2 }

    pub fn color(r: f64, g: f64, b: f64) -> Color{ Vec3(r, g, b) }

    pub fn len(&self) -> f64{
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn sqrlen(&self) -> f64{
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn random() -> Self { Self(random_double(), random_double(), random_double())}

    pub fn random_range(min: f64, max: f64) -> Self { 
        Self(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)) 
    }

    pub fn zero_near(&self) -> bool{
        self.0.abs() < f64::EPSILON && self.1.abs() < f64::EPSILON && self.2.abs() < f64::EPSILON
    } 

}

impl std::ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
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
impl std::ops::MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
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
    u.0*v.0 + u.1*v.1 + u.2*v.2
}

pub fn cross(u: Vec3, v:Vec3) -> Vec3{
    Vec3(u.1*v.2 - u.2*v.1, u.2*v.0 - u.0*v.2, u.0*v.1 - u.1*v.0)

}

pub fn unit_vector(v: Vec3)-> Vec3{
    let l = v.len();
    Vec3(v.0/l, v.1/l, v.2/l)
}

pub fn random_in_unit_sphere() -> Vec3{
    loop {
        let p = Vec3::random_range( -1.0, 1.0);
        if p.sqrlen() >= 1.0 { continue }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.sqrlen() < 1.0 { return p }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn reflect(v: Vec3, n: Vec3)-> Vec3{
    v - n * (2.0*dot(v, n))
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-*uv, *n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-1.0 * (1.0 - r_out_perp.sqrlen()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5) //- 6.0 * cosine * (1.0-cosine).powf(6.0)
}


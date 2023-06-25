use rand::prelude::*;

pub const INF: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg_to_rad(deg: f64)-> f64{ deg * PI / 180.0 }

//[0,1]
pub fn random_double() -> f64 { rand::thread_rng().gen() }

//[min, max)
pub fn random_double_range(min: f64, max: f64) -> f64 { min + (max-min) * random_double() }

pub fn clamp (x: f64, min: f64, max: f64) -> f64 { if x < min { min } else if x > max { max } else { x } }
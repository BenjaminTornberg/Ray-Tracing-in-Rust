use rand::prelude::*;

pub const PI: f64 = 3.141592653589793238462643383279502884197169;


pub fn deg_to_rad(deg: f64)-> f64{ 
    deg * (PI / 180.0) }


pub fn random_double() -> f64 { rand::thread_rng().gen_range(0.0..1.0)  }

pub fn random_double_range(min: f64, max: f64) -> f64 { rand::thread_rng().gen_range(min..max) }

pub fn random_int_range(min: i32, max: i32) -> i32 { rand::thread_rng().gen_range(min..max) }

pub fn clamp (x: f64, min: f64, max: f64) -> f64 { if x < min { min } else if x > max { max } else { x } }
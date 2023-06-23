use super::vector::*;
use super::utils::*;

pub fn write_colour(pixel_color: Color, samples_per_pixel: i32){
    //divide the color by the number of samples and gamma-correct for gamma=2
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.r() * scale).sqrt();
    let g = (pixel_color.g() * scale).sqrt();
    let b = (pixel_color.b() * scale).sqrt();

    let r = (255.0 * clamp(r, 0.0, 0.999)) as i32;
    let g = (255.0 * clamp(g, 0.0, 0.999)) as i32;
    let b = (255.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{r} {g} {b}")
}

pub fn color(r: f64, g: f64, b: f64) -> Color{
    Vec3(r, g, b)
}
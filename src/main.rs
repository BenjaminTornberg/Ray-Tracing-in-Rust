use std::sync::{Arc, Mutex};

use camera::Camera;
use config::random_scene;
use image_object::{ImageParams, Image};
use raytracer::render;
use vector::Vec3;

/* real time ray tracing
** my interpretation of the famous:
** ray tracing in one weekend
** done completely in rust
*/
pub mod vector;
pub mod ray;
pub mod hittable;
pub mod objects;
pub mod camera;
pub mod utils;
pub mod image_object;
pub mod material;
pub mod config;
pub mod raytracer;


//TODO: create tests
//TODO: make the project more Rust-like
//TODO: fix Dielectric Material

fn main() {

    //image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 600;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let params = ImageParams::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    let image = Image::new(params.image_width, params.image_height);

    
    //world
    //let world = random_scene();
    
    let world = config::test_scene();
     
    //camera
    //13 2 3 
    //0 0 0
    //20
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    //let dist_to_focus = (look_from - look_at).len();
    //let aperture = 2.0;

    let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio);

    let world_arc = Arc::new(Mutex::new(world));
    let image_mutex = Arc::new(Mutex::new(image));

    render(cam, world_arc, image_mutex, params);
}

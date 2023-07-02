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
pub mod aabb;
pub mod bvh;


//use minifb::{Window, WindowOptions, ScaleMode};
//might be fun to try creating a ray tracing by rendering rays to the screen individually
//but what I would have to do is render the pixels to a vec than periodically start taking 
//them from the vec calculate the color by storing the amoun of ray per pixel we have and render it to the screen
//Not so sure how i'm going to be able to render the scene unless i use asyn function, but at that poin it seems too complicated


//TODO: add textures, volumes, boxes, triangles, lights
//TODO: add ability to render .obj files
//TODO: write a scene editor
//TODO: serialize the scene constructor


fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let params = ImageParams::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    let image = Image::new(params.image_width, params.image_height);


    let scene_number = 0;
    let (world, camera) = match scene_number {
        0 => {
            let world = config::test_scene();
            let look_from = Vec3(0.0, 0.0, 0.0);
            let look_at = Vec3(0.0, 0.0, -1.0);
            let vup = Vec3(0.0, 1.0, 0.0);
            let vfov = 90.0;
            let dist_to_focus = 1.0; 
            let aperture = 0.1;
            let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio, dist_to_focus, aperture, 0.0, 1.0);
            (Some(world), Some(cam))
        },
        1 => {
            let world = random_scene();
            let look_from = Vec3(13.0, 2.0, 3.0);
            let look_at = Vec3(0.0, 0.0, 0.0);
            let vup = Vec3(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let dist_to_focus = 10.0; 
            let aperture = 0.1;
            let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio, dist_to_focus, aperture, 0.0, 1.0);
            (Some(world), Some(cam))

        },
        _ => {
            eprintln!("Invalid scene selected");
            (None, None) 
        }
    };

    let world_arc = Arc::new(world.unwrap());
    let image_mutex = Arc::new(Mutex::new(image));

    render(camera.unwrap(), world_arc, image_mutex, params);
}

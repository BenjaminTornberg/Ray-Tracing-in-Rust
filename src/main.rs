use std::sync::{Arc, Mutex};
use camera::Camera;
use config::*;
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
pub mod texture;
pub mod perlin;


//use minifb::{Window, WindowOptions, ScaleMode};
//might be fun to try creating a ray tracing by rendering rays to the screen individually
//but what I would have to do is render the pixels to a vec than periodically start taking 
//them from the vec calculate the color by storing the amoun of ray per pixel we have and render it to the screen
//Not so sure how i'm going to be able to render the scene unless i use asyn function, but at that poin it seems too complicated


//TODO: add volumes, boxes, triangles
//TODO: add ability to render .obj files
//TODO: write a scene editor
//TODO: serialize the scene constructor

//IDEAS: Have a version of the ray tracer focused on speed and efficiency and have another that visualy shows the rendering 
//IDEAS: create a file format that can store scene data and write a file converter that converst mtl files to my format
//IDEAS: write a scene editor capable of taking spheres, boxes, lights, or even obj files and move them, put textures on them and select materials

fn main() {
    //image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 400;
    let max_depth = 50;

    
    let mut background = Vec3::color(0.0, 0.0, 0.0);
    let mut look_from = Vec3(0.0, 0.0, 0.0);
    let mut look_at = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let mut vfov = 90.0;
    let mut dist_to_focus = 1.0; 
    let mut aperture = 0.0;

    //scene change
    let scene_number = 6;

    let world = match scene_number {
        0 => {
            let world = test_scene();
            background = Vec3::color(0.70, 0.80, 1.00);
            dist_to_focus = 1.0; 
            aperture = 0.1;
            Some(world)
        },
        1 => {
            let world = random_scene();
            background = Vec3::color(0.70, 0.80, 1.00);
            look_from = Vec3(13.0, 2.0, 3.0);
            look_at = Vec3(0.0, 0.0, 0.0);
            vfov = 20.0;
            dist_to_focus = 10.0;
            aperture = 0.1;
            Some(world)

        },
        2 => {
            let world = two_spheres();
            background = Vec3::color(0.70, 0.80, 1.00);
            look_from = Vec3(13.0, 2.0, 3.0);
            look_at = Vec3(0.0, 0.0, 0.0);
            vfov = 20.0;
            dist_to_focus = 10.0; 
            Some(world)
        },
        3 => {
            let world = earth();
            background = Vec3::color(0.70, 0.80, 1.00);
            look_from = Vec3(13.0, 2.0, 3.0);
            look_at = Vec3(0.0, 0.0, 0.0);
            vfov = 20.0;
            dist_to_focus = 10.0; 
            Some(world)
        },
        4 => {
            let world = simple_light();
            background =Vec3::color(0.0, 0.0, 0.0);
            look_from = Vec3(26.0, 3.0, 6.0);
            look_at = Vec3(0.0, 2.0, 0.0);
            vfov = 20.0;
            dist_to_focus = 10.0; 
            Some(world)
        },
        5 => {
            let world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            look_from = Vec3(278.0, 278.0, -800.0);
            look_at = Vec3(278.0, 278.0, 0.0);
            vfov = 40.0;
            Some(world)
        },
        6 => {
            let world = cornell_ball();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 600;
            look_from = Vec3(278.0, 278.0, -800.0);
            look_at = Vec3(278.0, 278.0, 0.0);
            vfov = 40.0;
            Some(world)
        },
        _ => {
            eprintln!("Invalid scene selected");
            None 
        }
    };

    let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio, dist_to_focus, aperture, 0.0, 1.0);

    let params = ImageParams::new(aspect_ratio, image_width, samples_per_pixel, max_depth, background);
    let image = Image::new(params.image_width, params.image_height);

    let world_arc = Arc::new(world.unwrap());
    let image_mutex = Arc::new(Mutex::new(image));

    render(cam, world_arc, image_mutex, params);
}

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
use material::*;
use utils::*;
use camera::*;
use vector::*;
use ray::*;
use hittable::*;
use objects::*;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, TryRecvError};
use image_object::Image;



//TODO: create tests
//TODO: make the project more Rust-like
//TODO: fix Dielectric Material

use std::thread;

// Define the number of threads
const NUM_THREADS: usize = 4;

fn ray_color(r: Ray,  world: &HittableList, depth: i32) -> Color{
    if depth <= 0{
        return Vec3::color(0.0, 0.0, 0.0);
    }
    let hit = world.hit(&r, 0.0001, INF);

    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(&r, &hit_record);
            match scattered{
                Some((albedo, scattered_ray)) => { 
                    let rgb = Vec3::color(0.0, 0.0, 0.0);
                    //let prob = 0.1;
                    match scattered_ray {
                        Some(sr) => {
                            let target_color = ray_color(sr, world, depth-1);

                            Vec3::color(
                                clamp(rgb.r() + albedo.r() * target_color.r(), 0.0, 1.0),
                                clamp(rgb.g() + albedo.g() * target_color.g(), 0.0, 1.0),
                                clamp(rgb.b() + albedo.b() * target_color.b(), 0.0, 1.0)
                            )
                        }
                        None => albedo
                    }
                },
                None => { return Vec3::color(0.0, 0.0, 0.0)}

            }

        },
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0-t)*Vec3::color(1.0, 1.0, 1.0) + t*Vec3::color(0.5, 0.7, 1.0);
        }

    }
}

fn main() {

    //image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let image = Image::new(image_width as u32, image_height as u32);

    //world
    //let world = random_scene();
    
    let world = config::test_scene();
    
     
    //camera
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    //let dist_to_focus = (look_from - look_at).len();
    //let aperture = 2.0;
    let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio);

    //let tile_width = image_width / NUM_THREADS;

    let world_arc = Arc::new(Mutex::new(world));
    let image_mutex = Arc::new(Mutex::new(image));

    let start = Instant::now();

    // Create a work queue using a channel
    let (tx, rx) = channel::<(u32, u32)>();

    // Enqueue all tiles into the work queue
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            tx.send((i as u32, j as u32)).unwrap();
        }
    }
    
    let rx_arc = Arc::new(Mutex::new(rx));

    let handles: Vec<_> = (0..NUM_THREADS).map(|thread_id| {
        eprintln!("Thread: {} starts", thread_id+1);
        let world_arc = world_arc.clone();
        let image_mutex = image_mutex.clone();
        let rx_mutex = Arc::clone(&rx_arc);
        thread::spawn(move || {
            loop{
                let rx = rx_mutex.lock().unwrap();
                let mut image_lock = image_mutex.lock().unwrap();
                let world_lock = world_arc.lock().unwrap();
                match rx.try_recv(){
                    Ok((i, j)) => {
                        let mut pixel_color = Vec3::color(0.0, 0.0, 0.0);

                        for _ in 0..samples_per_pixel {
                            let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                            let v = (j as f64 + random_double()) / (image_height - 1) as f64;
    
                            let r = cam.get_ray(u, v);
                            pixel_color += ray_color(r, &world_lock, max_depth);
                        }
    
                        image_lock.set_pixel(i as u32, j as u32, pixel_color, samples_per_pixel);
                    },
                    Err(TryRecvError::Empty) => { break},
                    Err(TryRecvError::Disconnected) => { break }
                }
            }
            eprintln!("Thread {} is finised", thread_id+1)
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    eprintln!("Finito, Time Elapsed {}ms", start.elapsed().as_millis());
    let image_lock = image_mutex.lock().unwrap();
    image_lock.output();
}



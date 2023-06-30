use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, TryRecvError};
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::hittable::{HittableList, Hittable};
use crate::image_object::{Image, ImageParams};
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::utils::{random_double, clamp};
use crate::vector::{Vec3, Color, unit_vector};

// Define the number of threads
const NUM_THREADS: usize = 4;

pub fn render(cam: Camera, world: Arc<HittableList>, image: Arc<Mutex<Image>>, params: ImageParams){
    let start = Instant::now();
    // Create a work queue using a channel
    let (tx, rx) = channel::<(u32, u32)>();

    // Enqueue all tiles into the work queue
    for j in (0..params.image_height).rev() {
        for i in 0..params.image_width {
            tx.send((i, j)).unwrap();
        }
    }

    let pb = Arc::new(Mutex::new(ProgressBar::new(((params.image_height-1) * (params.image_width - 1)) as u64)));
    
    let rx_arc = Arc::new(Mutex::new(rx));

    let handles: Vec<_> = (0..NUM_THREADS).map(|_| {
        let world_arc = Arc::clone(&world);
        let image_mutex = Arc::clone(&image);
        let rx_mutex = Arc::clone(&rx_arc);
        let pb_mutex = Arc::clone(&pb);
        thread::spawn(move || {
            loop{
                let rx = rx_mutex.lock().unwrap();
                match rx.try_recv(){
                    Ok((i, j)) => {
                        drop(rx);
                        let mut pixel_color = Vec3::color(0.0, 0.0, 0.0);
                        for _ in 0..params.samples_per_pixel {
                            let u = (i as f64 + random_double()) / (params.image_width - 1) as f64;
                            let v = (j as f64 + random_double()) / (params.image_height - 1) as f64;
    
                            let r = cam.get_ray(u, v);
                            pixel_color += ray_color(r, &world_arc, params.max_depth);
                        }
                        let mut image_lock = image_mutex.lock().unwrap();
                        image_lock.set_pixel(i as u32, j as u32, pixel_color, params.samples_per_pixel);
                        drop(image_lock);
                        let pb_lock = pb_mutex.lock().unwrap();
                        pb_lock.inc(1);
                    },
                    Err(TryRecvError::Empty) => { break},
                    Err(TryRecvError::Disconnected) => { break }
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    pb.lock().unwrap().finish_with_message("Done");
    eprintln!("\nTime Elapsed {:.2}s", start.elapsed().as_millis() as f64/1000.0);
    let image_lock = image.lock().unwrap();
    image_lock.output();
}

fn ray_color(r: Ray,  world: &HittableList, depth: u32) -> Color{
    if depth <= 0{
        return Vec3::color(0.0, 0.0, 0.0);
    }
    let hit = world.hit(&r, 0.001, f64::MAX);

    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(&r, &hit_record);
            match scattered{
                Some((albedo, scattered_ray)) => { 
                    let rgb = Vec3::color(0.0, 0.0, 0.0);
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
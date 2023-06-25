/* real time ray tracing
** my interpretation of the famous:
** ray tracing in one weekend
** done completely in rust
*/
pub mod vector;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod utils;
pub mod color;
pub mod material;
use material::*;
use color::*;
use utils::*;
use camera::*;
use vector::*;
use ray::*;
use hittable::*;
use sphere::*;
use std::rc::Rc;

//TODO: create tests

//TODO: make the project more Rust-like

fn ray_color(r: Ray,  world: &HittableList, depth: i32) -> Color{
    if depth <= 0{
        return color(0.0, 0.0, 0.0);
    }

    let hit = world.hit(&r, 0.001, INF);

    match hit {
        Some(hit_record) => {
            let scattered = hit_record.mat_ptr.0.scatter(&r, &hit_record);
            match scattered{
                Some((albedo, scattered_ray)) => { 
                    let rgb = color(0.0, 0.0, 0.0);
                    //let prob = 0.1;
                    match scattered_ray {
                        Some(sr) => {
                            let target_color = ray_color(sr, world, depth-1);

                            color(
                                clamp(rgb.r() + albedo.r() * target_color.r(), 0.0, 1.0),
                                clamp(rgb.g() + albedo.g() * target_color.g(), 0.0, 1.0),
                                clamp(rgb.b() + albedo.b() * target_color.b(), 0.0, 1.0)
                            )
                        }
                        None => albedo
                    }

                    
                },
                None => { return color(0.0, 0.0, 0.0)}

            }

        },
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
        }

    }
}

fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //world
    //let R = (PI/4.0).cos();

    let mut world: HittableList = HittableList{..Default::default()};

    let material_ground = MatPtr(Rc::new(Lambertian{albedo: color( 0.8, 0.8, 0.0)}));
    let material_center =  MatPtr(Rc::new(Lambertian{albedo: color( 0.2, 0.3, 0.6)}));
    let material_left = MatPtr(Rc::new(Dielectric{ir: 1.5}));
    let material_right = MatPtr(Rc::new(Metal{albedo: color(0.8, 0.6, 0.2), fuzz: 1.0}));

    world.add(Rc::new(
        Sphere{
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            mat_ptr: material_ground
    }));

    world.add(Rc::new(
        Sphere{ center: Vec3(0.0, 0.0, -1.0 ),
                radius: 0.5,
                mat_ptr: material_center
    }));
    world.add(Rc::new(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            mat_ptr: material_right
    }));
    world.add(Rc::new(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: -0.4,
            mat_ptr: material_left.clone()
    }));
    world.add(Rc::new(
        Sphere{
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat_ptr: material_left
    }));

    //camera
    let look_from = Vec3(-2.0, 2.0, 1.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 20.0;
    let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio);

    // Rendering to ppm
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev(){
        eprintln!("\rscanlines remaining: {j} ");
        for i in 0..image_width{
            let mut pixel_color: Color = color(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel{
                //gives you a value of 0.0 to 1.0 that represents a spot onthe screen
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }

            write_colour(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone!! ")

}

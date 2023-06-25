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

fn ray_color(r: Ray,  world: &HittableList, depth: i32) -> Color{
    let mut rec = HitRecord{ ..Default::default() };

    if depth <= 0{
        return color(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, INF, &mut rec){
        let mut scattered =  Ray{..Default::default()};
        let mut attenuation = Vec3{..Default::default()};

        if rec.mat_ptr.0.scatter(&r, &rec, &mut attenuation, &mut scattered){
            return attenuation * ray_color(scattered, world, depth-1);
        }

        return color(0.0,0.0,0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0)
}

fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //world
    let mut world: HittableList = HittableList{..Default::default()};

    let material_ground = MatPtr(Rc::new(Lambertian{albedo: color( 0.8, 0.8, 0.0)}));
    let material_center = MatPtr(Rc::new(Lambertian{albedo: color(0.7, 0.3, 0.3)}));
    let material_left = MatPtr(Rc::new(Metal{albedo:color(0.8, 0.8, 0.8), fuzz: 0.3}));
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
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat_ptr: material_left
    }));

    //camera
    let cam = basic_camera();

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

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
use utils::*;
use camera::*;
use vector::*;
use ray::*;
use hittable::*;
use sphere::*;
use std::rc::Rc;

fn ray_color(r: Ray,  world: &HittableList) -> Color{
    let mut rec = HitRecord{ ..Default::default() };
    if world.hit(&r, 0.0, INF, &mut rec){
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0))
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}

fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    //world
    let mut world: HittableList = HittableList{..Default::default()};
    world.add(Rc::new(
            Sphere{ center: Vec3(0.0, 0.0, -1.0 ),
                    radius: 0.5
        }));
    world.add(Rc::new(
            Sphere{
                center: Vec3(0.0, -100.5, -10.0),
                radius: 100.0
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
            let mut pixel_color: Color = Vec3(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel{
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }

            write_colour(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone!! ")

}

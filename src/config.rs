use crate::hittable::HittableList;
use crate::material::*;
use crate::utils::{random_double_range, random_double};
use crate::vector::Vec3;
use crate::objects::{Object, Sphere};

//TODO: CREATE SETUP IN HERE



pub fn test_scene() -> HittableList{
    let mut world: HittableList = HittableList{..Default::default()};

    let material_ground = Material::Lambertian(Lambertian{albedo: Vec3::color( 0.8, 0.8, 0.0)});
    let material_center =  Material::Lambertian(Lambertian{albedo: Vec3::color( 0.2, 0.3, 0.6)});
    //let material_center =  MatPtr(Rc::new(Dielectric{ir: 1.5}));
    let material_left = Material::Metal(Metal{albedo: Vec3::color(0.8, 0.6, 0.2), fuzz: 0.0});
    let material_right = Material::Dielectric(Dielectric{ir: 1.5});

    world.add(Object::Sphere(
        Sphere{
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground
    }));

    world.add(Object::Sphere(
        Sphere{ center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: material_center
    }));
    world.add(Object::Sphere(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right.clone()
    }));
    world.add(Object::Sphere(
        Sphere{
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left
    })); 
    world.add(Object::Sphere(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: -0.4,
            material: material_right.clone()
    }));

    world
}

pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList{..Default::default()};

    let ground_material = Material::Lambertian(Lambertian{albedo: Vec3::color(0.5, 0.5, 0.5)});
    world.add(Object::Sphere(Sphere{
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material
    }));
    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Vec3(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_mat;
                 if choose_mat < 0.8{
                    //diffuse
                    let albedo = Vec3(random_double(), random_double(), random_double());
                    sphere_mat = Material::Lambertian(Lambertian{albedo});
                    world.add(Object::Sphere(Sphere{
                        center,
                        radius: 0.2, 
                        material: sphere_mat
                    }))
                 } else if choose_mat < 0.95{
                    //metal
                    let albedo = Vec3(random_double_range(0.5, 1.0), random_double_range(0.5, 1.0), random_double_range(0.5, 1.0));
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_mat = Material::Metal(Metal{albedo, fuzz});
                    world.add(Object::Sphere(Sphere{
                        center,
                        radius: 0.2, 
                        material: sphere_mat
                    }))
                 } else{
                    //glass
                    sphere_mat = Material::Dielectric(Dielectric{ir: 1.5});
                    world.add(Object::Sphere(Sphere{
                        center,
                        radius: 0.2,
                        material: sphere_mat
                    }))
                 }
            }

            let mat1 = Material::Dielectric(Dielectric{ir: 1.5});
            world.add(Object::Sphere(Sphere{
                center: Vec3(0.0, 1.0, 0.0),
                radius: 1.0,
                material: mat1
            }));
            let mat2 = Material::Lambertian(Lambertian{albedo: Vec3::color(0.4, 0.2, 0.1)});
            world.add(Object::Sphere(Sphere{
                center: Vec3(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: mat2
            }));
            let mat3 = Material::Metal(Metal{albedo: Vec3::color(0.7, 0.6, 0.5), fuzz: 0.0});
            world.add(Object::Sphere(Sphere{
                center: Vec3(4.0, 1.0, 0.0),
                radius: 1.0,
                material: mat3
            }));
        }

    }
    world
}
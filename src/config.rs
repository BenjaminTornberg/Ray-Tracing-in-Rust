use crate::bvh::{BvhNode, Hittables};
use crate::hittable::{HittableList};
use crate::material::*;
use crate::texture::{Texture, CheckeredTexture, NoiseTexture, ImageTexture, SolidColor};
use crate::utils::{random_double_range, random_double};
use crate::vector::Vec3;
use crate::objects::{Object, Sphere, MovingSphere, XyRect, XzRect, YzRect};

//TODO: CREATE SETUP IN HERE



pub fn test_scene() -> HittableList{
    let mut world: HittableList = HittableList::default();

    let checkered = Texture::CheckeredTexture(CheckeredTexture::new_rgb(Vec3::color(0.2, 0.3, 0.1), Vec3::color(0.9, 0.9, 0.9)));

    let material_ground = Material::Lambertian(Lambertian::new(checkered));
    let material_center =  Material::Lambertian(Lambertian::new_rgb(Vec3::color( 0.2, 0.3, 0.6)));
    //let material_center =  MatPtr(Rc::new(Dielectric{ir: 1.5}));
    let material_left = Material::Metal(Metal{albedo: Vec3::color(0.8, 0.6, 0.2), fuzz: 0.0});
    let material_right = Material::Dielectric(Dielectric{ir: 1.5});

    world.add_obj(Object::Sphere(
        Sphere{
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground
    }));

    world.add_obj(Object::Sphere(
        Sphere{ center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: material_center.clone()
    }));
    world.add_obj(Object::Sphere(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right.clone()
    }));

    world.add_obj(Object::MovingSphere(MovingSphere::new(
        Vec3(1.0, 1.0, -1.0),
        Vec3(1.0, 1.0, 0.0),
        0.0,
        2.0, 
        0.5,
        material_center
    )));
    world.add_obj(Object::Sphere(
        Sphere{
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left
    })); 

    let mut bworld: HittableList = HittableList::default();
    let bvhs = BvhNode::new( world, 0.0, 1.0);
    let boundary_nodes = Hittables::BvhNode(bvhs);
    bworld.add(boundary_nodes);
    //negative radius sphere do not render properly using BVH
    bworld.add_obj(Object::Sphere(
        Sphere{
            center: Vec3(1.0, 0.0, -1.0),
            radius: -0.4,
            material: material_right.clone()
    }));
    
    bworld
}

pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::default();

    //let checkered = Texture::CheckeredTexture(CheckeredTexture::new_rgb(Vec3::color(0.2, 0.3, 0.1), Vec3::color(0.9, 0.9, 0.9)));
    let marble = Texture::NoiseTexture(NoiseTexture::new(4.0));
    let ground_material = Material::Lambertian(Lambertian::new(marble));

    world.add_obj(Object::Sphere(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material
    )));
    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Vec3(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_mat;
                 if choose_mat < 0.8{
                    //diffuse
                    let albedo = Vec3(random_double(), random_double(), random_double());
                    let center2 = center + Vec3(0.0, random_double_range(0.0, 0.5), 0.0);
                    sphere_mat = Material::Lambertian(Lambertian::new_rgb(albedo));
                    world.add_obj(Object::MovingSphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2, 
                        sphere_mat
                    )))
                 } else if choose_mat < 0.95{
                    //metal
                    let albedo = Vec3::color(random_double_range(0.5, 1.0), random_double_range(0.5, 1.0), random_double_range(0.5, 1.0));
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_mat = Material::Metal(Metal::new(albedo, fuzz));
                    world.add_obj(Object::Sphere(Sphere::new(
                        center,
                        0.2, 
                        sphere_mat
                    )))
                 } else{
                    //glass
                    sphere_mat = Material::Dielectric(Dielectric::new(1.5));
                    world.add_obj(Object::Sphere(Sphere::new(
                        center,
                        0.2,
                        sphere_mat
                    )))
                 }
            }

            let mat1 = Material::Dielectric(Dielectric::new(1.5));
            world.add_obj(Object::Sphere(Sphere::new(
                Vec3(0.0, 1.0, 0.0),
                1.0,
                mat1
            )));
            let mat2 = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.4, 0.2, 0.1)));
            world.add_obj(Object::Sphere(Sphere::new(
                Vec3(-4.0, 1.0, 0.0),
                1.0,
                mat2
            )));
            let mat3 = Material::Metal(Metal{albedo: Vec3::color(0.7, 0.6, 0.5), fuzz: 0.0});
            world.add_obj(Object::Sphere(Sphere::new(
                Vec3(4.0, 1.0, 0.0),
                1.0,
                mat3
            )));
        }
    }
    let mut scene = HittableList::default();
    let bvh_scene = BvhNode::new(world, 0.0, 1.0);
    scene.add(Hittables::BvhNode(bvh_scene));
    scene
}

pub fn two_spheres() -> HittableList{
    let mut world = HittableList::default();
    //let checker = Texture::CheckeredTexture(CheckeredTexture::new_rgb(Vec3::color(0.2, 0.3, 0.1), Vec3::color(0.9, 0.9, 0.9)));
    let pertext = Texture::NoiseTexture(NoiseTexture::new(4.0));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian(Lambertian::new(pertext.clone())))));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, 2.0, 0.0), 2.0, Material::Lambertian(Lambertian::new(pertext.clone())))));
    world
}

pub fn earth() -> HittableList{
    let mut world = HittableList::default();
    let earth_tex = Texture::ImageTexture(ImageTexture::new("src/textures/earthmap.jpeg"));
    let earth_surface = Material::Lambertian(Lambertian::new(earth_tex));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, 0.0, 0.0), 2.0, earth_surface)));
    world
}
pub fn simple_light() -> HittableList{
    let mut world = HittableList::default();

    let pertext = Texture::NoiseTexture(NoiseTexture::new(4.0));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian(Lambertian::new(pertext.clone())))));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, 2.0, 0.0), 2.0, Material::Lambertian(Lambertian::new(pertext.clone())))));
    
    let difflight = Material::DiffuseLight(DiffuseLight::new(Texture::SolidColor(SolidColor::new(4.0, 4.0, 4.0))));
    world.add_obj(Object::XyRect(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone())));
    let difflight_pink = Material::DiffuseLight(DiffuseLight::new(Texture::SolidColor(SolidColor::new(2.0, 0.8, 0.8))));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, 7.0, 0.0), 2.0, difflight_pink)));
    world
}
pub fn cornell_box() -> HittableList{
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.12, 0.45, 0.15)));
    
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(15.0, 15.0, 15.0)));
    world.add_obj(Object::XzRect(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone())));

    world
}

pub fn cornell_ball() -> HittableList{
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.12, 0.45, 0.15)));
    
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(15.0, 15.0, 15.0)));
    world.add_obj(Object::XzRect(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone())));

    let metal = Material::Metal(Metal::new(Vec3::color(0.73, 0.53, 0.63), 0.0));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(260.0, 200.0, 260.0), 200.0,metal)));

    world


}
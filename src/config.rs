use crate::bvh::{BvhNode, Hittables};
use crate::hittable::{HittableList};
use crate::material::*;
use crate::texture::{Texture, CheckeredTexture, NoiseTexture, ImageTexture, SolidColor, UVtest};
use crate::utils::{random_double_range, random_double};
use crate::vector::Vec3;
use crate::objects::{Object, Sphere, MovingSphere, XyRect, XzRect, YzRect, BoxObject, Translate, RotateY, ConstantMedium, Triangle};
use crate::obj_models::ObjModel;
//TODO: CREATE SETUP IN HERE



pub fn test_scene() -> HittableList{
    let mut world: HittableList = HittableList::default();

    let checkered = Texture::CheckeredTexture(CheckeredTexture::new_rgb(Vec3::color(0.2, 0.3, 0.1), Vec3::color(0.9, 0.9, 0.9)));

    let material_ground = Material::Lambertian(Lambertian::new(checkered));
    let material_center =  Material::Lambertian(Lambertian::new_rgb(Vec3::color( 0.2, 0.3, 0.6)));
    //let material_center =  MatPtr(Rc::new(Dielectric{ir: 1.5}));
    let marble = Texture::NoiseTexture(NoiseTexture::new(4.0));
    //let cehckered = Texture::CheckeredTexture(CheckeredTexture::new_rgb(Vec3::color(0.78, 0.78, 0.78), Vec3::color(0.65, 0.30, 0.30)));
    let material_left = Material::Metal(Metal::new(marble, 0.0));
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
                    sphere_mat = Material::Metal(Metal::new_color(albedo, fuzz));
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
            let mat3 = Material::Metal(Metal::new_color(Vec3::color(0.7, 0.6, 0.5), 0.0));
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
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let mut box_1 = Object::BoxObject(BoxObject::new(Vec3(0.0, 0.0, 65.0), Vec3(165.0, 330.0, 1650.0), white.clone()));
    box_1 = Object::RotateY(RotateY::new_obj(box_1, 15.0));
    box_1 = Object::Translate(Translate::new_obj(box_1, Vec3(265.0, 0.0, 295.0)));
    world.add_obj(box_1);

    let mut box_2 = Object::BoxObject(BoxObject::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 165.0, 160.0), white.clone()));
    box_2 = Object::RotateY(RotateY::new_obj(box_2, -18.0));
    box_2 = Object::Translate(Translate::new_obj(box_2, Vec3(130.0, 0.0, 65.0)));
    world.add_obj(box_2);

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

    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    world.add_obj(Object::XyRect(XyRect::new_ss(0.0, 555.0, 0.0, 555.0, 0.0, white.clone(), Vec3(0.0, 0.0, 1.0))));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(15.0, 15.0, 15.0)));
    world.add_obj(Object::XzRect(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));

    let metal = Material::Metal(Metal::new_color(Vec3::color(0.73, 0.73, 0.73), 0.0));
    world.add_obj(Object::Sphere(Sphere::new(Vec3(277.5, 200.0, 277.5), 200.0,metal)));

    world

}
pub fn cornell_smoke() -> HittableList{
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.12, 0.45, 0.15)));
    
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let mut box_1 = Object::BoxObject(BoxObject::new(Vec3(0.0, 0.0, 65.0), Vec3(165.0, 330.0, 1650.0), white.clone()));
    box_1 = Object::RotateY(RotateY::new_obj(box_1, 15.0));
    box_1 = Object::Translate(Translate::new_obj(box_1, Vec3(265.0, 0.0, 295.0)));
    world.add_obj(Object::ConstantMedium(ConstantMedium::new_color(box_1, 0.002, Vec3::color(0.0, 0.0, 0.0))));

    let mut box_2 = Object::BoxObject(BoxObject::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 165.0, 160.0), white.clone()));
    box_2 = Object::RotateY(RotateY::new_obj(box_2, -18.0));
    box_2 = Object::Translate(Translate::new_obj(box_2, Vec3(130.0, 0.0, 65.0)));
    world.add_obj(Object::ConstantMedium(ConstantMedium::new_color(box_2, 0.002, Vec3::color(1.0, 1.0, 1.0))));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(7.0, 7.0, 7.0)));
    world.add_obj(Object::XzRect(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));

    world
}
pub fn final_scene() -> HittableList{
    let mut boxes1 = HittableList::default();
    let ground = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add_obj(Object::BoxObject(BoxObject::new(Vec3(x0, y0, z0), Vec3(x1, y1, z1), ground.clone())));
        }
    }

    let mut objects = HittableList::default();

    objects.add(Hittables::BvhNode(BvhNode::new(boxes1, 0.0, 1.0)));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(7.0, 7.0, 7.0)));
    objects.add_obj(Object::XzRect(XzRect::new(123.0, 432.0, 147.0, 412.0, 554.0, light)));
    
    let center_1 = Vec3(400.0, 400.0, 200.0);
    let center_2 = center_1 + Vec3(30.0, 0.0, 0.0);
    let moving_spher_material = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.7, 0.3, 0.1)));
    objects.add_obj(Object::MovingSphere(MovingSphere::new(center_1, center_2, 0.0, 1.0, 50.0, moving_spher_material)));

    objects.add_obj(Object::Sphere(Sphere::new(Vec3(260.0, 150.0, 45.0), 50.0, Material::Dielectric(Dielectric::new(1.5)))));
    objects.add_obj(Object::Sphere(Sphere::new(Vec3(0.0, 150.0, 145.0), 50.0, Material::Metal(Metal::new_color(Vec3::color(0.8, 0.8, 0.9), 1.0)))));

    let mut boundary = Object::Sphere(Sphere::new(Vec3(360.0, 150.0, 145.0), 70.0, Material::Dielectric(Dielectric::new(1.5))));
    objects.add_obj(boundary.clone());
    objects.add_obj(Object::ConstantMedium(ConstantMedium::new_color(boundary, 0.1, Vec3::color(0.2, 0.4, 0.9))));
    boundary = Object::Sphere(Sphere::new(Vec3(0.0, 0.0, 0.0), 5000.0, Material::Dielectric(Dielectric::new(1.5))));
    objects.add_obj(Object::ConstantMedium(ConstantMedium::new_color(boundary, 0.0001, Vec3::color(1.0, 1.0, 1.0))));

    let emat = Material::Lambertian(Lambertian::new(Texture::ImageTexture(ImageTexture::new("src/textures/earthmap.jpeg"))));
    objects.add_obj(Object::Sphere(Sphere::new(Vec3(400.0, 200.0, 400.0), 100.0, emat)));
    let pertext = Texture::NoiseTexture(NoiseTexture::new(0.1));
    objects.add_obj(Object::Sphere(Sphere::new(Vec3(220.0, 280.0, 300.0), 80.0, Material::Lambertian(Lambertian::new(pertext)))));

    let mut boxes2 = HittableList::default();
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns{
        boxes2.add_obj(Object::Sphere(Sphere::new(Vec3::random_range(0.0, 165.0), 10.0, white.clone())));
    }
    objects.add_obj(
        Object::Translate(
            Translate::new_obj(
            Object::RotateY(
                    RotateY::new(
                        Hittables::BvhNode(
                            BvhNode::new(boxes2, 0.0, 1.0)), 15.0)),
                             Vec3(-100.0, 270.0, 395.0))));
    objects
}

pub fn cornell_triangle() -> HittableList{
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.12, 0.45, 0.15)));
    
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));


    let a = Vec3(455.0, 255.0, 450.0);
    let b = Vec3(455.0, 20.0, 150.0);
    let c = Vec3(100.0, 20.0, 150.0);
    let triag = Object::Triangle(Triangle::new(a, b, c, Material::Lambertian(Lambertian::new(Texture::UVtest(UVtest::new())))));
    world.add_obj(triag);

    //world.add_obj(Object::XyRect(XyRect::new_ss(0.0, 555.0, 0.0, 555.0, 0.0, white.clone(), Vec3(0.0, 0.0, 1.0))));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(7.0, 7.0, 7.0)));
    world.add_obj(Object::XzRect(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));

    world
}

pub fn cornell_chess() -> HittableList{
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.12, 0.45, 0.15)));
    
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add_obj(Object::YzRect(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add_obj(Object::XzRect(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add_obj(Object::XyRect(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));


    //let chess_mat = Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73)));
    let chess_mat = Material::Metal(Metal::new_color(Vec3(0.73, 0.73, 0.73), 0.0));
    let obj = ObjModel::new("src/models/queen-low-poly.obj", chess_mat, 60.0);
    let object_bhv = Hittables::BvhNode(BvhNode::new(obj.primitives, 0.0, 1.0));
    let model = Object::Translate(Translate::new(object_bhv, Vec3(277.5, 0.0, 277.5)));

    world.add_obj(model);

    world.add_obj(Object::XyRect(XyRect::new_ss(0.0, 555.0, 0.0, 555.0, 0.0, white.clone(), Vec3(0.0, 0.0, 1.0))));

    let light = Material::DiffuseLight(DiffuseLight::new_color(Vec3::color(7.0, 7.0, 7.0)));
    world.add_obj(Object::XzRect(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));

    world
}

pub fn obj_test() -> HittableList{
    let mut world = HittableList::default();
    let obj = ObjModel::new("src/models/queen-low-poly.obj", Material::Lambertian(Lambertian::new_rgb(Vec3::color(0.73, 0.73, 0.73))), 1.0);
    world.add(Hittables::BvhNode(BvhNode::new(obj.primitives, 0.0, 1.0)));
    world

}

//eventually render this: https://sketchfab.com/3d-models/low-poly-chess-set-0f440e2b01ca42f8b3fdee8178c51f20
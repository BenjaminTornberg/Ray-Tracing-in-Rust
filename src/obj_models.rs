use crate::hittable::Hittable;

#[cfg(test)]
use crate::material::Lambertian;
use crate::objects::{Triangle, Object};
use crate::vector::Vec3;
use crate::{material::Material, hittable::HittableList};
use tobj;

#[derive(Debug, Clone)]
pub struct ObjModel{
    pub primitives: HittableList
}
impl ObjModel {
    pub fn new(file: &str, material: Material, scale: f64) -> ObjModel{
        let file = tobj::load_obj(file, &tobj::GPU_LOAD_OPTIONS);
        
        let (models, _) = file.expect("Failed to load OBJ");

        let mesh = &models[0].mesh;
        let mut primitives = HittableList::default();
        for i in (0..mesh.indices.len()).step_by(3){
            let a = Vec3(mesh.positions[3 * mesh.indices[i] as usize] as f64, mesh.positions[3 * mesh.indices[i] as usize + 1] as f64, mesh.positions[3 * mesh.indices[i] as usize + 2] as f64) * scale;
            let b = Vec3(mesh.positions[3 * mesh.indices[i+1] as usize] as f64, mesh.positions[3 * mesh.indices[i+1] as usize + 1] as f64, mesh.positions[3 * mesh.indices[i+1] as usize + 2] as f64) * scale;
            let c = Vec3(mesh.positions[3 * mesh.indices[i+2] as usize] as f64, mesh.positions[3 * mesh.indices[i+2] as usize + 1] as f64, mesh.positions[3 * mesh.indices[i+2] as usize + 2] as f64) * scale;
            if !mesh.normals.is_empty(){
                let normal = Vec3(mesh.normals[3 * mesh.indices[i] as usize] as f64, mesh.normals[3 * mesh.indices[i] as usize + 1] as f64, mesh.normals[3 * mesh.indices[i] as usize + 2] as f64) * scale;
                //add texcoords eventualy
                primitives.add_obj(Object::Triangle(Triangle::new_normal(a, b, c, normal, material.clone())));
                continue
            }
            primitives.add_obj(Object::Triangle(Triangle::new(a, b, c, material.clone())));
        }
        ObjModel { primitives }
    }
}
impl Hittable for ObjModel{
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::Aabb> {
        self.primitives.bounding_box(time0, time1)
    }
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64)->  Option<crate::hittable::HitRecord> {
        self.primitives.hit(r, t_min, t_max)
    }
}

#[test]
fn test_obj(){
    let model = ObjModel::new("src/models/simple-model.obj", Material::Lambertian(Lambertian::new_rgb(Vec3(0.0, 0.0, 0.0))), 1.0);
    eprintln!("{:?}", model.primitives);
    // /assert_eq!(model.primitives.objects.len(), model.t/3);
}
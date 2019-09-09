use crate::materials::Material;
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;
pub use scene::Scene;
use std::sync::Arc;

pub mod camera;
pub mod scene;

pub struct HitRecord {
    pub p: Vec3,
    pub t: f32,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

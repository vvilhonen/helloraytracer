mod common;
mod dielectric;
mod diffuse;
mod metal;

use crate::scene::HitRecord;
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;
pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use metal::Metal;
use std::fmt::Debug;
use std::sync::Arc;

pub struct Scatter {
    pub attenuation: Option<Vec3>,
    pub scattered: Ray,
}

pub trait Material: Debug {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

pub type MaterialArc = Arc<dyn Material + Send + Sync>;

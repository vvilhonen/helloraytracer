use crate::materials::{Material, Scatter};
use crate::scene::HitRecord;
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Diffuse(pub Vec3);

impl Material for Diffuse {
    fn scatter(&self, _r: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let HitRecord { normal, p, .. } = hit;
        let target = *p + *normal + Vec3::random_in_unit_sphere();
        Some(Scatter {
            attenuation: Some(self.0),
            scattered: Ray::new(*p, target),
        })
    }
}

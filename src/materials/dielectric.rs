use crate::materials::common::schlick;
use crate::materials::{Material, Scatter};
use crate::scene::HitRecord;
use crate::util::rand::randf;
use crate::util::ray::Ray;

#[derive(Clone, Debug)]
pub struct Dielectric(pub f32);

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let HitRecord { normal, p, .. } = hit;

        let uv = r.direction().unit();
        let dot = uv.dot(normal);

        let (cosine, outward_normal, ratio) = if uv.dot(normal) > 0.0 {
            (
                r.direction().dot(normal) * self.0 / r.direction().length(),
                -*normal,
                self.0,
            )
        } else {
            (
                -r.direction().dot(normal) / r.direction().length(),
                *normal,
                1.0 / self.0,
            )
        };

        let outward_dot = uv.dot(&outward_normal);
        let discriminant = 1.0 - ratio * ratio * (1.0 - outward_dot * outward_dot);

        let scattered = if discriminant > 0.0 && randf() > schlick(cosine, self.0) {
            ratio * (uv - outward_normal * outward_dot) - outward_normal * discriminant.sqrt()
        } else {
            uv - 2.0 * dot * *normal
        };

        Some(Scatter {
            attenuation: None,
            scattered: Ray::new(*p, scattered),
        })
    }
}

use crate::materials::{Material, Scatter};
use crate::scene::HitRecord;
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Metal(pub Vec3);

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let HitRecord { normal, p, .. } = hit;

        let uv = r.direction().unit();
        let dot = uv.dot(normal);
        let reflected = uv - 2.0 * dot * *normal;
        let scattered = Ray::new(*p, reflected);

        if scattered.direction().dot(normal) > 0.0 {
            Some(Scatter {
                attenuation: Some(self.0),
                scattered,
            })
        } else {
            None
        }
    }
}

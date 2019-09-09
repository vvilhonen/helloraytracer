use crate::materials::MaterialArc;
use crate::scene::{HitRecord, Hitable};
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: MaterialArc,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialArc) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn collides(&self, other: &Self) -> bool {
        (self.center - other.center).length() < self.radius + other.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;

        let a = r.direction().dot(r.direction());
        let b = r.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant <= 0.0 {
            None
        } else {
            let first_t = (-b - discriminant.sqrt()) / a;
            if first_t < t_max && first_t > t_min {
                return Some(HitRecord {
                    p: r.point_at_param(first_t),
                    t: first_t,
                    normal: (r.point_at_param(first_t) - self.center).unit(),
                    material: self.material.clone(),
                });
            }

            let second_t = (-b + discriminant.sqrt()) / a;
            if second_t < t_max && second_t > t_min {
                return Some(HitRecord {
                    p: r.point_at_param(second_t),
                    t: second_t,
                    normal: (r.point_at_param(second_t) - self.center).unit(),
                    material: self.material.clone(),
                });
            }

            None
        }
    }
}

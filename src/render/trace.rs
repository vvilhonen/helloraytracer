use crate::scene::{Hitable, Scene};
use crate::util::ray::Ray;
use crate::util::vec3::Vec3;

pub fn trace(r: &Ray, scene: &Scene, depth: usize, max_depth: usize) -> Option<Vec3> {
    if depth > max_depth {
        return None;
    }
    if let Some(hit) = scene.hit(r, 0.001, std::f32::MAX) {
        match hit.material.scatter(r, &hit) {
            Some(ref scatter) => {
                let attenuation = scatter.attenuation.unwrap_or_else(|| Vec3(1.0, 1.0, 1.0));
                trace(&scatter.scattered, scene, depth + 1, max_depth).map(|c| attenuation * c)
            }
            _ => Some(Vec3(0.0, 0.0, 0.0)),
        }
    } else {
        let y = r.direction().unit().y();
        let t = 0.5 * (y + 1.0);
        Some((1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0))
    }
}

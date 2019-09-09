use crate::scene::{HitRecord, Hitable};
use crate::util::ray::Ray;

pub type HitBox = Box<dyn Hitable + Sync + Send>;

pub struct Scene {
    hitables: Vec<HitBox>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            hitables: Vec::new(),
        }
    }

    pub fn push(&mut self, hitable: HitBox) {
        self.hitables.push(hitable);
    }

    pub fn extend<T: IntoIterator<Item = HitBox>>(&mut self, addition: T) {
        self.hitables.extend(addition);
    }
}

impl Hitable for Scene {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut ret = None;
        for obj in self.hitables.iter() {
            if let Some(res) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = res.t;
                ret = Some(res);
            }
        }
        ret
    }
}

use crate::util::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    #[inline]
    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    #[inline]
    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    #[inline]
    pub fn point_at_param(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}

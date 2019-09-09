use crate::util::ray::Ray;
use crate::util::vec3::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    original_look_from: Vec3,
    look_from: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    look_at: Vec3,
    up: Vec3,
    theta: f32,
    aspect: f32,
    lower_left_corner: Vec3,
    rotation: usize,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        Self {
            original_look_from: look_from,
            look_from,
            look_at,
            up,
            theta,
            aspect,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            rotation: 0,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let a = self.look_from;
        let b = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.look_from;

        Ray::new(a, b)
    }

    pub fn rotate_around_y(&mut self, degrees: usize) {
        self.rotation = (self.rotation + degrees) % 360;
        let angle_rad = self.rotation as f32 * (PI / 180.0);

        let look_from = self.original_look_from.rotate_around_y(angle_rad)
            + Vec3(0.0, angle_rad.sin() / 2.0, -angle_rad.sin() / 2.0);
        let half_height = (self.theta / 2.0).tan();
        let half_width = self.aspect * half_height;
        let w = (look_from - self.look_at).unit();
        let u = self.up.cross(&w).unit();
        let v = w.cross(&u);

        self.look_from = look_from;
        self.horizontal = 2.0 * half_width * u;
        self.vertical = 2.0 * half_height * v;
        self.lower_left_corner = self.look_from - half_width * u - half_height * v - w;
    }
}

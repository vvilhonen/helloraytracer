use crate::geometry::sphere::Sphere;
use crate::materials::{Dielectric, Diffuse, MaterialArc, Metal};
use crate::scene::camera::Camera;
use crate::scene::scene::HitBox;
use crate::scene::Scene;
use crate::util::rand::{normal_randf, randf};
use crate::util::vec3::Vec3;
use std::sync::Arc;

pub struct Setup {
    pub nx: usize,
    pub ny: usize,
    pub ns: usize,
    pub max_depth: usize,
    pub scene: Scene,
    pub camera: Camera,
}

impl Setup {
    pub fn from_render_opts(
        debug_view: bool,
        width: usize,
        height: usize,
        samples: usize,
        max_depth: usize,
    ) -> Self {
        if debug_view {
            Setup::default_debug(width, height, samples, max_depth)
        } else {
            Setup::default(width, height, samples, max_depth)
        }
    }

    pub fn default(nx: usize, ny: usize, ns: usize, max_depth: usize) -> Self {
        Self {
            nx,
            ny,
            ns,
            max_depth,
            scene: Self::default_scene(),
            camera: Self::default_camera(nx, ny),
        }
    }

    pub fn default_debug(nx: usize, ny: usize, ns: usize, max_depth: usize) -> Self {
        Self {
            nx,
            ny,
            ns,
            max_depth,
            scene: Self::debug_scene(),
            camera: Self::default_camera(nx, ny),
        }
    }

    pub fn debug_scene() -> Scene {
        macro_rules! add_sphere {
            ($container:ident, $center:expr, $radius:expr, $material:expr) => {
                let sphere = Sphere::new($center, $radius, Arc::new($material));
                $container.push(Box::new(sphere));
            };
        }

        let mut scene = Scene::new();

        add_sphere!(
            scene,
            Vec3(0.0, 0.0, 0.0),
            0.5,
            Diffuse(Vec3(0.1, 0.2, 0.5))
        );
        add_sphere!(
            scene,
            Vec3(0.0, -100.5, 0.0),
            100.0,
            Diffuse(Vec3(0.8, 0.8, 0.0))
        );
        add_sphere!(scene, Vec3(1.0, 0.0, 0.0), 0.5, Metal(Vec3(0.8, 0.6, 0.2)));
        add_sphere!(scene, Vec3(-1.0, 0.0, 0.0), 0.5, Dielectric(1.5));

        scene
    }

    pub fn default_scene() -> Scene {
        let mut spheres: Vec<Sphere> = Vec::new();

        spheres.push(Sphere::new(
            Vec3(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Diffuse(Vec3(0.0, 0.5, 0.5))),
        ));

        for _ in 0..30 {
            let radius = normal_randf(0.15, 0.01);
            let material = {
                let color = Vec3(randf(), randf(), randf());
                let material_choose = randf();
                if material_choose > 0.7 {
                    Arc::new(Dielectric(1.3)) as MaterialArc
                } else if material_choose > 0.6 {
                    Arc::new(Metal(color)) as MaterialArc
                } else {
                    Arc::new(Diffuse(color)) as MaterialArc
                }
            };

            let sphere = 'next_sphere: loop {
                let candidate = {
                    let center = Vec3(
                        normal_randf(0.0, 0.5),
                        normal_randf(0.5, 0.2),
                        normal_randf(0.0, 0.4),
                    );
                    Sphere::new(center, radius, material.clone())
                };
                for existing in spheres.iter() {
                    if existing.collides(&candidate) {
                        continue 'next_sphere;
                    }
                }

                break candidate;
            };
            spheres.push(sphere);
        }

        let mut scene = Scene::new();
        scene.extend(spheres.into_iter().map(|x| Box::new(x) as HitBox));
        log::info!("Generated scene");

        scene
    }

    pub fn default_camera(nx: usize, ny: usize) -> Camera {
        let look_from = Vec3(1.0, 1.0, -2.0);
        let look_at = Vec3(0.0, 0.0, 0.0);
        let up = Vec3(0.0, 1.0, 0.0);
        Camera::new(look_from, look_at, up, 60.0, nx as f32 / ny as f32)
    }
}

use crate::render::trace::trace;
use crate::util::rand::randf;
use crate::util::vec3::Vec3;
use crate::{Frame, Setup};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn render(setup: &Setup) -> Frame {
    let Setup { nx, ny, .. } = setup;

    let buffer = (0..*ny)
        .flat_map(|y| (0..*nx).map(move |x| (x, y)))
        .collect::<Vec<_>>()
        .into_par_iter()
        .fold(
            || Vec::new(),
            |mut acc, (x, y)| {
                let Vec3(r, g, b) = render_pixel(x, y, setup);
                let (r, g, b) = ((255.99 * r) as u8, (255.99 * g) as u8, (255.99 * b) as u8);
                acc.extend(&[r, g, b]);
                acc
            },
        )
        .flatten()
        .collect();

    Frame::new(*nx, *ny, buffer)
}

fn render_pixel(x: usize, y: usize, setup: &Setup) -> Vec3 {
    let Setup {
        nx,
        ny,
        ns,
        max_depth,
        camera,
        scene,
    } = setup;

    let mut col = Vec3(0.0, 0.0, 0.0);
    for _ in 0..*ns {
        let u = (x as f32 + randf()) / *nx as f32;
        let v = (y as f32 + randf()) / *ny as f32;
        let ray = camera.ray(u, v);
        col = col + trace(&ray, &scene, 0, *max_depth).unwrap_or_else(|| Vec3(0.0, 0.0, 1.0));
    }
    let Vec3(r, g, b) = col / *ns as f32;
    Vec3(r.sqrt(), g.sqrt(), b.sqrt())
}

use rand::{thread_rng, Rng};
use rand_distr::Normal;

pub fn randf() -> f32 {
    rand::random()
}

pub fn normal_randf(mean: f32, std: f32) -> f32 {
    thread_rng().sample(Normal::new(mean, std).unwrap())
}

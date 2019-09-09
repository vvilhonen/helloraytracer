pub fn schlick(cosine: f32, refraction_idx: f32) -> f32 {
    let r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

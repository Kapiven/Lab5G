use crate::vec3::Vec3;

pub fn to_u32(c: Vec3) -> u32 {
    let r = (c.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (c.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (c.z.clamp(0.0, 1.0) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}

fn clamp01(x: f32) -> f32 {
    if x.is_nan() { 0.0 } else { x.max(0.0).min(1.0) }
}

use crate::vec3::Vec3;

pub fn hash1(mut n: u32) -> f32 {
    n = (n ^ 61).wrapping_add(n << 3);
    n = n ^ (n >> 4);
    n = n.wrapping_mul(0x27d4eb2d);
    n = n ^ (n >> 15);
    (n as f32) / (u32::MAX as f32)
}

pub fn noise3(p: Vec3) -> f32 {
    // coarse hashing of coordinates
    let xi = (p.x * 12.9898) as i32;
    let yi = (p.y * 78.233) as i32;
    let zi = (p.z * 37.719) as i32;
    let mut n = xi as u32;
    n = n.wrapping_mul(73856093) ^ ((yi as u32).wrapping_mul(19349663)) ^ ((zi as u32).wrapping_mul(83492791));
    hash1(n)
}

pub fn fbm(p: Vec3, octaves: usize) -> f32 {
    let mut sum = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    for _ in 0..octaves {
        sum += noise3(p * freq) * amp;
        freq *= 2.0;
        amp *= 0.5;
    }
    sum
}

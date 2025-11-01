// src/ray.rs
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir: dir.normalize() }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}

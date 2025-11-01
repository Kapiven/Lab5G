// src/vec3.rs
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x:f32,y:f32,z:f32)->Self{Self{x,y,z}}
    pub fn zero() -> Self { Self::new(0.0,0.0,0.0) }
    pub fn dot(self, o: Self) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l == 0.0 { self } else { self / l }
    }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y*o.z - self.z*o.y,
            self.z*o.x - self.x*o.z,
            self.x*o.y - self.y*o.x
        )
    }
    pub fn mul_scalar(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s) }
    pub fn clamp(self, a: f32, b: f32) -> Self {
        let c = |v:f32| v.max(a).min(b);
        Self::new(c(self.x), c(self.y), c(self.z))
    }
}

impl Add for Vec3 { type Output = Self; fn add(self, o: Self) -> Self { Self::new(self.x+o.x,self.y+o.y,self.z+o.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, o: Self) -> Self { Self::new(self.x-o.x,self.y-o.y,self.z-o.z) } }
impl Mul for Vec3 { type Output = Self; fn mul(self, o: Self) -> Self { Self::new(self.x*o.x,self.y*o.y,self.z*o.z) } }
impl Div for Vec3 { type Output = Self; fn div(self, o: Self) -> Self { Self::new(self.x/o.x,self.y/o.y,self.z/o.z) } }
impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x,-self.y,-self.z) } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s,self.y*s,self.z*s) } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s,self.y/s,self.z/s) } }

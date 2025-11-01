// src/sphere.rs
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::noise::{fbm, noise3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub is_light: bool,
    pub kind: SphereKind,
    pub rotate_speed: f32, 
}

#[derive(Copy, Clone)]
pub enum SphereKind {
    Star,
    Rocky,
    GasGiant,
    Moon,
}

pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub kind: SphereKind,
    pub color: Vec3,
    pub emissive: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius:f32, kind: SphereKind, is_light: bool, rotate_speed: f32) -> Self {
        Sphere { center, radius, is_light, kind, rotate_speed }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.orig - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius*self.radius;
        let disc = b*b - 4.0*a*c;
        if disc < 0.0 { return None; }
        let sq = disc.sqrt();
        let t1 = (-b - sq) / (2.0*a);
        let t2 = (-b + sq) / (2.0*a);
        let t = if t1>0.001 { t1 } else if t2>0.001 { t2 } else { return None; };
        Some(t)
    }

    pub fn shade(&self, p: Vec3, normal: Vec3, time: f32) -> (Vec3, Vec3) {
        // returns (diffuse_color, emissive)
        match self.kind {
            SphereKind::Star => self.shader_star(p, normal, time),
            SphereKind::Rocky => self.shader_rocky(p, normal, time),
            SphereKind::GasGiant => self.shader_gas(p, normal, time),
            SphereKind::Moon => self.shader_moon(p, normal, time),
        }
    }

    fn shader_star(&self, p: Vec3, _n: Vec3, time: f32) -> (Vec3, Vec3) {
        // radial emissive gradient + subtle noise rays
        let to_center = (p - self.center).normalize();
        let r = (p - self.center).length() / self.radius;
        // radial falloff
        let glow = (1.0 - r).max(0.0).powf(1.5);
        // base color yellowish with flicker noise
        let t = time * 0.8;
        let flick = 0.8 + 0.4 * noise3(Vec3::new(p.x*3.0 + t, p.y*3.0, p.z*3.0));
        let emissive = Vec3::new(1.0, 0.85, 0.5) * glow * flick * 2.5;
        // visible surface color (tiny)
        let surf = Vec3::new(1.0, 0.9, 0.6) * (0.4 + 0.6*glow);
        (surf, emissive)
    }

    fn shader_rocky(&self, p: Vec3, n: Vec3, time: f32) -> (Vec3, Vec3) {
        // compute spherical coords (latitude / longitude) relative to center
        let local = (p - self.center).normalize();
        let lat = local.y.asin(); // -pi/2..pi/2
        let lon = local.z.atan2(local.x);
        let mut base = Vec3::new(0.32, 0.24, 0.18); // rock base
        // layer 1: color variation by fbm
        let h = fbm(local * 3.0 + Vec3::new(time*0.05,0.0,0.0), 4) * 0.5;
        base = base + Vec3::new(h*0.15, h*0.1, h*0.05);
        // layer 2: bands / continents using sin lat+lon
        let band = ( (lat*6.0 + lon*2.0 + time*0.2).sin() * 0.5 + 0.5 ).powf(1.3);
        base = base * (0.7 + 0.6*band);
        // layer 3: craters: place darker circular spots using noise and threshold
        let crater_noise = noise3(local * 12.0);
        let mut crater_mask = 0.0;
        if crater_noise > 0.7 {
            crater_mask = (crater_noise - 0.7) / 0.3;
        }
        let crater_color = base * 0.5;
        let color = base*(1.0-crater_mask) + crater_color*crater_mask;
        // small specular highlight
        let rim = (1.0 - (n.dot(Vec3::new(0.0,1.0,0.0))).abs()).powf(3.0) * 0.2;
        let final_color = color + Vec3::new(0.05,0.05,0.06) * rim;
        (final_color, Vec3::new(0.0,0.0,0.0))
    }

    fn shader_gas(&self, p: Vec3, n: Vec3, time: f32) -> (Vec3, Vec3) {
    let local = (p - self.center).normalize();
    let lat = local.y;

    // Bandas atmosféricas
    let base = Vec3::new(0.45, 0.55, 0.85);
    let band = 0.5 + 0.5 * (lat * 10.0 + time * 0.5).sin();
    let band_color = base * (0.6 + 0.8 * band);

    // Swirls
    let swirl = fbm(local * 6.0 + Vec3::new(0.0, time*0.3, 0.0), 5) * 0.25;
    let color = band_color + Vec3::new(0.05, 0.08, 0.12) * swirl;

    // ANILLOS PROCEDURALES
    let rp = p - self.center;
    let dist = (rp.x*rp.x + rp.z*rp.z).sqrt();

    // Radio posible del anillo alrededor del gas giant
    let inner = self.radius * 1.4;
    let outer = self.radius * 2.4;

    let mut ring = 0.0;
    if dist > inner && dist < outer {
        ring = ((dist - inner) / (outer - inner) * 30.0).sin().abs();
    }

    let ring_color = Vec3::new(0.85, 0.8, 0.7) * ring;
    let final_color = color + ring_color * 1.5;

    (final_color, Vec3::zero())
}

    fn shader_moon(&self, p: Vec3, n: Vec3, _time: f32) -> (Vec3, Vec3) {
        // simple gray with crater noise
        let local = (p - self.center).normalize();
        let base = Vec3::new(0.7, 0.7, 0.75);
        let noise = fbm(local * 10.0, 4);
        let color = base * (0.6 + 0.6*noise);
        (color, Vec3::zero())
    }
}

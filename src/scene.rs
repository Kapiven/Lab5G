// src/scene.rs
use rayon::prelude::*;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::{Sphere, SphereKind};
use crate::color::to_u32;

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub camera_pos: Vec3,
    pub spheres: Vec<Sphere>,
    pub fov: f32,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let camera_pos = Vec3::new(0.0, 0.0, -9.0);

        let spheres = vec![
            // Estrella (luz)
            Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.4, SphereKind::Star, true, 0.0),

            // Planeta Rocoso
            Sphere::new(Vec3::new(-3.0, 0.0, 1.0), 1.0, SphereKind::Rocky, false, 0.2),

            // Luna (opcional, pequeña)
            Sphere::new(Vec3::new(-2.2, 0.0, 1.6), 0.28, SphereKind::Moon, false, 0.9),

            // Gigante Gaseoso (con anillos calculados proceduralmente)
            Sphere::new(Vec3::new(3.0, 0.5, 1.5), 1.0, SphereKind::GasGiant, false, 1.0),
        ];

        Scene { width, height, camera_pos, spheres, fov: 1.0 }
    }

    pub fn trace(&self, ray: &Ray, time: f32) -> Vec3 {
        // 1) buscar la intersección de esferas más cercana
        let mut nearest_t = f32::INFINITY;
        let mut hit_sphere: Option<&Sphere> = None;

        for s in &self.spheres {
            if let Some(t) = s.intersect(ray) {
                if t < nearest_t {
                    nearest_t = t;
                    hit_sphere = Some(s);
                }
            }
        }

        // 2) comprobar intersección con anillo 
        if self.spheres.len() > 3 {
            let gas = &self.spheres[3];
            let ring_center = gas.center;
            // plano del anillo: normal inclinada 
            let ring_normal = Vec3::new(0.0, 2.0, 0.26).normalize();

            // intersectar rayo con plano del anillo
            let denom = ray.dir.dot(ring_normal);
            if denom.abs() > 1e-6 {
                let t_plane = (ring_center - ray.orig).dot(ring_normal) / denom;
                if t_plane > 0.0 && t_plane < nearest_t {
                    let hit_point = ray.at(t_plane);
                    // proyectar punto al plano del anillo (sustrayendo componente normal)
                    let v = hit_point - ring_center;
                    let v_plane = v - ring_normal * v.dot(ring_normal);
                    let r = v_plane.length();

                    // radios del anillo relativos al radio del planeta gaseoso
                    let inner = gas.radius * 1.35;
                    let outer = gas.radius * 2.6;

                    if r > inner && r < outer {
                        // patrón bandado para el anillo (sin texturas)
                        let bands = ((r - inner) / (outer - inner) * 40.0).sin().abs();
                        // ligera variación radial para dar detalle
                        let band_mask = 0.5 + 0.5 * bands;
                        // color base del anillo y mezcla
                        let ring_base = Vec3::new(0.86, 0.82, 0.72); // arena clara
                        let ring_dark = Vec3::new(0.35, 0.32, 0.30); // bandas oscuras

                        let mut ring_color = ring_base * (0.6 + 0.8 * band_mask) + ring_dark * (0.25 * (1.0 - band_mask));
                        // añadir un brillo más fuerte en la parte iluminada por la estrella(s)
                        // calcular iluminación simple desde luces (mismas luces que para esferas)
                        let mut lighting = Vec3::zero();
                        for light in &self.spheres {
                            if light.is_light {
                                let to_light = (light.center - hit_point);
                                let light_dist = to_light.length();
                                let ldir = to_light.normalize();

                                // sombra
                                let shadow_origin = hit_point + ring_normal * 0.0005;
                                let shadow_ray = Ray::new(shadow_origin, ldir);
                                let mut in_shadow = false;
                                for other in &self.spheres {
                                    if std::ptr::eq(other, light) { continue; }
                                    if let Some(t_sh) = other.intersect(&shadow_ray) {
                                        if t_sh < light_dist { in_shadow = true; break; }
                                    }
                                }
                                if !in_shadow {
                                    let lam = ring_normal.dot(ldir).max(0.0);
                                    // atenuación
                                    let att = 1.0 / (0.5 + 0.1 * light_dist * light_dist);
                                    // tomar emisivo de la luz (star shader)
                                    let (_ls, lemi) = light.shade(light.center, Vec3::new(0.0,1.0,0.0), time);
                                    lighting = lighting + lemi * lam * att;
                                }
                            }
                        }

                        // mezclar color con lighting
                        ring_color = ring_color * (Vec3::new(0.12, 0.12, 0.12) + lighting * 1.8);
                        // un poco de brillo especular sutil basado en la orientación
                        let view = (self.camera_pos - hit_point).normalize();
                        let half = (view + (self.spheres[0].center - hit_point).normalize()).normalize();
                        let spec = ring_normal.dot(half).max(0.0).powf(8.0) * 0.25;
                        ring_color = ring_color + Vec3::new(spec, spec, spec);

                        // gamma y clamp
                        let final_ring = Vec3::new(ring_color.x.sqrt(), ring_color.y.sqrt(), ring_color.z.sqrt()).clamp(0.0, 1.0);
                        return final_ring;
                    }
                }
            }
        }

        // 3) si no golpeó anillo antes, proceder como antes con la esfera más cercana
        if hit_sphere.is_none() {
            let tbg = 0.5 * (ray.dir.y + 1.0);
            return Vec3::new(0.05, 0.05, 0.08)*(1.0 - tbg) + Vec3::new(0.02, 0.03, 0.06)*tbg;
        }

        let s = hit_sphere.unwrap();
        let p = ray.at(nearest_t);
        let n = (p - s.center).normalize();

        // obtener color base y componente emisiva desde el shader de la esfera
        let (surf_col, surf_emissive) = s.shade(p, n, time);

        // iluminación: sumar contribución de cada luz (esferas con is_light = true)
        let mut lighting = Vec3::zero();
        for light in &self.spheres {
            if light.is_light {
                let to_light = (light.center - p);
                let light_dist = to_light.length();
                let ldir = to_light.normalize();

                // rayito hacia la luz 
                let shadow_origin = p + n * 0.001;
                let shadow_ray = Ray::new(shadow_origin, ldir);

                // verificar sombras
                let mut in_shadow = false;
                for other in &self.spheres {
                    // saltar la propia luz
                    if std::ptr::eq(other, light) { continue; }
                    if let Some(t) = other.intersect(&shadow_ray) {
                        if t < light_dist { in_shadow = true; break; }
                    }
                }

                if !in_shadow {
                    // Lambert
                    let lam = n.dot(ldir).max(0.0);
                    // atenuación por distancia (simple)
                    let att = 1.0 / (0.5 + 0.1 * light_dist * light_dist);
                    // la propia luz puede aportar color emisivo (shader de la luz)
                    let (_l_surf, l_emissive) = light.shade(light.center, Vec3::new(0.0, 1.0, 0.0), time);
                    lighting = lighting + l_emissive * lam * att;

                    // specular pequeño
                    let view = (self.camera_pos - p).normalize();
                    let half = (view + ldir).normalize();
                    let spec = n.dot(half).max(0.0).powf(40.0) * 0.2;
                    lighting = lighting + Vec3::new(1.0, 1.0, 1.0) * spec * att;
                }
            }
        }

        // ambient
        let ambient = Vec3::new(0.06, 0.06, 0.07);

        let final_color = surf_col * (ambient + lighting) + surf_emissive;

        // gamma
        Vec3::new(final_color.x.sqrt(), final_color.y.sqrt(), final_color.z.sqrt()).clamp(0.0, 1.0)
    }

    pub fn render(&mut self, buffer: &mut [u32], time: f32) {
        let width = self.width;
        let height = self.height;
        let aspect = width as f32 / height as f32;
        let half_height = (self.fov / 2.0).tan();
        let half_width = aspect * half_height;

        let t = time * 0.9; // velocidad general

        // ORBITAS y posiciones animadas
        if self.spheres.len() >= 4 {
            let sun = self.spheres[0].center;

            // Planeta rocoso (índice 1) órbita circular alrededor del sol
            let rock_orbit_radius = 3.0;
            self.spheres[1].center = Vec3::new(
                sun.x + rock_orbit_radius * (t * 1.0).cos(),
                sun.y,
                sun.z + rock_orbit_radius * (t * 1.0).sin()
            );

            // Luna (índice 2) orbita al planeta rocoso
            let rock = self.spheres[1].center;
            self.spheres[2].center = Vec3::new(
                rock.x + 1.4 * (t * 2.2).cos(),
                rock.y + 0.65 * (t * 1.6).sin(),
                rock.z + 0.9 * (t * 2.2).sin()
            );

            // Gigante gaseoso (índice 3) órbita más lenta, más lejano
            let gas_orbit_radius = 6.0;
            self.spheres[3].center = Vec3::new(
                sun.x + gas_orbit_radius * (t * 0.4).cos(),
                sun.y - 0.6,
                sun.z + gas_orbit_radius * (t * 0.4).sin()
            );
        }

        // render en paralelo: cada chunk será una fila de width píxeles
        buffer.par_chunks_mut(width).enumerate().for_each(|(j, row)| {
            let y = j;
            for i in 0..width {
                let px = (2.0 * ((i as f32 + 0.5) / width as f32) - 1.0) * half_width;
                let py = (1.0 - 2.0 * ((y as f32 + 0.5) / height as f32)) * half_height;

                let dir = Vec3::new(px, py, 1.0).normalize();
                let ray = Ray::new(self.camera_pos, dir);
                let col = self.trace(&ray, time);
                row[i] = to_u32(col);
            }
        });
    }
}

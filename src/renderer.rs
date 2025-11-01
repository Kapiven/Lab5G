use glam::Vec3;
use image::{Rgb, RgbImage};
use crate::shader::shade;
use crate::scene::{SolarSystem, Planet};

pub fn render_frame(system: &SolarSystem, _time: f32) -> RgbImage {
    let size = system.size;
    let mut img = RgbImage::new(size, size);
    let bg = Rgb([5, 5, 15]); // fondo oscuro

    // fondo
    for y in 0..size {
        for x in 0..size {
            img.put_pixel(x, y, bg);
        }
    }

    // renderiza cada planeta
    for planet in &system.planets {
        render_planet(&mut img, planet);
    }

    img
}

fn render_planet(img: &mut RgbImage, planet: &Planet) {
    let size = img.width();
    let (crot, srot) = (planet.rotation.cos(), planet.rotation.sin());

    for y in 0..size {
        for x in 0..size {
            let pos = Vec3::new(x as f32, y as f32, 0.0);
            let dir = pos - planet.position;
            let dist = dir.length();

            if dist <= planet.radius {
                // rotación en Z manual
                let v = dir / planet.radius;
                let nx = v.x * crot - v.y * srot;
                let ny = v.x * srot + v.y * crot;
                let nz = v.z;
                let n = Vec3::new(nx, ny, nz);

                let color = shade(
                    planet.color_shader,
                    n,
                    dist / planet.radius,
                    pos,
                    planet.position,
                    size,
                );
                img.put_pixel(x, y, color);
            }
        }
    }
}

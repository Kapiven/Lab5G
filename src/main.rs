mod vec3;
mod ray;
mod color;
mod noise;
mod sphere;
mod scene;

use minifb::{WindowOptions, Window, Key};
use crate::scene::Scene;
use std::time::Instant;

fn main() {
    let mut width = 1024;
    let mut height = 640;

    let mut window = Window::new(
        "Planet Shaders - CPU Renderer",
        width,
        height,
        WindowOptions::default()
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    let mut buffer = vec![0u32; width * height];
    let mut scene = Scene::new(width, height);

    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t = (Instant::now() - start).as_secs_f32();

        // Actualización por si resize 
        let (w, h) = window.get_size();
        if w as usize != width || h as usize != height {
            width = w as usize;
            height = h as usize;
            buffer = vec![0u32; width * height];
            scene.width = width;
            scene.height = height;
        }

        scene.render(&mut buffer, t);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

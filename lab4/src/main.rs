mod vertex;
mod transform;
mod shader_mars;
mod shader_neptune;
mod shader_mocca;
mod obj_loader;

use raylib::prelude::*;
use glam::Vec3;
use obj_loader::load_obj;
use vertex::project_to_screen;
use transform::rotate_y;

struct Planet {
    theta: f32,
    radius: f32,
    orbit_radius: f32,
    shader: u8,
    spin: f32,
    spin_speed: f32,
    name: String,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Órbita de Planetas con Perspectiva")
        .build();

    let mesh = load_obj("assets/models/sphere.obj");

    let mut planets = vec![
        Planet { theta: 0.0, radius: 0.3, orbit_radius: 2.0, shader: 1, spin: 0.0, spin_speed: 1.0, name: "Marte".to_string() },
        Planet { theta: 2.0, radius: 0.5, orbit_radius: 2.0, shader: 2, spin: 0.0, spin_speed: 0.5, name: "Neptuno".to_string() },
        Planet { theta: 4.0, radius: 0.4, orbit_radius: 2.0, shader: 3, spin: 0.0, spin_speed: 1.5, name: "Mocca".to_string() },
    ];

    let cam_distance = 2.0;  // distancia de la "cámara" para perspectiva
    let zoom = 200.0;        // factor de escala para pantalla
    let screen_center_x = 400.0;
    let screen_center_y = 300.0;
    let orbit_speed = 0.02;

    let mut time = 0.0;

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            for p in planets.iter_mut() { p.theta += orbit_speed; }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            for p in planets.iter_mut() { p.theta -= orbit_speed; }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for planet in planets.iter_mut() {
            // posición en órbita
            let x = planet.orbit_radius * planet.theta.cos();
            let z = planet.orbit_radius * planet.theta.sin();
            let pos = Vec3::new(x, 0.0, z);

            // rotación propia
            planet.spin += planet.spin_speed * 0.05;

            // perspectiva: planetas más lejos se ven más pequeños
            let perspective = 1.0 / (cam_distance - z + 1.0);
            let planet_scale = planet.radius * perspective;

            // rotar y escalar vértices del mesh
            let transformed: Vec<Vec3> = mesh.vertices.iter()
                .map(|v| rotate_y(*v, planet.spin) * planet_scale + pos)
                .collect();

            for v in transformed.iter() {
                let normal = (*v - pos).normalize();
                let color = match planet.shader {
                    1 => shader_mars::shader_mars(normal, time),
                    2 => shader_neptune::shader_neptune(normal, time),
                    3 => shader_mocca::shader_mocca(normal, time),
                    _ => Vec3::ONE,
                };

                // proyección a pantalla
                let screen_x = screen_center_x + v.x * perspective * zoom;
                let screen_y = screen_center_y + v.y * perspective * zoom;
                d.draw_pixel(screen_x as i32, screen_y as i32, Color::new(
                    (color.x*255.0) as u8,
                    (color.y*255.0) as u8,
                    (color.z*255.0) as u8,
                    255
                ));
            }
            let label_x = screen_center_x + x * perspective * zoom;
            let label_y = screen_center_y - 30.0; // 30 pixeles arriba del planeta
            d.draw_text(&planet.name, label_x as i32, label_y as i32, 20, Color::WHITE);
        }

        time += 0.05;
    }
}

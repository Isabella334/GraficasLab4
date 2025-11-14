mod vertex;
mod transform;
mod shader_mars;
mod shader_neptune;
mod shader_mocca;
mod obj_loader;

use raylib::prelude::*;
use glam::Vec3;
use obj_loader::load_obj;
use transform::rotate_y;

struct Planet {
    theta: f32,
    radius: f32,
    orbit_radius: f32,
    shader: u8,
    spin: f32,
    spin_speed: f32,
    orbit_speed: f32,
    name: String,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Sistema Solar con Capas de Fondo")
        .build();

    let mesh = load_obj("assets/models/sphere.obj");
    let triangles = mesh.triangles();

    // Sol en el centro

    // Planetas
    let mut planets = vec![
        Planet { theta: 0.0, radius: 0.3, orbit_radius: 3.0, shader: 1, spin: 0.0, spin_speed: 1.0, orbit_speed: 0.3, name: "Marte".to_string() },
        Planet { theta: 2.0, radius: 0.4, orbit_radius: 4.5, shader: 2, spin: 0.0, spin_speed: 0.7, orbit_speed: 0.2, name: "Neptuno".to_string() },
        Planet { theta: 4.0, radius: 0.35, orbit_radius: 6.0, shader: 3, spin: 0.0, spin_speed: 1.2, orbit_speed: 0.25, name: "Mocca".to_string() },
    ];

    // Cámara
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.0;
    let mut cam_distance: f32 = 8.0f32;
    let mut orbit_direction: f32 = 1.0;

    // Render settings
    let zoom = 220.0;
    let screen_center_x = 400.0;
    let screen_center_y = 300.0;

    let mut time = 0.0;

    // 🔹 Capas asociadas a cada planeta
    let planet_layers: Vec<u8> = vec![1, 2, 3];

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) { cam_yaw += 0.03; }
        if rl.is_key_down(KeyboardKey::KEY_LEFT)  { cam_yaw -= 0.03; }
        if rl.is_key_down(KeyboardKey::KEY_UP)    { cam_pitch = (cam_pitch + 0.02).clamp(-0.6, 0.6); }
        if rl.is_key_down(KeyboardKey::KEY_DOWN)  { cam_pitch = (cam_pitch - 0.02).clamp(-0.6, 0.6); }

        // W/S: zoom
        if rl.is_key_down(KeyboardKey::KEY_W) { cam_distance = (cam_distance - 0.1f32).max(4.0f32); }
        if rl.is_key_down(KeyboardKey::KEY_S) { cam_distance = (cam_distance + 0.1f32).min(16.0f32); }

        // A/D: invertir dirección de órbita
        if rl.is_key_pressed(KeyboardKey::KEY_A) || rl.is_key_pressed(KeyboardKey::KEY_D) {
            orbit_direction *= -1.0;
        }

        // 🕒 Actualización de posiciones
        let delta_time = 0.05;
        for p in planets.iter_mut() {
            p.theta += orbit_direction * p.orbit_speed * delta_time;
            p.spin += p.spin_speed * delta_time;
        }

        let mut d = rl.begin_drawing(&thread);

        let planet_positions: Vec<Vec3> = planets.iter()
            .map(|p| {
                let x = p.orbit_radius * p.theta.cos();
                let z = p.orbit_radius * p.theta.sin();
                Vec3::new(x, 0.0, z)
            })
            .collect();

        let planet_layers: Vec<u8> = vec![0, 1, 2];
        let planet_radii: Vec<f32> = planets.iter().map(|p| p.radius * 2.0).collect();
        d.clear_background(Color::BLACK);

        // 🪐 Dibujar planetas
        for planet in planets.iter_mut() {
            let x = planet.orbit_radius * planet.theta.cos();
            let z = planet.orbit_radius * planet.theta.sin();
            let pos = Vec3::new(x, 0.0, z);

            let perspective = 1.0 / (cam_distance - z + 1.0);
            let planet_scale = planet.radius * perspective;

            for tri in mesh.triangles() {
                let mut tri_world = [
                    rotate_y(tri[0], planet.spin) * planet_scale + pos,
                    rotate_y(tri[1], planet.spin) * planet_scale + pos,
                    rotate_y(tri[2], planet.spin) * planet_scale + pos,
                ];

                // Proyección
                let mut screen_coords = tri_world.map(|v| {
                    let perspective = 1.0 / (cam_distance - v.z + 1.0);
                    (
                        (screen_center_x + v.x * perspective * zoom) as i32,
                        (screen_center_y - v.y * perspective * zoom) as i32,
                    )
                });

                // Color por planeta
                let color = match planet.shader {
                    1 => shader_mars::shader_mars(tri_world[0].normalize(), time),
                    2 => shader_neptune::shader_neptune(tri_world[0].normalize(), time),
                    3 => shader_mocca::shader_mocca(tri_world[0].normalize(), time),
                    _ => Vec3::new(1.0, 1.0, 1.0),
                };

                // Dibujar triángulo relleno
                d.draw_triangle(
                    rvec2(screen_coords[0].0, screen_coords[0].1),
                    rvec2(screen_coords[1].0, screen_coords[1].1),
                    rvec2(screen_coords[2].0, screen_coords[2].1),
                    Color::new(255, 255, 255, 255),
                );

            }

            // 🏷️ Etiqueta
            let label_x = screen_center_x + x * perspective * zoom;
            let label_y = screen_center_y - (planet.radius * perspective * zoom) - 25.0;
            d.draw_text(&planet.name, label_x as i32 - 20, label_y as i32, 20, Color::WHITE);
        }

        time += 0.05;
    }
}

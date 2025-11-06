mod vertex;
mod transform;
mod shader_mars;
mod shader_neptune;
mod shader_mocca;
mod shader_background;
mod obj_loader;

use raylib::prelude::*;
use glam::Vec3;
use obj_loader::load_obj;
use transform::rotate_y;
use shader_background::background_color;

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

    // Sol en el centro
    let sun_pos = Vec3::new(0.0, 0.0, 0.0);

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
        // 🎮 Controles de cámara
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

        // 🖤 Fondo base negro
        d.clear_background(Color::BLACK);

        // 💫 Capa de fondo calculada (se sobrepone al negro)
        let bg_color = background_color(
            Vec3::new(0.0, 0.0, 0.0),
            time,
            &planet_positions,
            &planet_layers,
            &planet_radii,
        );

        // Dibujamos una capa semitransparente encima del fondo
        d.draw_rectangle(
            0,
            0,
            800,
            600,
            Color::new(
                (bg_color.x * 255.0) as u8,
                (bg_color.y * 255.0) as u8,
                (bg_color.z * 255.0) as u8,
                80, // ← opacidad baja (ajústala entre 50 y 150 según el efecto)
            ),
        );

        // 🌞 Sol en el centro (efecto simple)
        let sun_screen_x = screen_center_x;
        let sun_screen_y = screen_center_y;
        for r in (10..80).step_by(5) {
            let intensity = (255 - (r * 3)).max(0);
            d.draw_circle(
                sun_screen_x as i32,
                sun_screen_y as i32,
                r as f32,
                Color::new(255, (220 - (r as u8 * 2)) as u8, 50, intensity as u8),
            );
        }

        // 🪐 Dibujar planetas
        for planet in planets.iter_mut() {
            let x = planet.orbit_radius * planet.theta.cos();
            let z = planet.orbit_radius * planet.theta.sin();
            let pos = Vec3::new(x, 0.0, z);

            let perspective = 1.0 / (cam_distance - z + 1.0);
            let planet_scale = planet.radius * perspective;

            for v in mesh.vertices.iter() {
                let world_v = rotate_y(v.position, planet.spin) * planet_scale + pos;
                let normal = (world_v - pos).normalize();

                // 💫 Shader por planeta
                let color = match planet.shader {
                    1 => shader_mars::shader_mars(normal, time),
                    2 => shader_neptune::shader_neptune(normal, time),
                    3 => shader_mocca::shader_mocca(normal, time),
                    _ => Vec3::new(1.0, 1.0, 1.0),
                };

                let screen_x = screen_center_x + world_v.x * perspective * zoom;
                let screen_y = screen_center_y - world_v.y * perspective * zoom;

                let dx = screen_x - (screen_center_x + x * perspective * zoom);
                let dy = screen_y - (screen_center_y - 0.0 * perspective * zoom);

                if (dx * dx + dy * dy).sqrt() < planet.radius * 200.0 {
                    d.draw_pixel(
                        screen_x as i32,
                        screen_y as i32,
                        Color::new(
                            (color.x * 255.0) as u8,
                            (color.y * 255.0) as u8,
                            (color.z * 255.0) as u8,
                            255,
                        ),
                    );
                }
            }

            // 🏷️ Etiqueta
            let label_x = screen_center_x + x * perspective * zoom;
            let label_y = screen_center_y - (planet.radius * perspective * zoom) - 25.0;
            d.draw_text(&planet.name, label_x as i32 - 20, label_y as i32, 20, Color::WHITE);
        }

        time += 0.05;
    }
}

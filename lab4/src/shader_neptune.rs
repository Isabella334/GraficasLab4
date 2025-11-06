use glam::Vec3;

pub fn shader_neptune(normal: Vec3, time: f32) -> Vec3 {
    // --- Capa 1: base azul profundo ---
    let base_color = Vec3::new(0.1, 0.25, 0.8);

    // --- Capa 2: iluminación suave ---
    let light_dir = Vec3::new(-0.3, 0.5, 1.0).normalize();
    let light_intensity = normal.dot(light_dir).max(0.0);
    let light_layer = base_color * (0.5 + light_intensity * 0.5);

    // --- Capa 3: ondas gaseosas en movimiento ---
    let waves = ((normal.x * 15.0 + time * 0.8).sin()
               + (normal.y * 10.0 - time * 0.6).cos()) * 0.5 + 0.5;
    let wave_layer = Vec3::new(0.0, 0.3, 0.6) * waves;

    // --- Capa 4: brillo polar o neblina ---
    let polar_effect = (1.0 - normal.y.abs()).powf(3.0);
    let glow_layer = Vec3::new(0.5, 0.8, 1.0) * polar_effect * 0.4;

    // --- Combinación final ---
    (base_color * 0.5 + light_layer * 0.3 + wave_layer * 0.8 + glow_layer * 0.5)
        .clamp(Vec3::ZERO, Vec3::ONE)
}


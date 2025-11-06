use glam::Vec3;

pub fn shader_mocca(normal: Vec3, time: f32) -> Vec3 {
    // --- Capa 1: color base cálido ---
    let base_color = Vec3::new(0.55, 0.35, 0.2); // marrón suave

    // --- Capa 2: iluminación dorada ---
    let light_dir = Vec3::new(0.4, 0.6, 1.0).normalize();
    let light_intensity = normal.dot(light_dir).max(0.0);
    let light_layer = Vec3::new(1.0, 0.8, 0.6) * light_intensity * 0.4;

    // --- Capa 3: vetas o textura terrestre ---
    let pattern = ((normal.x * 10.0 + time * 0.5).sin()
                 * (normal.z * 12.0 + time * 0.3).cos()).abs();
    let texture_layer = Vec3::new(0.4, 0.25, 0.15) * pattern;

    // --- Capa 4: reflejos cálidos ---
    let reflect = (normal.z * 0.5 + 0.5).powf(4.0);
    let reflect_layer = Vec3::new(1.0, 0.6, 0.3) * reflect * 0.3;

    // --- Combinación final ---
    (base_color * 0.6 + light_layer * 0.3 + texture_layer * 0.8 + reflect_layer * 0.4)
        .clamp(Vec3::ZERO, Vec3::ONE)
}

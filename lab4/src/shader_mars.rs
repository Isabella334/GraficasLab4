use glam::Vec3;

pub fn shader_mars(normal: Vec3, time: f32) -> Vec3 {
    // Capa base (color marciano)
    let base = Vec3::new(0.8, 0.3, 0.1);

    // Capa 1: patrón senoidal
    let layer1 = (normal.x * 10.0 + time).sin().abs();

    // Capa 2: variación vertical (latitud)
    let layer2 = (normal.y * 4.0).sin().abs();

    // Capa 3: ondulación temporal
    let layer3 = ((normal.z + time * 0.8).sin() * 0.5 + 0.5);

    // Capa 4: sombreado dependiente del ángulo (simula luz)
    let light_dir = Vec3::new(0.3, 0.6, 0.8).normalize();
    let diffuse = normal.dot(light_dir).max(0.0);

    // Mezcla de capas
    let color = base * (0.5 + 0.5 * layer1)
        + Vec3::new(0.2, 0.1, 0.05) * layer2
        + Vec3::new(0.9, 0.4, 0.2) * layer3
        + diffuse * Vec3::new(0.3, 0.2, 0.1);

    color.clamp(Vec3::ZERO, Vec3::ONE)
}

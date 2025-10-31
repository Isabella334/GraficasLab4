use glam::Vec3;

pub fn shader_neptune(normal: Vec3, time: f32) -> Vec3 {
    let deep_blue = Vec3::new(0.0, 0.1, 0.5);
    let light_blue = Vec3::new(0.1, 0.4, 0.9);

    // Ondas gaseosas animadas
    let wave = (normal.x * 5.0 + time * 0.3).sin() * 0.3 + 0.7;
    deep_blue.lerp(light_blue, wave)
}

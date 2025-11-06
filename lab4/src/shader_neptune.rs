use glam::Vec3;

pub fn shader_neptune(normal: Vec3, time: f32) -> Vec3 {
    let layer1 = (normal.x * 4.0 + time * 0.6).cos().abs();
    let layer2 = (normal.y * 2.0 + time).sin().abs();
    let layer3 = (normal.z * 5.0 - time * 0.3).cos().abs();
    Vec3::new(layer1 * 0.2, layer2 * 0.4 + 0.5, layer3 * 0.8 + 0.2)
}


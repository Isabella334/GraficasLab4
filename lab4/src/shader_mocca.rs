use glam::Vec3;

pub fn shader_mocca(normal: Vec3, time: f32) -> Vec3 {
    let layer1 = (normal.x * 6.0 + time).sin().abs();
    let layer2 = (normal.y * 2.5 - time * 0.7).cos().abs();
    let layer3 = (normal.z * 3.5 + time).sin().abs();
    Vec3::new(layer1 * 0.6 + 0.3, layer2 * 0.3 + 0.2, layer3 * 0.1)
}

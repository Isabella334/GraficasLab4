use glam::Vec3;

pub fn shader_mars(normal: Vec3, time: f32) -> Vec3 {
    let layer1 = (normal.x * 5.0 + time).sin().abs();
    let layer2 = (normal.y * 3.0 - time * 0.5).cos().abs();
    let layer3 = (normal.z * 2.0 + time * 0.8).sin().abs();
    let color = Vec3::new(layer1 * 0.8 + 0.4, layer2 * 0.2, layer3 * 0.1);
    color
}

use glam::Vec3;

pub fn shader_mars(normal: Vec3, time: f32) -> Vec3 {
    let base_color = Vec3::new(0.8, 0.4, 0.2); // marrón rojizo
    let lighting = normal.dot(Vec3::new(0.0, 0.0, 1.0)).abs();
    base_color * lighting
}

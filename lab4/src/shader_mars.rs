use glam::Vec3;

pub fn shader_mars(normal: Vec3, time: f32) -> Vec3 {
    let base_color = Vec3::new(0.8, 0.3, 0.1);
    let dark_color = Vec3::new(0.4, 0.15, 0.05);
    let intensity = 0.3 + 0.7 * normal.y.max(0.0);
    let color = base_color * intensity + dark_color * (1.0 - intensity);
    let variation = (time * 0.5 + normal.x * 10.0).sin() * 0.1;
    color + Vec3::splat(variation)
}

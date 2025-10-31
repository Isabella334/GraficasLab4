use glam::Vec3;

pub fn shader_mocca(normal: Vec3, time: f32) -> Vec3 {
    let base = Vec3::new(0.2, 0.7, 0.4);
    let glow = Vec3::new(0.5, 1.0, 0.8);

    // bioluminiscencia simulada
    let pulsation = (time * 2.0 + normal.x * 10.0).sin() * 0.5 + 0.5;
    base.lerp(glow, pulsation)
}

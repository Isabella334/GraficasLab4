use glam::Vec3;

pub fn clamp_vec3(v: Vec3, min: f32, max: f32) -> Vec3 {
    Vec3::new(
        v.x.clamp(min, max),
        v.y.clamp(min, max),
        v.z.clamp(min, max),
    )
}

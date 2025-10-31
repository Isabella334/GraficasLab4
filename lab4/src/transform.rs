use glam::{Vec3, Mat4};

pub fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let rotation = Mat4::from_rotation_y(angle);
    (rotation * v.extend(1.0)).truncate()
}


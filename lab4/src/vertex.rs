use glam::Vec3;

pub fn project_to_screen(pos: Vec3, width: usize, height: usize, zoom: f32) -> (usize, usize) {
    let x = ((pos.x * zoom + 1.0) * 0.5 * width as f32) as usize;
    let y = ((1.0 - (pos.y * zoom + 1.0) * 0.5) * height as f32) as usize;
    (x.min(width-1), y.min(height-1))
}

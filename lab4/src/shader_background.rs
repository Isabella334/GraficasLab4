use glam::Vec3;

pub fn background_color(
    _pos: Vec3,
    _time: f32,
    _planet_positions: &Vec<Vec3>,
    planet_layers: &Vec<u8>,
    _planet_radii: &Vec<f32>,
) -> Vec3 {
    // 🎨 Definimos tres capas de fondo fijas
    let layers = vec![
        Vec3::new(0.05, 0.05, 0.1),  // Capa 0 - azul oscuro
        Vec3::new(0.15, 0.05, 0.2),  // Capa 1 - violeta
        Vec3::new(0.0, 0.0, 0.0),    // Capa 2 - negro profundo
    ];

    // Toma el color promedio de las capas activas (puedes simplificarlo)
    let mut color_sum = Vec3::ZERO;
    for &layer in planet_layers {
        color_sum += layers[layer as usize % layers.len()];
    }

    color_sum / (planet_layers.len() as f32)
}

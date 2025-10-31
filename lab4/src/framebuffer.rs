use glam::Vec3;
use std::fs::File;
use std::io::Write;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub color_buffer: Vec<Vec3>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![Vec3::ZERO; width * height],
        }
    }

    pub fn clear(&mut self, color: Vec3) {
        self.color_buffer.fill(color);
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.color_buffer[idx] = color.clamp(Vec3::ZERO, Vec3::ONE);
        }
    }

    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).expect("No se pudo crear el archivo PPM");
        write!(file, "P3\n{} {}\n255\n", self.width, self.height).unwrap();
        for pixel in &self.color_buffer {
            let r = (pixel.x * 255.0) as u8;
            let g = (pixel.y * 255.0) as u8;
            let b = (pixel.z * 255.0) as u8;
            write!(file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}

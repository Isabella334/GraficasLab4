// src/obj_loader.rs
use glam::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub vertices: Vec<Vec3>,
}

pub fn load_obj(path: &str) -> Mesh {
    let file = File::open(path).expect(&format!("No se pudo abrir el archivo OBJ: {}", path));
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error leyendo línea del OBJ");
        let line = line.trim();

        // Solo procesamos las líneas que comienzan con "v " (vértices)
        if line.starts_with("v ") {
            let coords: Vec<f32> = line[2..]
                .split_whitespace()
                .map(|s| s.parse::<f32>().expect("Error convirtiendo a f32"))
                .collect();

            if coords.len() >= 3 {
                vertices.push(Vec3::new(coords[0], coords[1], coords[2]));
            }
        }
    }

    Mesh { vertices }
}

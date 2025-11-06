use std::fs::File;
use std::io::{BufRead, BufReader};
use glam::Vec3;

// --- Estructura básica de un vértice ---
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
}

// --- Estructura de una malla (mesh) con vértices y caras ---
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<[usize; 3]>, // triángulos: índices de vértices
}

// --- Función para cargar archivos .obj ---
pub fn load_obj(path: &str) -> Mesh {
    let file = File::open(path).expect("No se pudo abrir el archivo OBJ");
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error al leer línea del archivo");

        if line.starts_with("v ") {
            // Línea de vértice
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                vertices.push(Vertex { position: Vec3::new(x, y, z) });
            }
        } else if line.starts_with("f ") {
            // Línea de cara
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let v0 = parts[1].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                let v1 = parts[2].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                let v2 = parts[3].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                faces.push([v0, v1, v2]);
            }
        }
    }

    // Devolvemos la malla completa
    Mesh { vertices, faces }
}

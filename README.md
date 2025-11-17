# 🪐 Sistema Solar con Shaders Procedurales

Implementación de un sistema solar estilizado en Rust, con shaders procedurales para cada planeta.  
Incluye **Marte**, **Mocca** (inventado), **Saturno** (con anillos), y el **Sol**.

![Sistema Solar](docs/SistemaSolar1.png)

![Sistema Solar](docs/SistemaSolar3.png)  

---

## 🛠️ Compilación y ejecución

```bash
git clone https://github.com/tu-usuario/sistema-solar.git
cd GraficasLab4
cargo run
```
---

## 🎮 Controles
Flechas ← → ↑ ↓: rotar cámara alrededor del sistema.

---

## 📚 Documentación técnica

# Estructuras clave
Uniforms (src/main.rs)

```rust
pub struct Uniforms {
    pub model_matrix: Matrix,    // Transformación: mundo local → mundo
    pub view_matrix: Matrix,     // Cámara: mundo → vista
    pub projection_matrix: Matrix, // Proyección: vista → clip
    pub viewport_matrix: Matrix, // Viewport: clip → pantalla
    pub is_ring: bool,           // Activa geometría plana para anillos
}
```
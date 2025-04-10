use glam::Vec2;
use minifb::Window;

use crate::utils::Color;

use super::{texture::Material, transform::Transform};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(x: f32, y: f32, color: Color, uv: Vec2) -> Self {
        Self { x, y, color, uv }
    }
}
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<[usize; 3]>,
    pub transform: Transform,
    pub material: Material,
    pub layer: usize, // Z-index para orden de dibujo, como en capas
}

impl Shape {
    pub fn quad() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    x: 10.0,
                    y: 10.0,
                    color: Color::rgb(255, 0, 0),
                    uv: Vec2::new(1.0, 1.0),
                },
                Vertex {
                    x: 00.0,
                    y: 10.0,
                    color: Color::rgb(0, 255, 0),
                    uv: Vec2::new(0.0, 1.0),
                },
                Vertex {
                    x: 00.0,
                    y: 00.0,
                    color: Color::rgb(0, 0, 255),
                    uv: Vec2::new(0.0, 0.0),
                },
                Vertex {
                    x: 10.0,
                    y: 00.0,
                    color: Color::rgb(255, 0, 255),
                    uv: Vec2::new(1.0, 0.0),
                },
            ],
            indices: vec![[0, 1, 2], [0, 3, 2]],
            transform: Transform::identity(),
            material: Material::default(),
            layer: 0,
        }
    }
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
    pub fn wit_texture(mut self, path: &str) -> Self {
        self.material = Material::load_texture(path);
        self
    }
}

// impl PluginGalar for Shape {
//     fn update(&mut self, window: &Window, buffer: &mut Vec<u32>) {
//         self.transform.displace(window);
//         draw_shape(buffer, window, &self);
//     }
// }

pub fn draw_shape(buffer: &mut Vec<u32>, window: &Window, shape: &Shape) {
    for triangle in &shape.indices {
        let v0 = &shape.vertices[triangle[0]];
        let v1 = &shape.vertices[triangle[1]];
        let v2 = &shape.vertices[triangle[2]];

        draw_triangle_cpu(
            buffer,
            window,
            &shape.transform,
            v0,
            v1,
            v2,
            &shape.material,
        );
    }
}

fn draw_triangle_cpu(
    buffer: &mut Vec<u32>,
    window: &Window,
    transform: &Transform,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    material: &Material,
) {
    let (width, height) = window.get_size();

    // 🧠 Dimensiones aproximadas del shape (puede venir de otra parte)
    let shape_w = 10.0;
    let shape_h = 10.0;

    // 💫 Aplicamos transformación centrada
    let p0 = transform.apply_centered(v0.x, v0.y, shape_w, shape_h);
    let p1 = transform.apply_centered(v1.x, v1.y, shape_w, shape_h);
    let p2 = transform.apply_centered(v2.x, v2.y, shape_w, shape_h);

    let min_x = p0.0.min(p1.0).min(p2.0).max(0);
    let min_y = p0.1.min(p1.1).min(p2.1).max(0);
    let max_x = p0.0.max(p1.0).max(p2.0).min(width - 1);
    let max_y = p0.1.max(p1.1).max(p2.1).min(height - 1);

    let area = edge_function(p0, p1, p2) as f32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // UV usando coordenadas baricéntricas
            let w0 = edge_function((x, y), p1, p2) as f32 / area;
            let w1 = edge_function((x, y), p2, p0) as f32 / area;
            let w2 = edge_function((x, y), p0, p1) as f32 / area;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let index = y * width + x;
                let color = if let Some(color) = material.base_color {
                    color.to_hex()
                } else {
                    // Interpolación de color
                    let r =
                        (v0.color.r as f32 * w0 + v1.color.r as f32 * w1 + v2.color.r as f32 * w2)
                            .round() as u8;

                    let g =
                        (v0.color.g as f32 * w0 + v1.color.g as f32 * w1 + v2.color.g as f32 * w2)
                            .round() as u8;

                    let b =
                        (v0.color.b as f32 * w0 + v1.color.b as f32 * w1 + v2.color.b as f32 * w2)
                            .round() as u8;

                    Color::rgb(r, g, b).to_hex()
                };
                if let Some(_) = material.texture {
                    let uv_0 = v0.uv;
                    let uv_1 = v1.uv;
                    let uv_2 = v2.uv;
                    let uv = (
                        w0 * uv_0.x + w1 * uv_1.x + w2 * uv_2.x,
                        w0 * uv_0.y + w1 * uv_1.y + w2 * uv_2.y,
                    );
                    buffer[index] = material.sample_texture(uv);
                } else {
                    buffer[index] = color;
                }
            }
        }
    }
}

fn edge_function(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> i32 {
    let (ax, ay) = (a.0 as i32, a.1 as i32);
    let (bx, by) = (b.0 as i32, b.1 as i32);
    let (cx, cy) = (c.0 as i32, c.1 as i32);

    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}

pub fn pixel_set(
    buffer: &mut Vec<u32>,
    window: &minifb::Window,
    transform: &Transform,
    color: Color,
) {
    let x = transform.translation.x;
    let y = transform.translation.y;

    let (width, height) = window.get_size();

    if x < width && y < height {
        let index = y * width + x;
        buffer[index] = color.to_hex();
    }
}

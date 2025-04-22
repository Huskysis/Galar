use glam::Vec2;

use super::{draws::draw_shape, texture::Material, transform::Transform, utils::{ConfigGalar, PluginGalar}};
use super::colores::Color;

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

#[derive(Debug, Clone)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<[usize; 3]>,
    pub transform: Transform,
    pub material: Material,
    pub layer: usize, // Z-index para orden de dibujo, como en capas
    pub origen: bool, // esquina superior izquierda origen local
}

impl Shape {
    pub fn quad(size: usize) -> Self {
        let size = size as f32;
        Self {
            vertices: vec![
                Vertex { x: 0.0, y:  size, color: Color::rgb(255, 0, 0,) , uv: Vec2::new(0.0, 1.0)},
                Vertex { x: 0.0, y: 0.0, color: Color::rgb(0, 255, 0,) , uv: Vec2::new(0.0, 0.0)},
                Vertex { x:  size, y:0.0, color: Color::rgb(0, 0, 255,) , uv: Vec2::new(1.0, 0.0)},
                Vertex { x:  size, y:  size, color: Color::rgb(255, 0, 255,) , uv: Vec2::new(1.0, 1.0)},
            ],
            indices: vec![[0, 1, 2],[0, 3, 2]],
            transform: Transform::identity(),
            material: Material::default(),
            layer: 0,
            origen: false,
        }
    }
    pub fn with_position(mut self, x: f32,y: f32) -> Self {
        self.transform.translation = Vec2::new(x, y);
        self
    }
    pub fn with_rotation(mut self, degree: f32) -> Self {
        self.transform.rotation = degree.to_radians();
        self
    }
    pub fn with_scale(mut self, x: f32,y: f32 ) -> Self {
        self.transform.scale = Vec2::new(x, y);
        self
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
    /// Ruta Absoluta Ej: "src/assets/image.png"
    pub fn with_texture(mut self, path: &str) -> Self {
        self.material = Material::load_texture(path);
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.material = Material {
            base_color: Some(color),
            texture: None,
        };
        self
    }
    pub fn with_orgien(mut self, origen: bool) -> Self {
        self.origen = origen;
        self
    }
    pub fn set_position(&mut self, x: f32,y: f32) {
        self.transform.translation = Vec2::new(x, y);
    }
    pub fn set_rotation(&mut self, degree: f32) {
        self.transform.rotation = degree.to_radians();
    }
    pub fn set_scale(&mut self, x: f32,y: f32 ) {
        self.transform.scale = Vec2::new(x, y);
    }
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    /// Ruta Absoluta Ej: "src/assets/image.png"
    pub fn set_texture(&mut self, path: &str) {
        self.material = Material::load_texture(path);
    }
    pub fn set_color(&mut self, color: Color) {
        self.material = Material {
            base_color: Some(color),
            texture: None,
        };
    }
    pub fn set_origen(&mut self, origen: bool){
        self.origen = origen;
    }
    pub fn get_dimensions(&self) -> (f32, f32) {
        let min_x = self.vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
        let max_x = self.vertices.iter().map(|v| v.x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = self.vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
        let max_y = self.vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max);

        (max_x - min_x, max_y - min_y)
    }
    pub fn get_center(&self) -> (f32, f32) {
        let (w, h) = self.get_dimensions();
        let min_x = self.vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
        let min_y = self.vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
        (min_x + w / 2.0, min_y + h / 2.0)
    }
}

impl PluginGalar for Shape {
    fn update(&mut self, config: &mut ConfigGalar) {
        draw_shape(config, &self);
    }
}


pub enum GeometryShape {
    Triangle {
        x: f32,
        y: f32,
        size: f32,
        color: Color,
    },
    Square {
        x: f32,
        y: f32,
        size: f32,
        color: Color,
    },
    Circle {
        x: f32,
        y: f32,
        radio: f32,
        segments: usize,
        color: Color,
    }
}

impl GeometryShape {
    fn new_shape(&self) -> Shape {
        match self {
            &GeometryShape::Triangle { x, y, size, color } => Self::shape_triangle(x, y, size, color),
            &GeometryShape::Square { x, y, size, color } => Self::shape_square(x, y, size, color),
            &GeometryShape::Circle { x, y, radio, segments, color } => Self::shape_circle(x, y, radio, segments, color),
        }
    }
    fn shape_triangle(x: f32, y: f32, size: f32, color: Color) -> Shape {
        let h = (size * (3.0_f32).sqrt()) / 2.0; // altura del triángulo equilátero
    
        Shape {
            vertices: vec![
                Vertex { x: -size / 2.0, y: -h / 3.0, color, uv: Vec2::new(1.0, 1.0)},
                Vertex { x: size / 2.0, y: -h / 3.0, color, uv: Vec2::new(1.0, 1.0)},
                Vertex { x: 0.0, y: (2.0 * h) / 3.0, color, uv: Vec2::new(1.0, 1.0)},
            ],
            indices: vec![[0, 1, 2]],
            transform: Transform::from_translation(x, y),
            material: Material::default(),
            layer: 0,
            origen: false
        }
    }
    fn shape_square(x: f32, y: f32, size: f32, color: Color) -> Shape {
        Shape {
            vertices: vec![
                Vertex { x: -size / 2.0, y:  size / 2.0, color , uv: Vec2::new(0.0, 0.0)},
                Vertex { x: -size / 2.0, y: -size / 2.0, color , uv: Vec2::new(0.0, 1.0)},
                Vertex { x:  size / 2.0, y: -size / 2.0, color , uv: Vec2::new(1.0, 1.0)},
                Vertex { x:  size / 2.0, y:  size / 2.0, color , uv: Vec2::new(1.0, 0.0)},
            ],
            indices: vec![[0, 1, 2], [0, 2, 3]],
            transform: Transform::from_translation(x, y),
            material: Material::default(),
            layer: 0,
            origen: false
        }
    }
    fn shape_circle(x: f32, y: f32, radius: f32, segments: usize, color: Color) -> Shape {
        let mut vertices = vec![Vertex { x: 0.0, y: 0.0, color, uv: Vec2::ZERO }];
        let mut indices = vec![];
    
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex { x, y, color, uv: Vec2::ZERO});
    
            let i0 = 0;
            let i1 = i + 1;
            let i2 = if i + 2 > segments { 1 } else { i + 2 };
            indices.push([i0, i1, i2]);
        }
    
        Shape {
            vertices,
            indices,
            transform: Transform::from_translation(x, y),
            material: Material::default(),
            layer: 0,
            origen: false
        }
    }
}

impl PluginGalar for GeometryShape {
    fn update(&mut self, config: &mut ConfigGalar) {
        let shape = self.new_shape();

        draw_shape(config, &shape);
    }
}
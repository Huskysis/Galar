use glam::Vec2;
use minifb::{Window, WindowOptions};

use crate::{
    mesh::{Shape, Vertex, draw_shape},
    texture::Material,
    transform::Transform,
};

pub trait PluginGalar {
    fn update(&mut self, window: &Window, buffer: &mut Vec<u32>);
}

pub enum GeometryShape {
    Triangle {
        x: usize,
        y: usize,
        size: f32,
        color: Color,
    },
    Square {
        x: usize,
        y: usize,
        size: f32,
        color: Color,
    },
    Circle {
        x: usize,
        y: usize,
        radio: f32,
        segments: usize,
        color: Color,
    },
}

impl GeometryShape {
    fn new_shape(&self) -> Shape {
        match self {
            &GeometryShape::Triangle { x, y, size, color } => {
                Self::shape_triangle(x, y, size, color)
            }
            &GeometryShape::Square { x, y, size, color } => Self::shape_square(x, y, size, color),
            &GeometryShape::Circle {
                x,
                y,
                radio,
                segments,
                color,
            } => Self::shape_circle(x, y, radio, segments, color),
        }
    }
    fn shape_triangle(x: usize, y: usize, size: f32, color: Color) -> Shape {
        let h = (size * (3.0_f32).sqrt()) / 2.0; // altura del triángulo equilátero

        Shape {
            vertices: vec![
                Vertex {
                    x: -size / 2.0,
                    y: -h / 3.0,
                    color,
                    uv: Vec2::new(1.0, 1.0),
                },
                Vertex {
                    x: size / 2.0,
                    y: -h / 3.0,
                    color,
                    uv: Vec2::new(1.0, 1.0),
                },
                Vertex {
                    x: 0.0,
                    y: (2.0 * h) / 3.0,
                    color,
                    uv: Vec2::new(1.0, 1.0),
                },
            ],
            indices: vec![[0, 1, 2]],
            transform: Transform::from_translation(x, y),
            material: Material::default(),
            layer: 0,
        }
    }
    fn shape_square(x: usize, y: usize, size: f32, color: Color) -> Shape {
        Shape {
            vertices: vec![
                Vertex {
                    x: -size / 2.0,
                    y: size / 2.0,
                    color,
                    uv: Vec2::new(0.0, 0.0),
                },
                Vertex {
                    x: -size / 2.0,
                    y: -size / 2.0,
                    color,
                    uv: Vec2::new(0.0, 1.0),
                },
                Vertex {
                    x: size / 2.0,
                    y: -size / 2.0,
                    color,
                    uv: Vec2::new(1.0, 1.0),
                },
                Vertex {
                    x: size / 2.0,
                    y: size / 2.0,
                    color,
                    uv: Vec2::new(1.0, 0.0),
                },
            ],
            indices: vec![[0, 1, 2], [0, 2, 3]],
            transform: Transform::from_translation(x, y),
            material: Material::default(),
            layer: 0,
        }
    }
    fn shape_circle(x: usize, y: usize, radius: f32, segments: usize, color: Color) -> Shape {
        let mut vertices = vec![Vertex {
            x: 0.0,
            y: 0.0,
            color,
            uv: Vec2::ZERO,
        }];
        let mut indices = vec![];

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex {
                x,
                y,
                color,
                uv: Vec2::ZERO,
            });

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
        }
    }
}

impl PluginGalar for GeometryShape {
    fn update(&mut self, window: &Window, buffer: &mut Vec<u32>) {
        let shape = self.new_shape();

        draw_shape(buffer, window, &shape);
    }
}

pub struct Galar {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub plugins: Vec<Box<dyn PluginGalar>>, // Aqui van los draw y posiblemente otras funcionalidades
}

impl Galar {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
        framerate: usize,
        options: Option<WindowOptions>,
    ) -> Self {
        let mut window = Window::new(
            name,
            width,
            height,
            if let Some(window_options) = options {
                window_options
            } else {
                WindowOptions::default()
            },
        )
        .unwrap();
        window.set_target_fps(framerate);

        let buffer = vec![0u32; width * height];
        let plugins = Vec::new();
        Self {
            window,
            buffer,
            plugins,
        }
    }

    pub fn add_plugin<P: PluginGalar + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn run(&mut self) {
        let (width, height) = self.window.get_size();

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Space) {
            self.buffer.fill(0u32); // borrar el rastro en pocas palabras

            for plugin in self.plugins.iter_mut() {
                plugin.update(&self.window, &mut self.buffer);
            }

            self.window
                .update_with_buffer(&self.buffer, width, height)
                .unwrap();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255, // Alpha por defecto completamente opaco
        }
    }
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Valores en un rango de:
    ///
    /// h => 0.0 .. 360.0
    ///
    /// s => 0.0 .. 1.0
    ///
    /// l => 0.0 .. 1.0
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);

        if s == 0.0 {
            let value = (l * 255.0).round() as u8;
            return Self::rgb(value, value, value);
        }

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h = h % 360.0;
        let h = if h < 0.0 { h + 360.0 } else { h };
        let h_prime = h / 60.0;
        let sector = h_prime as u8 % 6;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

        let (r1, g1, b1) = match sector {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => unreachable!(),
        };

        let m = l - c / 2.0;

        let r = ((r1 + m).clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = ((g1 + m).clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = ((b1 + m).clamp(0.0, 1.0) * 255.0).round() as u8;

        Self::rgb(r, g, b)
    }

    /// Order `RRGGBBAA``, no el clásico `AARRGGBB``
    pub fn to_hex(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | self.a as u32
    }

    // Método adicional para crear desde HSLA
    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        let mut color = Self::hsl(h, s, l);
        color.a = a;
        color
    }

    /// Empieza en `&self` y termina con `end`
    pub fn interpolate_color(&self, end: Self, t: f32) -> Self {
        Self {
            r: ((1.0 - t) * self.r as f32 + t * end.r as f32) as u8,
            g: ((1.0 - t) * self.g as f32 + t * end.g as f32) as u8,
            b: ((1.0 - t) * self.b as f32 + t * end.b as f32) as u8,
            a: ((1.0 - t) * self.a as f32 + t * end.a as f32) as u8,
        }
    }

    // Constantes de colores básicos y extendidos
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const GRAY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const ORANGE: Self = Self {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };
    pub const PURPLE: Self = Self {
        r: 128,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const BROWN: Self = Self {
        r: 139,
        g: 69,
        b: 19,
        a: 255,
    };
    pub const PINK: Self = Self {
        r: 255,
        g: 192,
        b: 203,
        a: 255,
    };
    pub const LIME: Self = Self {
        r: 50,
        g: 205,
        b: 50,
        a: 255,
    };
    pub const TEAL: Self = Self {
        r: 0,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const NAVY: Self = Self {
        r: 0,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const GOLD: Self = Self {
        r: 255,
        g: 215,
        b: 0,
        a: 255,
    };
    pub const SILVER: Self = Self {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };
}

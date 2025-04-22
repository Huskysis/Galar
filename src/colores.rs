
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

    pub fn random() -> Self {
        let r = rand::random_range(0..255);
        let g = rand::random_range(0..255);
        let b = rand::random_range(0..255);
        let a = 255;
        Self { r, g, b, a }
    }
    pub fn random_with_alpha() -> Self {
        let r = rand::random_range(0..255);
        let g = rand::random_range(0..255);
        let b = rand::random_range(0..255);
        let a = rand::random_range(0..255);
        Self { r, g, b, a }
    }
    /// Valores en un rango de:  
    /// h => 0.0 .. 360.0  
    /// s => 0.0 .. 1.0  
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


    /// Interpolación directa HSL
    pub fn rainbow(value: f32) -> Self {
        let value = value.clamp(0.0, 1.0);
        let hue = value * 360.0;
        Self::hsl(hue, 1.0, 0.5)
    }

    /// Interpolación a escala grises
    pub fn lightness(value: f32) -> Self {
        let value = value.clamp(0.0, 1.0);
        Self::hsl(0.0, 0.0, value)
    }

    /// Interpola linealmente entre dos colores
    pub fn lerp(c1: Color, c2: Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let r = c1.r as f32 + (c2.r as f32 - c1.r as f32) * t;
        let g = c1.g as f32 + (c2.g as f32 - c1.g as f32) * t;
        let b = c1.b as f32 + (c2.b as f32 - c1.b as f32) * t;
        let a = c1.a as f32 + (c2.a as f32 - c1.a as f32) * t;
        Color {
            r: r.round() as u8,
            g: g.round() as u8,
            b: b.round() as u8,
            a: a.round() as u8,
        }
    }

    /// Color de un valor de 0.0 a 1.0 usando un gradiente tipo fuego (negro → rojo → amarillo → blanco)
    pub fn from_heat(value: f32) -> Self {
        let value = value.clamp(0.0, 1.0);
        if value < 0.5 {
            // Negro a Rojo a Amarillo
            if value < 0.25 {
                Color::lerp(Color::rgb(0, 0, 0), Color::rgb(255, 0, 0), value / 0.25)
            } else {
                Color::lerp(Color::rgb(255, 0, 0), Color::rgb(255, 255, 0), (value - 0.25) / 0.25)
            }
        } else {
            // Amarillo a Blanco
            Color::lerp(Color::rgb(255, 255, 0), Color::rgb(255, 255, 255), (value - 0.5) / 0.5)
        }
    }

    // Método adicional para crear desde HSLA
    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        let mut color = Self::hsl(h, s, l);
        color.a = a;
        color
    }

    /// Para formato RRGGBBAA
    pub fn to_rgba_hex(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | self.a as u32
    }

    /// Para formato AARRGGBB
    pub fn to_hex(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
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

    // Obtener colores individuales
    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }
    pub fn a(&self) -> u8 { self.a }

    // Alterar colores individuales
    pub fn set_r(&mut self, r: u8) { self.r = r; }
    pub fn set_g(&mut self, g: u8) { self.g = g; }
    pub fn set_b(&mut self, b: u8) { self.b = b; }
    pub fn set_a(&mut self, a: u8) { self.a = a; }

    // Constantes de colores básicos y extendidos
    pub const WHITE:  Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK:  Self = Self { r: 0,   g: 0,   b: 0,   a: 255 };
    pub const RED:    Self = Self { r: 255, g: 0,   b: 0,   a: 255 };
    pub const GREEN:  Self = Self { r: 0,   g: 255, b: 0,   a: 255 };
    pub const BLUE:   Self = Self { r: 0,   g: 0,   b: 255, a: 255 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0,   a: 255 };
    pub const CYAN:   Self = Self { r: 0,   g: 255, b: 255, a: 255 };
    pub const MAGENTA:Self = Self { r: 255, g: 0,   b: 255, a: 255 };
    pub const GRAY:   Self = Self { r: 128, g: 128, b: 128, a: 255 };
    pub const ORANGE: Self = Self { r: 255, g: 165, b: 0,   a: 255 };
    pub const PURPLE: Self = Self { r: 128, g: 0,   b: 128, a: 255 };
    pub const BROWN:  Self = Self { r: 139, g: 69,  b: 19,  a: 255 };
    pub const PINK:   Self = Self { r: 255, g: 192, b: 203, a: 255 };
    pub const LIME:   Self = Self { r: 50,  g: 205, b: 50,  a: 255 };
    pub const TEAL:   Self = Self { r: 0,   g: 128, b: 128, a: 255 };
    pub const NAVY:   Self = Self { r: 0,   g: 0,   b: 128, a: 255 };
    pub const GOLD:   Self = Self { r: 255, g: 215, b: 0,   a: 255 };
    pub const SILVER: Self = Self { r: 192, g: 192, b: 192, a: 255 };
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.to_hex()
    }
}
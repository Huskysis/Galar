use rand::random_range;

#[derive(Clone, Copy)]
pub struct Float(pub f32);

impl core::fmt::Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.0).finish()
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for Float {}

pub fn roll_dice(faces: u32) -> u32 {
    random_range(0..faces)
}

/// Implementa la detección de dos objetos.
///
/// El campo de deteccion es Esferica si se le da un radio
pub fn proximidad(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distancia_cuadrada = dx * dx + dy * dy;

    // Si no quieres calcular la raiz cuadrada, descomenta
    // distancia_cuadrada
    // distancia_cuadrada.sqrt()
    fast_sqrt(distancia_cuadrada)
}

pub fn fast_sqrt(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }

    let x_half = 0.5 * x;
    let mut i = x.to_bits();        // Interpretar f32 como u32
    i = 0x5f3759df - (i >> 1);           // El famoso número mágico
    let mut y = f32::from_bits(i);  // Interpretar de nuevo como f32
    y = y * (1.5 - x_half * y * y) ;     // Primera iteración de Newton-Raphson
    1.0 / y                              // Invertimos para obtener sqrt(x)
}


/// ⚠️ k define qué tan brusco o suave es el corte.
///
/// Algo como k = 0.01 es bastante sutil, k = 0.1 ya lo vuelve más marcado.
pub fn soft_proximity(x1: f32, y1: f32, x2: f32, y2: f32, k: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let d = (dx * dx + dy * dy).sqrt();
    1.0 - (-k * d).exp() // Valor suavizado entre 0 y 1
}

/// ```
/// Value: the value to be remapped.
/// Start1: lower bound of the value current range
/// Stop1: upper bound of the value current range
/// Start2: lower bound of the value target range
/// Stop2: upper bound of the value target range
/// Bounds: constrain the value to the newly mapped range
/// ```
pub fn remapear(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32, bounds: bool) -> f32 {
    let new_value = (value - start1) / (stop1 - start1) * (stop2 - start2) + start2;

    if !bounds {
        return new_value;
    }

    if start2 < stop2 {
        new_value.clamp(start2, stop2)
    } else {
        new_value.clamp(stop2, start2)
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

    /// Para formato RRGGBBAA
    pub fn to_rgba(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | self.a as u32
    }

    /// Para formato AARRGGBB
    pub fn to_hex(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
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

    /// Empieza en `&self` y termina con `end`
    pub fn interpolate_color(&self, end: Self, t: f32) -> Self {
        Self {
            r: ((1.0 - t) * self.r as f32 + t * end.r as f32) as u8,
            g: ((1.0 - t) * self.g as f32 + t * end.g as f32) as u8,
            b: ((1.0 - t) * self.b as f32 + t * end.b as f32) as u8,
            a: ((1.0 - t) * self.a as f32 + t * end.a as f32) as u8,
        }
    }

    ///Obtener colores individuales
    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }
    pub fn a(&self) -> u8 { self.a }

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
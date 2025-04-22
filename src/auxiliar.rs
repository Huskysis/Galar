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
    let mut i = x.to_bits(); // Interpretar f32 como u32
    i = 0x5f3759df - (i >> 1); // El famoso número mágico
    let mut y = f32::from_bits(i); // Interpretar de nuevo como f32
    y = y * (1.5 - x_half * y * y); // Primera iteración de Newton-Raphson
    1.0 / y // Invertimos para obtener sqrt(x)
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
/// Value: valor a reasignar.
/// Start1: límite inferior del rango actual del valor
/// Stop1: límite superior del rango actual del valor
/// Start2: límite inferior del rango objetivo del valor
/// Stop2: límite superior del rango objetivo del valor
/// Bounds: restringe el valor al nuevo rango mapeado.
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

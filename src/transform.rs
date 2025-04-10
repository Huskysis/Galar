#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub translation: Translation,
    pub rotation: Angle, // en radianes
    pub scale: Scale,    // escala en x e y
}

impl Transform {
    pub fn identity() -> Self {
        Self::new(0, 0, 0.0, 1.0, 1.0)
    }

    pub fn new(x: usize, y: usize, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            translation: Translation { x, y },
            rotation: Angle(rotation),
            scale: Scale {
                x: scale_x,
                y: scale_y,
            },
        }
    }

    pub fn from_translation(x: usize, y: usize) -> Self {
        Self {
            translation: Translation { x, y },
            scale: Scale::new(1.0, 1.0),
            ..Default::default()
        }
    }
    pub fn apply(&self, local_x: f32, local_y: f32) -> (usize, usize) {
        let cos_r = self.rotation.0.cos();
        let sin_r = self.rotation.0.sin();

        // escalado y rotación
        let x = local_x * self.scale.x;
        let y = local_y * self.scale.y;

        // rotación
        let rotated_x = x * cos_r - y * sin_r;
        let rotated_y = x * sin_r + y * cos_r;

        // traslación
        let final_x = rotated_x + self.translation.x as f32;
        let final_y = rotated_y + self.translation.y as f32;

        (final_x.round() as usize, final_y.round() as usize)
    }

    pub fn apply_centered(
        &self,
        local_x: f32,
        local_y: f32,
        shape_width: f32,
        shape_height: f32,
    ) -> (usize, usize) {
        let cos_r = self.rotation.0.cos();
        let sin_r = self.rotation.0.sin();

        // 💠 Desplazamiento al centro del shape
        let cx = shape_width / 2.0;
        let cy = shape_height / 2.0;

        let x = (local_x - cx) * self.scale.x;
        let y = (local_y - cy) * self.scale.y;

        // 🎯 Rotación
        let rotated_x = x * cos_r - y * sin_r;
        let rotated_y = x * sin_r + y * cos_r;

        // 📦 Traslación final
        let final_x = rotated_x + self.translation.x as f32;
        let final_y = rotated_y + self.translation.y as f32;

        (final_x.round() as usize, final_y.round() as usize)
    }

    pub fn displace(&mut self, window: &minifb::Window) {
        self.translation.displace(window);
        self.rotation.displace(window);
        self.scale.displace(window);
    }
}

// Aqui empieza lo bueno
// impl PluginGalar for Transform {
//     fn update(&mut self, window: &Window, buffer: &mut Vec<u32>) {

//         self.displace(window);
//     }
// }

/// Aqui Angulo en grados, luego se pasa a radianes
#[derive(Debug, Default, Clone, Copy)]
pub struct Angle(pub f32);

impl Angle {
    pub fn displace(&mut self, window: &minifb::Window) {
        if window.is_key_down(minifb::Key::Q) {
            self.0 -= 0.01;
        }
        if window.is_key_down(minifb::Key::E) {
            self.0 += 0.01;
        }
        self.0 = self.0 % 360.0;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    pub const ZERO: Scale = Scale { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Scale { x, y }
    }
    pub fn displace(&mut self, window: &minifb::Window) {
        if window.is_key_down(minifb::Key::R) {
            self.x += 0.1;
            self.y += 0.1;
        }
        if window.is_key_down(minifb::Key::F) {
            self.x -= 0.1;
            self.y -= 0.1;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Translation {
    pub x: usize,
    pub y: usize,
}

impl Translation {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn displace(&mut self, window: &minifb::Window) {
        let (width, height) = window.get_size();

        // desplazamiento longitudinal
        if window.is_key_down(minifb::Key::A) {
            self.x = self.x.saturating_sub(1);
        }
        if window.is_key_down(minifb::Key::D) {
            self.x = self.x + 1;
        }
        self.x = self.x.clamp(0, width - 1);

        // desplazamiento latitudinal
        if window.is_key_down(minifb::Key::W) {
            self.y = self.y.saturating_sub(1);
        }
        if window.is_key_down(minifb::Key::S) {
            self.y = self.y + 1;
        }
        self.y = self.y.clamp(0, height - 1);
    }
}

use super::utils::ConfigGalar;

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub translation: Translation,
    pub rotation: Angle, // en radianes
    pub scale: Scale,    // escala en x e y
}

impl Transform {
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0, 1.0)
    }

    pub fn new(x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            translation: Translation { x, y },
            rotation: Angle(rotation),
            scale: Scale {
                x: scale_x,
                y: scale_y,
            },
        }
    }

    pub fn from_translation(x: f32, y: f32) -> Self {
        Self {
            translation: Translation { x, y },
            scale: Scale::new(1.0, 1.0),
            ..Default::default()
        }
    }
    pub fn apply(&self, local_x: f32, local_y: f32) -> (f32, f32) {
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

        (final_x, final_y)
    }

    // pub fn apply_centered(
    //     &self,
    //     local_x: f32,
    //     local_y: f32,
    //     shape_center_x: f32,
    //     shape_center_y: f32,
    // ) -> (f32, f32) {
    //     let cos_r = self.rotation.0.cos();
    //     let sin_r = self.rotation.0.sin();

    //     let x = (local_x - shape_center_x) * self.scale.x;
    //     let y = (local_y - shape_center_y) * self.scale.y;

    //     let rotated_x = x * cos_r - y * sin_r;
    //     let rotated_y = x * sin_r + y * cos_r;

    //     let final_x = rotated_x + self.translation.x as f32 + shape_center_x;
    //     let final_y = rotated_y + self.translation.y as f32 + shape_center_y;

    //     (final_x, final_y)
    // }
    pub fn apply_centered(
        &self,
        local_x: f32,
        local_y: f32,
        shape_center_x: f32,
        shape_center_y: f32,
    ) -> (f32, f32) {
        let cos_r = self.rotation.0.cos();
        let sin_r = self.rotation.0.sin();

        // Adjust for pixel center alignment
        let x = (local_x - shape_center_x) * self.scale.x;
        let y = (local_y - shape_center_y) * self.scale.y;

        let rotated_x = x * cos_r - y * sin_r;
        let rotated_y = x * sin_r + y * cos_r;

        // Apply rounding correction to avoid pixel shift
        let final_x = (rotated_x + self.translation.x + shape_center_x).round();
        let final_y = (rotated_y + self.translation.y + shape_center_y).round();

        (final_x, final_y)
    }

    pub fn displace(&mut self, config: &mut ConfigGalar) {
        let pos = &mut self.translation;

        // desplazamiento longitudinal
        if config.is_key_down(minifb::Key::A) {
            pos.x = pos.x - 1.0;
        }
        if config.is_key_down(minifb::Key::D) {
            pos.x = pos.x + 1.0;
        }

        // desplazamiento latitudinal
        if config.is_key_down(minifb::Key::W) {
            pos.y = pos.y - 1.0;
        }
        if config.is_key_down(minifb::Key::S) {
            pos.y = pos.y + 1.0;
        }

        if config.is_key_down(minifb::Key::Q) {
            self.rotation.0 -= 0.01;
        }
        if config.is_key_down(minifb::Key::E) {
            self.rotation.0 += 0.01;
        }
        self.rotation.0 = self.rotation.0 % 360.0;

        if config.is_key_down(minifb::Key::R) {
            self.scale.x += 0.1;
            self.scale.y += 0.1;
        }
        if config.is_key_down(minifb::Key::F) {
            self.scale.x -= 0.1;
            self.scale.y -= 0.1;
        }
    }
}

/// Aqui Angulo en grados, luego se pasa a radianes
#[derive(Debug, Default, Clone, Copy)]
pub struct Angle(pub f32);

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
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Translation {
    pub x: f32,
    pub y: f32,
}

impl Translation {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

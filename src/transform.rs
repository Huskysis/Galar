#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub translation: glam::Vec2,
    pub rotation: f32,   // en radianes
    pub scale: glam::Vec2, // escala en x e y
}

impl Transform {
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0, 1.0)
    }

    pub fn new(x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            translation: glam::Vec2 { x, y },
            rotation: rotation,
            scale: glam::Vec2 {
                x: scale_x,
                y: scale_y,
            },
        }
    }

    pub fn from_translation(x: f32, y: f32) -> Self {
        Self {
            translation: glam::Vec2 { x, y },
            scale: glam::Vec2::new(1.0, 1.0),
            ..Default::default()
        }
    }
    pub fn apply(&self, local_x: f32, local_y: f32) -> (f32, f32) {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

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

    pub fn apply_centered(
        &self,
        local_x: f32,
        local_y: f32,
        shape_center_x: f32,
        shape_center_y: f32,
    ) -> (f32, f32) {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

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
}

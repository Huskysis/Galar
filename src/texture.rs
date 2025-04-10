use crate::utils::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub path: String,
    pub data: Vec<u32>,
    pub size: (u32, u32),
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub base_color: Option<Color>,
    pub texture: Option<Texture>,
    // blend_mode: BlendMode,
}

impl Material {
    pub fn load_texture(path: &str) -> Material {
        use image::GenericImageView;

        let imagen = image::open(path).expect("Error loading texture");
        let (width, height) = imagen.dimensions();
        let rgba_img = imagen.to_rgba8();

        let mut pixels = Vec::with_capacity((width * height) as usize);
        for pixel in rgba_img.pixels() {
            let [r, g, b, a] = pixel.0;
            let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            pixels.push(color);
        }

        let texture = Texture {
            path: path.to_string(),
            data: pixels,
            size: (width, height),
        };

        Material {
            base_color: None,
            texture: Some(texture),
        }
    }
    pub fn sample_texture(&self, uv: (f32, f32)) -> u32 {
        let (mut u, mut v) = uv;

        u = u.clamp(0.0, 1.0);
        v = v.clamp(0.0, 1.0);

        let mut pixel: u32 = 0;
        if let Some(texture) = &self.texture {
            let width = texture.size.0;
            let height = texture.size.1;
            let x = (u * (width as f32 - 1.0)) as u32;
            let y = (v * (height as f32 - 1.0)) as u32;

            let index = (y * width + x) as usize;
            pixel = texture.data.get(index).copied().unwrap_or(0xFF00FFFF) // magenta if out-of-bounds
        }
        pixel
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    Additive,
    Multiply,
    Subtract,
    Alpha(f32),
}

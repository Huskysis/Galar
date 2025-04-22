use galar::prelude::*;

fn main() -> Result<(), String> {
    let mut galar = Galar::new("Voronoid Example", 400, 400, 60, None)?;

    let voronoid = Voronoi::new(10);

    println!("\nPresiona 'ESC' para cerrar la ventana 😉\n");

    galar.add_plugin(voronoid);

    galar.run()
}

#[derive(Default)]
struct Voronoi {
    points: Vec<(usize, usize)>,
    n_points: usize,
}

impl Voronoi {
    fn new(n_points: usize) -> Self {
        Self {
            points: Vec::new(),
            n_points,
        }
    }
}

impl PluginGalar for Voronoi {
    fn update(&mut self, config: &mut galar::utils::ConfigGalar) {
        let (width, height) = config.size();
        config.set_clean_pixels(false);
        for idx in 0..height * width {
            let x = idx % width;
            let y = idx / width;

            // limite para dibujar cada 2 pixeles
            if x % 2 == 0 && y % 2 == 0 {
                continue;
            }

            // let mut min = f32::MAX;
            // for point in &self.points {
            //     let d =
            //         proximidad(x as f32, y as f32, point.0 as f32, point.1 as f32);
            //     if d < min {
            //         min = d;
            //     }
            // }
            // let valor = min;
            // let r = remapear(valor, 150.0, 0.0, 0.0, 255.0, false).round() as u8;
            // let g = remapear(valor, 150.0, 0.0, 25.0, 0.0, false).round() as u8;
            // let b = remapear(valor, 150.0, 0.0, 155.0, 150.0, false).round() as u8;

            // let base_color = Color::rgb(r, g, b).to_hex();

            let closest = self
                .points
                .iter()
                .enumerate()
                .min_by_key(|(_, point)| {
                    Float(proximidad(
                        x as f32,
                        y as f32,
                        point.0 as f32,
                        point.1 as f32,
                    ))
                })
                .unwrap()
                .0;

            let base_color = match closest % 10 {
                0 => Color::rgb(255, 0, 0),
                1 => Color::rgb(0, 255, 0),
                2 => Color::rgb(0, 0, 255),
                3 => Color::rgb(255, 0, 255),
                4 => Color::rgb(255, 255, 0),
                5 => Color::rgb(0, 255, 255),
                6 => Color::rgb(89, 0, 255),
                7 => Color::rgb(0, 0, 0),
                8 => Color::rgb(255, 255, 255),
                9 => Color::rgb(255, 166, 0),
                _ => Color::random(),
            }
            .to_hex();

            config.draw_pixel(x, y, base_color);
        }
    }
    fn init(&mut self, config: &mut ConfigGalar) {
        config.set_frame_mode(FrameMode::SingleStep);

        if self.points.is_empty() {
            let (width, height) = config.size();
            for _ in 0..self.n_points {
                let x = random_range(0..width);
                let y = random_range(0..height);
                self.points.push((x, y));
            }
        }
    }
    fn name(&self) -> &str {
        "Voronoi Rustic"
    }
}

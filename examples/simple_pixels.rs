use galar::prelude::*;

fn main() -> Result<(), String> {
    let mut galar = Galar::new("Simple Geometry Example", 400, 400, 60, None)?;

    let marching_squared = SimpleGeometry;

    println!("\nPresiona 'ESC' para cerrar la ventana 😉\n");

    galar.add_plugin(marching_squared);

    galar.run()
}

struct SimpleGeometry;

impl PluginGalar for SimpleGeometry {
    fn update(&mut self, config: &mut ConfigGalar) {
        // dibuja un circulo
        config.draw_circle(100, 100, 50, Color::random().to_hex());

        // dibuja un rectangulo
        config.draw_rect(250, 100, 100, 100, Color::random().to_hex());

        // dibuja una linea
        config.draw_line(350, 300, 100, 250, Color::random().to_hex());

        let (width, height) = config.size();
        for _ in 0..10 {
            
            // dibuja pixeles segun las coordenadas
            config.draw_pixel(
                random_range(0..width - 1),
                random_range(0..height - 1),
                Color::random().to_hex(),
            );

            // dibuja pixeles segun el indice del frame buffer
            config.explicit_draw(
                random_range(0..(width - 1) * (height - 1)),
                Color::random().to_hex(),
            );
            
        }
    }
    fn init(&mut self, config: &mut ConfigGalar) {
        config.set_frame_mode(FrameMode::SingleStep);
    }
    fn name(&self) -> &str {
        "Simple Geometry Example"
    }
}

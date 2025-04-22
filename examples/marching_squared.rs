use galar::prelude::*;

fn main() -> Result<(), String> {
    let mut galar = Galar::new("Marching Squared Example", 400, 400, 60, None)?;

    let marching_squared = MarchingSquared::new(10);

    println!("\nPresiona 'ESC' para cerrar la ventana 😉\n");

    galar.add_plugin(marching_squared);

    galar.run()
}

#[derive(Debug, Default)]
struct MarchingSquared {
    campo: Vec<Vec<f32>>,
    resolucion: usize,
    filas: usize,
    columnas: usize,
}
impl MarchingSquared {
    fn new(resolucion: usize) -> Self {
        Self {
            campo: vec![],
            resolucion,
            filas: 1,
            columnas: 1,
        }
    }
}

impl PluginGalar for MarchingSquared {
    fn update(&mut self, config: &mut ConfigGalar) {
        // for i in 0..self.columnas {
        //     for j in 0..self.filas {
        for idx in 0..(self.columnas * self.filas) {
            let row = idx % self.filas;
            let col = idx / self.filas;
            let x = row * self.resolucion;
            let y = col * self.resolucion;

            let a = USizeVec2::new(x, y + self.resolucion / 2); // [0.0,0.5]
            let b = USizeVec2::new(x + self.resolucion / 2, y); // [0.5,0.0]
            let c = USizeVec2::new(x + self.resolucion, y + self.resolucion / 2); // [1.0,0.5]
            let d = USizeVec2::new(x + self.resolucion / 2, y + self.resolucion); // [0.5,1.0]

            let state = get_state(
                self.campo[col][row + 1].round() as u32,
                self.campo[col][row].round() as u32,
                self.campo[col + 1][row + 1].round() as u32,
                self.campo[col + 1][row].round() as u32,
            );

            match state {
                1 | 14 => draw_lines(config, a, d, self.campo[col][row + 1]),
                2 | 13 => draw_lines(config, d, c, self.campo[col][row]),
                3 | 12 => draw_lines(config, a, c, self.campo[col + 1][row + 1]),
                4 | 11 => draw_lines(config, a, b, self.campo[col + 1][row]),
                5 | 10 => draw_lines(config, b, d, self.campo[col][row + 1]),
                6 => {
                    draw_lines(config, a, b, self.campo[col][row]);
                    draw_lines(config, d, c, self.campo[col + 1][row + 1])
                }
                7 | 8 => draw_lines(config, b, c, self.campo[col + 1][row]),
                9 => {
                    draw_lines(config, a, d, self.campo[col][row + 1]);
                    draw_lines(config, b, c, self.campo[col][row])
                }
                _ => (),
            }
        }
    }
    fn init(&mut self, config: &mut ConfigGalar) {
        config.set_frame_mode(FrameMode::SingleStep);
        config.set_background(Color::BLACK.to_hex());

        let (width, height) = config.size();

        self.filas = width / self.resolucion;
        self.columnas = height / self.resolucion;

        self.campo = vec![vec![0f32; self.filas + 1]; self.columnas + 1];

        let total_rows = self.filas + 1;
        let total_cols = self.columnas + 1;

        for idx in 0..(total_cols * total_rows) {
            let col = idx / total_rows;
            let row = idx % total_rows;

            self.campo[col][row] = random_range(0f32..1.0);
        }
    }
    fn name(&self) -> &str {
        "Marching Squared"
    }
}

fn get_state(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a * 8 + b * 4 + c * 2 + d
}

fn draw_lines(config: &mut ConfigGalar, edge1: USizeVec2, edge2: USizeVec2, color: f32) {
    config.draw_line(edge1.x, edge1.y, edge2.x, edge2.y, Color::rainbow(color).to_hex());
}


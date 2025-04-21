use std::ops::Range;

use super::auxiliar::Color;
use minifb::{Window, WindowOptions};

pub trait PluginGalar {
    fn update(&mut self, config: &mut ConfigGalar);

    // Método opcional para inicialización
    fn init(&mut self, _config: &mut ConfigGalar) {
        println!("Initialize Plugin: {}", self.name());
    }

    // Método opcional para limpiar recursos
    fn cleanup(&mut self, _config: &mut ConfigGalar) {}

    // Nombre del plugin para depuración
    fn name(&self) -> &str {
        "UnnamedPlugin"
    }
}

// Configuración con lifetimes explícitos y mejor encapsulación
pub struct ConfigGalar<'g> {
    window: &'g mut Window,
    buffer: &'g mut Vec<u32>,
    clean: &'g mut bool,
    width: usize,
    height: usize,
    background: &'g mut u32,
    nonloop: &'g mut bool,
}

impl<'g> ConfigGalar<'g> {
    // Constructor privado usado solo por Galar
    fn new(
        window: &'g mut Window,
        buffer: &'g mut Vec<u32>,
        clean: &'g mut bool,
        background: &'g mut u32,
        nonloop: &'g mut bool,
    ) -> Self {
        let (width, height) = window.get_size();
        Self {
            window,
            buffer,
            clean,
            width,
            height,
            background,
            nonloop,
        }
    }

    /// Métodos públicos para consulta
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Mejor semántica: usa set_* para modificadores
    pub fn set_clean_pixels(&mut self, clear: bool) {
        *self.clean = clear;
    }

    /// Define el color del fondo
    pub fn set_background(&mut self, color: u32) {
        *self.background = color;
    }

    /// Se ejecuta 1 frame, ya no se ejecute cada frame
    pub fn nonloop(&mut self) {
        *self.nonloop = false;
    }

    /// Control fino sobre la limpieza
    pub fn clear_buffer(&mut self) {
        if *self.clean {
            self.buffer.fill(*self.background);
        }
    }

    /// Método para dibujar directamente
    pub fn explicit_draw(&mut self, index: usize, color: u32) {
        self.buffer[index] = color;
    }

    /// Iteración sobre una dimención segun la resolución de la pantalla
    pub fn iter_d1(&self) -> Range<usize> {
        0..(self.width * self.height)
    }

    /// Métodos para dibujar con validación de límites
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Optimización: dibujar segmentos de línea de manera eficiente
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        // Implementación de Bresenham para líneas
        let hex_color = color.to_hex();

        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let width = self.width as isize;
        let height = self.height as isize;

        loop {
            if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
                self.buffer[(y0 as usize) * self.width + (x0 as usize)] = hex_color;
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Métodos adicionales para formas rectangulares
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        let x_max = (x + width).min(self.width);
        let y_max = (y + height).min(self.height);
        let hex_color = color.to_hex();

        for curr_y in y..y_max {
            for curr_x in x..x_max {
                self.buffer[curr_y * self.width + curr_x] = hex_color;
            }
        }
    }

    /// Métodos adicionales para formas circulares
    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: u32) {
        let hex_color = color;
        let radius = radius as isize;

        let r2 = radius * radius;

        let width = self.width as isize;
        let height = self.height as isize;

        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= r2 {
                    let px = cx as isize + x;
                    let py = cy as isize + y;
                    if px >= 0 && px < width && py >= 0 && py < height {
                        self.buffer[(py as usize) * self.width + (px as usize)] = hex_color;
                    }
                }
            }
        }
    }

    /// WIREFRAME: Métodos adicionales para formas circulares
    pub fn draw_circle_outline(&mut self, cx: isize, cy: isize, radius: isize, color: Color) {
        let hex_color = color.to_hex();
        let r2 = radius * radius;
        let inner_r2 = (radius - 1) * (radius - 1);

        let width = self.width as isize;
        let height = self.height as isize;

        for y in -radius..=radius {
            for x in -radius..=radius {
                let dist2 = x * x + y * y;
                if dist2 <= r2 && dist2 >= inner_r2 {
                    let px = cx + x;
                    let py = cy + y;
                    if px >= 0 && px < width && py >= 0 && py < height {
                        self.buffer[(py as usize) * self.width + (px as usize)] = hex_color;
                    }
                }
            }
        }
    }

    // Acceso controlado a las pulsaciones de teclas
    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }

    // Obtorga las posiciones logicas del Mouse
    pub fn get_mouse_position(&mut self, mode: minifb::MouseMode) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(mode)
    }

    // Acceso controlado al estado de la ventana
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

// Motor principal con mejor gestión de recursos
pub struct Galar {
    window: Window,
    buffer: Vec<u32>,
    clean: bool,
    background: u32,
    plugins: Vec<Box<dyn PluginGalar>>,
    running: bool,
    nonloop: bool,
}

impl Galar {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
        framerate: usize,
        options: Option<WindowOptions>,
    ) -> Result<Self, String> {
        let window = Window::new(
            name,
            width,
            height,
            options.unwrap_or(WindowOptions::default()),
        )
        .map_err(|e| format!("Failed to create window: {}", e))?;

        let mut window_instance = window;
        window_instance.set_target_fps(framerate);

        let buffer = vec![0u32; width * height];

        Ok(Self {
            window: window_instance,
            buffer,
            clean: true,
            background: 0,
            plugins: Vec::new(),
            running: false,
            nonloop: false,
        })
    }

    // Builder pattern para una API más fluida
    pub fn with_clean_pixels(mut self, clear: bool) -> Self {
        self.clean = clear;
        self
    }

    // Añadir plugins con verificación
    pub fn add_plugin<P: PluginGalar + 'static>(&mut self, mut plugin: P) -> &mut Self {
        // Configuración temporal para inicializar el plugin
        let mut config = ConfigGalar::new(
            &mut self.window,
            &mut self.buffer,
            &mut self.clean,
            &mut self.background,
            &mut self.nonloop,
        );

        // Inicializar el plugin, para pre-configuración
        plugin.init(&mut config);

        // Añadir a la lista
        self.plugins.push(Box::new(plugin));
        self
    }

    // Método para detener el bucle de ejecución desde fuera
    pub fn stop(&mut self) {
        self.running = false;
    }

    // Método para ejecutar una vez el dibujo
    pub fn nonloop(&mut self) {
        self.nonloop = false;
    }

    // Bucle principal con mejor manejo de errores
    pub fn run(&mut self) -> Result<(), String> {
        if self.plugins.is_empty() {
            eprintln!("No plugins added. Add at least one plugin before running.");
            return Err("No plugins added. Add at least one plugin before running.".to_string());
        }

        let (width, height) = self.window.get_size();
        self.running = true;

        let mut local_nonloop = true;

        // Bucle principal
        while self.running && self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape)
        {
            if local_nonloop {
                // Crear configuración para este frame
                let mut config = ConfigGalar::new(
                    &mut self.window,
                    &mut self.buffer,
                    &mut self.clean,
                    &mut self.background,
                    &mut self.nonloop,
                );

                // Limpiar buffer si es necesario
                config.clear_buffer();

                // Actualizar todos los plugins
                for plugin in self.plugins.iter_mut() {
                    plugin.update(&mut config);
                }
                local_nonloop = self.nonloop;
            }

            // Actualizar la ventana con el buffer
            self.window
                .update_with_buffer(&self.buffer, width, height)
                .map_err(|e| format!("Failed to update window: {}", e))?;
        }

        // Limpiar recursos de plugins
        let mut config = ConfigGalar::new(
            &mut self.window,
            &mut self.buffer,
            &mut self.clean,
            &mut self.background,
            &mut self.nonloop,
        );

        for plugin in self.plugins.iter_mut() {
            plugin.cleanup(&mut config);
        }

        Ok(())
    }
}

// Implementación de Drop para limpieza segura
impl Drop for Galar {
    fn drop(&mut self) {
        // Asegurar que todos los plugins liberen sus recursos
        if !self.plugins.is_empty() {
            let mut config = ConfigGalar::new(
                &mut self.window,
                &mut self.buffer,
                &mut self.clean,
                &mut self.background,
                &mut self.nonloop,
            );

            for plugin in self.plugins.iter_mut() {
                plugin.cleanup(&mut config);
            }
        }
    }
}

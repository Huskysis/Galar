use super::{
    colores::Color,
    shape::{GeometryShape, Shape, Vertex},
    texture::Material,
    transform::Transform,
    utils::ConfigGalar,
};

pub fn draw_circle(x: f32, y: f32, radio: f32, segments: usize, color: Color) -> GeometryShape {
    GeometryShape::Circle {
        x,
        y,
        radio,
        segments,
        color,
    }
}

pub fn draw_square(x: f32, y: f32, size: f32, color: Color) -> GeometryShape {
    GeometryShape::Square { x, y, size, color }
}

pub fn draw_triangle(x: f32, y: f32, size: f32, color: Color) -> GeometryShape {
    GeometryShape::Triangle { x, y, size, color }
}

pub fn draw_shape(config: &mut ConfigGalar, shape: &Shape) {
    for triangle in &shape.indices {
        let v0 = &shape.vertices[triangle[0]];
        let v1 = &shape.vertices[triangle[1]];
        let v2 = &shape.vertices[triangle[2]];

        draw_triangle_cpu(config, shape, &shape.transform, v0, v1, v2, &shape.material);
    }
}

fn draw_triangle_cpu(
    config: &mut ConfigGalar,
    shape: &Shape,
    transform: &Transform,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    material: &Material,
) {
    let (width, _) = config.size();

    let p0: (f32, f32);
    let p1: (f32, f32);
    let p2: (f32, f32);
    if shape.origen {
        // Dimensiones aproximadas del shape (puede venir de otra parte)
        let (center_x, center_y) = shape.get_center();

        // Aplicamos transformación centrada
        p0 = transform.apply_centered(v0.x, v0.y, center_x, center_y);
        p1 = transform.apply_centered(v1.x, v1.y, center_x, center_y);
        p2 = transform.apply_centered(v2.x, v2.y, center_x, center_y);
    } else {
        p0 = transform.apply(v0.x, v0.y);
        p1 = transform.apply(v1.x, v1.y);
        p2 = transform.apply(v2.x, v2.y);
    }

    // bounding box con clamp
    let min_x = p0.0.min(p1.0).min(p2.0).trunc().max(0.0) as usize;
    let max_x = p0.0.max(p1.0).max(p2.0).trunc().min(width as f32 - 1.0) as usize;

    let min_y = p0.1.min(p1.1).min(p2.1).trunc().max(0.0) as usize;
    let max_y = p0.1.max(p1.1).max(p2.1).trunc().min(width as f32 - 1.0) as usize;

    let area = edge_function_f32(p0, p1, p2);

    let draw_width = max_x - min_x + 1;
    let draw_height = max_y - min_y + 1;

    for i in 0..(draw_width * draw_height) {
        let local_x = i % draw_width;
        let local_y = i / draw_width;
        let x = min_x + local_x;
        let y = min_y + local_y;
        let px = x as f32;
        let py = y as f32;

        let w0 = edge_function_f32((px, py), p1, p2) / area;
        let w1 = edge_function_f32((px, py), p2, p0) / area;
        let w2 = edge_function_f32((px, py), p0, p1) / area;

        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
            let index = y * width + x;
            let color = if let Some(color) = material.base_color {
                color.to_hex()
            } else {
                // Interpolación de color
                let r = (v0.color.r as f32 * w0 + v1.color.r as f32 * w1 + v2.color.r as f32 * w2)
                    .round() as u8;

                let g = (v0.color.g as f32 * w0 + v1.color.g as f32 * w1 + v2.color.g as f32 * w2)
                    .round() as u8;

                let b = (v0.color.b as f32 * w0 + v1.color.b as f32 * w1 + v2.color.b as f32 * w2)
                    .round() as u8;

                Color::rgb(r, g, b).to_hex()
            };
            if let Some(_) = material.texture {
                let uv_0 = v0.uv;
                let uv_1 = v1.uv;
                let uv_2 = v2.uv;
                let uv = (
                    w0 * uv_0.x + w1 * uv_1.x + w2 * uv_2.x,
                    w0 * uv_0.y + w1 * uv_1.y + w2 * uv_2.y,
                );
                config.explicit_draw(index, material.sample_texture(uv));
            } else {
                config.explicit_draw(index, color);
            }
        }
    }
}

fn edge_function_f32(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

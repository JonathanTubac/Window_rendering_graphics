use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }
    for i in 0..points.len() - 1 {
        line(framebuffer, points[i], points[i + 1]);
    }
    line(framebuffer, *points.last().unwrap(), points[0]);
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 3 {
        return;
    }

    let y_min = points.iter().map(|p| p.y as i32).min().unwrap();
    let y_max = points.iter().map(|p| p.y as i32).max().unwrap();

    let n = points.len();

    for y in y_min..=y_max {
        let mut intersections: Vec<i32> = Vec::new();

        for i in 0..n {
            let p0 = points[i];
            let p1 = points[(i + 1) % n];

            let y0 = p0.y as i32;
            let y1 = p1.y as i32;

            // La scanline cruza esta arista si y está dentro del rango vertical
            if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                let x = p0.x + (y as f32 - p0.y) * (p1.x - p0.x) / (p1.y - p0.y);
                intersections.push(x as i32);
            }
        }

        intersections.sort_unstable();

        // Rellenar entre pares de intersecciones
        for pair in intersections.chunks(2) {
            if let [x0, x1] = pair {
                for x in *x0..=*x1 {
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }
}

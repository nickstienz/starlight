use crate::primitives::color::*;
use super::super::settings::*;

pub fn ray_trace_pixel(x: u32, y: u32) -> Color {
    let (u, v) = ((x as f64) / WIDTH as f64, (y as f64) / HEIGHT as f64);

    if u > 0.3 && u < 0.7 && v > 0.3 && v < 0.7 {
        return Color::new(0.0, 0.0, 0.0, 1.0);
    }

    Color::new(u, v, 0.5, 1.0)
}

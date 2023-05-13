use crate::primitives::color::*;

pub fn ray_trace_pixel(x: u32, y: u32, width: u32, height: u32) -> [u8; 4] {
    let (u, v) = ((x as f64) / width as f64, (y as f64) / height as f64);

    if u > 0.3 && v > 0.3 && u < 0.7 && v < 0.7 {
        return Color::new(1.0, 1.0, 1.0, 1.0).to_u8();
    }

    Color::new(u, v, 0.5, 1.0).to_u8()
}

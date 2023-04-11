pub fn ray_trace_pixel(x: u32, y: u32, width: u32, height: u32) -> [u8; 4] {
    let u = (x as f64) / (width as f64);
    let v = (y as f64) / (height as f64);

    [
        (255.99 * u) as u8,
        (255.99 * v) as u8,
        (255.99 * 0.5) as u8,
        255,
    ]
}

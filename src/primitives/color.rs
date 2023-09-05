#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_u8(&self) -> [u8; 4] {
        [
            (255.99 * self.r) as u8,
            (255.99 * self.g) as u8,
            (255.99 * self.b) as u8,
            (255.99 * self.a) as u8,
        ]
    }
}

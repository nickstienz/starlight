use super::tuple::*;
use crate::utils::eq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    fn x(&self) -> f64 {
        self.r
    }

    fn y(&self) -> f64 {
        self.g
    }

    fn z(&self) -> f64 {
        self.b
    }
}

impl_add!(Color, Color, Color);
impl_sub!(Color, Color, Color);
impl_mul_scalar!(Color, f64, Color);
impl_mul!(Color, Color, Color);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 + c2;
        assert_eq!(c3.r(), 1.6);
        assert_eq!(c3.g(), 0.7);
        assert_eq!(c3.b(), 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 - c2;
        assert!(eq::equal(c3.r(), 0.2));
        assert!(eq::equal(c3.g(), 0.5));
        assert!(eq::equal(c3.b(), 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let c2 = c * 2.0;
        assert_eq!(c2.r(), 0.4);
        assert_eq!(c2.g(), 0.6);
        assert_eq!(c2.b(), 0.8);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let c3 = c1 * c2;
        assert!(eq::equal(c3.r(), 0.9));
        assert!(eq::equal(c3.g(), 0.2));
        assert!(eq::equal(c3.b(), 0.04));
    }
}

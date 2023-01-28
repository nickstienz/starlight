use crate::primitives::tuple::*;
use crate::primitives::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        1.0
    }
}

impl_sub!(Point, Point, Vector);
impl_sub!(Point, Vector, Point);
impl_add!(Point, Vector, Point);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creation() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x(), 4.3);
        assert_eq!(p.y(), -4.2);
        assert_eq!(p.z(), 3.1);
        assert_eq!(p.w(), 1.0);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let v = p1 - p2;
        assert_eq!(v.x(), -2.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), -6.0);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let p2 = p - v;
        assert_eq!(p2.x(), -2.0);
        assert_eq!(p2.y(), -4.0);
        assert_eq!(p2.z(), -6.0);
        assert_eq!(p2.w(), 1.0);
    }
}

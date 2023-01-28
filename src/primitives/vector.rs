use super::tuple::*;
use crate::primitives::point::Point;

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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
        0.0
    }
}

impl_add!(Vector, Vector, Vector);
impl_add!(Vector, Point, Point);
impl_sub!(Vector, Vector, Vector);
impl_neg!(Vector, Vector);

#[cfg(test)]
mod tests {
    use crate::primitives::point::Point;

    use super::*;

    #[test]
    fn vector_creation() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x(), 4.3);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn vector_addition() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 1.0);
        assert_eq!(v3.y(), 1.0);
        assert_eq!(v3.z(), 6.0);
        assert_eq!(v3.w(), 0.0);
    }

    #[test]
    fn vector_plus_point() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let p = Point::new(-2.0, 3.0, 1.0);
        let t = v + p;
        assert_eq!(t.x(), 1.0);
        assert_eq!(t.y(), 1.0);
        assert_eq!(t.z(), 6.0);
        assert_eq!(t.w(), 1.0);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -2.0);
        assert_eq!(v3.y(), -4.0);
        assert_eq!(v3.z(), -6.0);
        assert_eq!(v3.w(), 0.0);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = zero - v;
        assert_eq!(v2.x(), -1.0);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), -3.0);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = -v;
        assert_eq!(v2.x(), -1.0);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), -3.0);
        assert_eq!(v2.w(), 0.0);
    }
}

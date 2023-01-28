use super::tuple::*;
use crate::primitives::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        Vector::new(self.x / m, self.y / m, self.z / m)
    }

    pub fn dot(&self, rhs: Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
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
impl_mul_scalar!(Vector, f64, Vector);
impl_div_scalar!(Vector, f64, Vector);

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

    #[test]
    fn multiplying_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = v * 3.5;
        assert_eq!(v2.x(), 3.5);
        assert_eq!(v2.y(), -7.0);
        assert_eq!(v2.z(), 10.5);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn multiplying_vector_by_fraction() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = v * 0.5;
        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), -1.0);
        assert_eq!(v2.z(), 1.5);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = v / 2.0;
        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), -1.0);
        assert_eq!(v2.z(), 1.5);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn magnitude_of_vector_neg1_neg2_neg3() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let v2 = v.normalize();
        assert_eq!(v2.x(), 1.0);
        assert_eq!(v2.y(), 0.0);
        assert_eq!(v2.z(), 0.0);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = v.normalize();
        assert_eq!(v2.x(), 1.0 / (14.0 as f64).sqrt());
        assert_eq!(v2.y(), 2.0 / (14.0 as f64).sqrt());
        assert_eq!(v2.z(), 3.0 / (14.0 as f64).sqrt());
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = v.normalize();
        assert_eq!(v2.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_v3() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let v3 = v1.cross(v2);
        assert_eq!(v3.x(), -1.0);
        assert_eq!(v3.y(), 2.0);
        assert_eq!(v3.z(), -1.0);
        assert_eq!(v3.w(), 0.0);
    }

    #[test]
    fn cross_product_of_two_vectors_v4() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let v4 = v2.cross(v1);
        assert_eq!(v4.x(), 1.0);
        assert_eq!(v4.y(), -2.0);
        assert_eq!(v4.z(), 1.0);
        assert_eq!(v4.w(), 0.0);
    }
}

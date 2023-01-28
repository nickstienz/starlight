use super::matrix::*;
use crate::primitives::point::Point;
use crate::primitives::tuple::*;

pub struct Matrix4 {
    pub buffer: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn new(m: [[f64; 4]; 4]) -> Self {
        Matrix4 { buffer: m }
    }
}

impl Matrix for Matrix4 {
    fn zero() -> Self {
        Matrix4 {
            buffer: [[0.0; 4]; 4],
        }
    }

    fn get(&self, row: usize, column: usize) -> f64 {
        self.buffer[row][column]
    }

    fn set(&mut self, row: usize, column: usize, value: f64) {
        self.buffer[row][column] = value;
    }

    fn equals(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

matrix_impl_mul!(Matrix4, Matrix4, Matrix4);
matrix_impl_mul_tuple!(Matrix4, Point, Point);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix4_new() {
        let m = Matrix4::zero();
        assert_eq!(m.buffer, [[0.0; 4]; 4]);
    }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 3), 4.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a.equals(&b), true);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_eq!(a.equals(&b), false);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let c = a * b;

        assert_eq!(c.get(0, 0), 20.0);
        assert_eq!(c.get(0, 1), 22.0);
        assert_eq!(c.get(0, 2), 50.0);
        assert_eq!(c.get(0, 3), 48.0);
        assert_eq!(c.get(1, 0), 44.0);
        assert_eq!(c.get(1, 1), 54.0);
        assert_eq!(c.get(1, 2), 114.0);
        assert_eq!(c.get(1, 3), 108.0);
        assert_eq!(c.get(2, 0), 40.0);
        assert_eq!(c.get(2, 1), 58.0);
        assert_eq!(c.get(2, 2), 110.0);
        assert_eq!(c.get(2, 3), 102.0);
        assert_eq!(c.get(3, 0), 16.0);
        assert_eq!(c.get(3, 1), 26.0);
        assert_eq!(c.get(3, 2), 46.0);
        assert_eq!(c.get(3, 3), 42.0);
    }

    #[test]
    fn multiplying_a_matrix_by_a_point() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Point::new(1.0, 2.0, 3.0);

        let c = a * b;

        assert_eq!(c.x(), 18.0);
        assert_eq!(c.y(), 24.0);
        assert_eq!(c.z(), 33.0);
        assert_eq!(c.w(), 1.0);
    }
}

use super::matrix::Matrix;

pub struct Matrix2 {
    pub buffer: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn new(m: [[f64; 2]; 2]) -> Self {
        Matrix2 { buffer: m }
    }
}

impl Matrix for Matrix2 {
    fn zero() -> Self {
        Matrix2 {
            buffer: [[0.0; 2]; 2],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix2_new() {
        let m = Matrix2::zero();
        assert_eq!(m.buffer, [[0.0; 2]; 2]);
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let m = Matrix2::new([[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix2::new([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m.buffer, [[-3.0, 5.0], [1.0, -2.0]]);
    }
}

use super::matrix::Matrix;

pub struct Matrix3 {
    pub buffer: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn new(m: [[f64; 3]; 3]) -> Self {
        Matrix3 { buffer: m }
    }
}

impl Matrix for Matrix3 {
    fn zero() -> Self {
        Matrix3 {
            buffer: [[0.0; 3]; 3],
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
    fn test_matrix3_new() {
        let m = Matrix3::zero();
        assert_eq!(m.buffer, [[0.0; 3]; 3]);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let m = Matrix3::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 9.0]]);

        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 6.0);
        assert_eq!(m.get(2, 2), 9.0);
    }
}

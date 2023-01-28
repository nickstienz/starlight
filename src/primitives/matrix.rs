pub trait Matrix {
    fn zero() -> Self;
    fn get(&self, row: usize, column: usize) -> f64;
    fn set(&mut self, row: usize, column: usize, value: f64);
    fn equals(&self, other: &Self) -> bool;
}

// ft: from type, st: op type, rt: return type
macro_rules! matrix_impl_mul {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Mul<$ot> for $ft {
            type Output = $rt;
            fn mul(self, rhs: $ot) -> $rt {
                let mut result = <$rt>::zero();
                for (i, result_row) in result.buffer.iter_mut().enumerate() {
                    for (j, result_cell) in result_row.iter_mut().enumerate() {
                        *result_cell = (0..self.buffer.len())
                            .map(|k| self.get(i, k) * rhs.get(k, j))
                            .sum();
                    }
                }
                result
            }
        }
    };
}

pub(crate) use matrix_impl_mul;

// ft: from type, st: op type, rt: return type
macro_rules! matrix_impl_mul_tuple {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Mul<$ot> for $ft {
            type Output = $rt;
            fn mul(self, rhs: $ot) -> $rt {
                let mut result = <$rt>::zero();
                for (i, self_row) in self.buffer.iter().enumerate() {
                    for (j, self_cell) in self_row.iter().enumerate() {
                        result.set_at(i, result.get_at(i) + self_cell * rhs.get_at(j));
                    }
                }
                result
            }
        }
    };
}

pub(crate) use matrix_impl_mul_tuple;

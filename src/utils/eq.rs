use crate::primitives::tuple::Tuple;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::point::Point;

    #[test]
    fn equal_test() {
        assert!(equal(1.0, 1.0));
        assert!(!equal(1.0, 2.0));
        assert!(equal(1.00002, 1.00001));
    }
}

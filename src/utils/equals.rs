use crate::primitives::tuple::Tuple;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

pub fn equal_tuples<T>(a: T, b: T) -> bool
where
    T: Tuple,
{
    equal(a.x(), b.x()) && equal(a.y(), b.y()) && equal(a.z(), b.z())
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

    #[test]
    fn equal_tuples_test_ab() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(1.0, 2.0, 3.0);
        assert!(equal_tuples(a, b));
    }

    #[test]
    fn equal_tuples_test_ac() {
        let a = Point::new(1.0, 2.0, 3.0);
        let c = Point::new(1.0, 2.0, 4.0);
        assert!(!equal_tuples(a, c));
    }
}

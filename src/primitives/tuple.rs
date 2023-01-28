pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

// ft: from type, st: op type, rt: return type
macro_rules! impl_add {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Add<$ot> for $ft {
            type Output = $rt;
            fn add(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
            }
        }
    };
}

pub(crate) use impl_add;

// ft: from type, st: op type, rt: return type
macro_rules! impl_sub {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Sub<$ot> for $ft {
            type Output = $rt;
            fn sub(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
            }
        }
    };
}

pub(crate) use impl_sub;

// ft: from type, rt: return type
macro_rules! impl_neg {
    ($ft:ty, $rt:ty) => {
        impl std::ops::Neg for $ft {
            type Output = $rt;
            fn neg(self) -> $rt {
                <$rt>::new(-self.x(), -self.y(), -self.z())
            }
        }
    };
}

pub(crate) use impl_neg;

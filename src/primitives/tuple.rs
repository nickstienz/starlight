pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self
    where
        Self: Sized,
    {
        let mut new = Self::new(0.0, 0.0, 0.0);
        new.set_w(0.0);
        new
    }
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
    fn set_z(&mut self, z: f64);
    fn set_w(&mut self, w: f64);
    fn get_at(&self, index: usize) -> f64 {
        match index {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            3 => self.w(),
            _ => panic!("Index out of bounds"),
        }
    }
    fn set_at(&mut self, index: usize, value: f64) {
        match index {
            0 => self.set_x(value),
            1 => self.set_y(value),
            2 => self.set_z(value),
            3 => self.set_w(value),
            _ => panic!("Index out of bounds"),
        }
    }
}

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_add {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Add<$ot> for $ft {
            type Output = $rt;
            fn add(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
            }
        }
    };
}

pub(crate) use tuple_impl_add;

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_sub {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Sub<$ot> for $ft {
            type Output = $rt;
            fn sub(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
            }
        }
    };
}

pub(crate) use tuple_impl_sub;

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_mul {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Mul<$ot> for $ft {
            type Output = $rt;
            fn mul(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
            }
        }
    };
}

pub(crate) use tuple_impl_mul;

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_div {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Div<$ot> for $ft {
            type Output = $rt;
            fn div(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
            }
        }
    };
}

pub(crate) use tuple_impl_div;

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_mul_scalar {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Mul<$ot> for $ft {
            type Output = $rt;
            fn mul(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
            }
        }
    };
}

pub(crate) use tuple_impl_mul_scalar;

// ft: from type, st: op type, rt: return type
macro_rules! tuple_impl_div_scalar {
    ($ft:ty, $ot:ty, $rt:ty) => {
        impl std::ops::Div<$ot> for $ft {
            type Output = $rt;
            fn div(self, rhs: $ot) -> $rt {
                <$rt>::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
            }
        }
    };
}

pub(crate) use tuple_impl_div_scalar;

// ft: from type, rt: return type
macro_rules! tuple_impl_neg {
    ($ft:ty, $rt:ty) => {
        impl std::ops::Neg for $ft {
            type Output = $rt;
            fn neg(self) -> $rt {
                <$rt>::new(-self.x(), -self.y(), -self.z())
            }
        }
    };
}

pub(crate) use tuple_impl_neg;

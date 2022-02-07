use crate::features::primitives::operations::consts::{EPSILON, LOW_EPSILON};

pub trait Operations<Rhs = Self> {
    fn equals(self, _: Rhs) -> bool;
    fn equals_low_epsilon(self, _: Rhs) -> bool;
}

impl Operations for f64 {
    fn equals(self, b: f64) -> bool {
        if self.abs() == f64::INFINITY && b.abs() == f64::INFINITY {
            return self == b
        }
        let diff = self - b;
        diff.abs() < EPSILON
    }

    fn equals_low_epsilon(self, b: f64) -> bool {
        let diff = self - b;
        diff.abs() < LOW_EPSILON
    }
}

pub mod consts {
    pub const EPSILON: f64 = 0.00001;
    pub const LOW_EPSILON: f64 = 0.0001;
}
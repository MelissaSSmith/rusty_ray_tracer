use crate::features::operations::consts::EPSILON;

pub trait Operations<Rhs = Self> {
    fn equals(self, _: Rhs) -> bool;
}

impl Operations for f64 {
    fn equals(self, b: f64) -> bool {
        let diff = self - b;
        diff.abs() < EPSILON
    }
}

pub mod consts {
    pub const EPSILON: f64 = 0.00001;
}
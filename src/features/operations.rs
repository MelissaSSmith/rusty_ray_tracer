pub trait Operations {
    fn equals(self, _: f64) -> bool;
}

impl Operations for f64 {
    fn equals(self, b: f64) -> bool {
        let epsilon = 0.00001;
        let diff = self - b;
        diff.abs() < epsilon
    }
}

pub mod consts {
    pub const EPSILON: f64 = 0.00001;
}
use std::ops::{Div, Mul, Neg};
use crate::features::operations::Operations;

#[derive(Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn create(x: f64, y:f64, z:f64, w:f64) -> Tuple {
        Tuple{x, y, z, w}
    }

    pub fn add(&self, _tuple: Tuple) -> Tuple {
        Tuple{
            x: self.x + _tuple.x,
            y: self.y + _tuple.y,
            z: self.z + _tuple.z,
            w: self.w + _tuple.w
        }
    }

    pub fn subtract(&self, _tuple: Tuple) -> Tuple {
        Tuple{
            x: self.x - _tuple.x,
            y: self.y - _tuple.y,
            z: self.z - _tuple.z,
            w: self.w - _tuple.w
        }
    }

    pub fn negate(&self) -> Tuple {
        Tuple{
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
            w: self.w.neg()
        }
    }

    pub fn magnitude(&self) -> f64 {
        let base = self.x.powf(2.0) + self.y.powf(2.0) +
            self.z.powf(2.0) + self.w.powf(2.0);
        base.sqrt()
    }

    pub fn multiply(&self, _scalar: f64) -> Tuple {
        Tuple{
            x: self.x.mul(_scalar),
            y: self.y.mul(_scalar),
            z: self.z.mul(_scalar),
            w: self.w.mul(_scalar)
        }
    }

    pub fn divide(&self, _scalar: f64) -> Tuple {
        Tuple{
            x: self.x.div(_scalar),
            y: self.y.div(_scalar),
            z: self.z.div(_scalar),
            w: self.w.div(_scalar)
        }
    }

    pub fn dot(&self, _tuple: Tuple) -> f64 {
        let product = self.x * _tuple.x +
            self.y * _tuple.y +
            self.z * _tuple.z +
            self.w * _tuple.w;
        product
    }

    pub fn cross(&self, _tuple: Tuple) -> Tuple {
        Tuple {
            x: self.y * _tuple.z - self.z * _tuple.y,
            y: self.z * _tuple.x - self.x * _tuple.z,
            z: self.x * _tuple.y - self.y * _tuple.x,
            w: self.w
        }
    }

    pub fn equals(&self, _tuple: Tuple) -> bool {
        if self.x.equals(_tuple.x) &&
            self.y.equals(_tuple.y) &&
            self.z.equals(_tuple.z) &&
            self.w.equals(_tuple.w) {
            return true;
        }
        false
    }

    pub fn convert_to_tuple(vec: &Vec<f64>) -> Tuple {
        if vec.len() != 4 {
            panic!("Invalid vector size!");
        }
        Tuple {
            x: vec[0],
            y: vec[1],
            z: vec[2],
            w: vec[3]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::tuple::Tuple;

    #[test]
    fn test_add_tuple_types() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};
        let tuple_b = Tuple{x:5.0, y:6.0, z:7.0, w:4.0};

        let tuple = tuple_a.add(tuple_b);

        assert_eq!(8.0, tuple.x);
        assert_eq!(8.0, tuple.y);
        assert_eq!(8.0, tuple.z);
        assert_eq!(7.0, tuple.w);
    }

    #[test]
    fn test_subtract_tuple_types() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};
        let tuple_b = Tuple{x:5.0, y:6.0, z:7.0, w:4.0};

        let tuple = tuple_a.subtract(tuple_b);

        assert_eq!(-2.0, tuple.x);
        assert_eq!(-4.0, tuple.y);
        assert_eq!(-6.0, tuple.z);
        assert_eq!(-1.0, tuple.w);
    }

    #[test]
    fn test_negate_tuple_type() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};

        let tuple = tuple_a.negate();

        assert_eq!(-3.0, tuple.x);
        assert_eq!(-2.0, tuple.y);
        assert_eq!(-1.0, tuple.z);
        assert_eq!(-3.0, tuple.w);
    }

    #[test]
    fn test_multiply_tuple_type_by_scalar() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 3.5;

        let tuple = tuple_a.multiply(scalar);

        assert_eq!(3.5, tuple.x);
        assert_eq!(-7.0, tuple.y);
        assert_eq!(10.5, tuple.z);
        assert_eq!(-14.0, tuple.w);
    }

    #[test]
    fn test_multiply_tuple_type_by_fraction() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 0.5;

        let tuple = tuple_a.multiply(scalar);

        assert_eq!(0.5, tuple.x);
        assert_eq!(-1.0, tuple.y);
        assert_eq!(1.5, tuple.z);
        assert_eq!(-2.0, tuple.w);
    }

    #[test]
    fn test_divide_tuple_type_by_scalar() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 2.0;

        let tuple = tuple_a.divide(scalar);

        assert_eq!(0.5, tuple.x);
        assert_eq!(-1.0, tuple.y);
        assert_eq!(1.5, tuple.z);
        assert_eq!(-2.0, tuple.w);
    }

}

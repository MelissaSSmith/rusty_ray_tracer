use core::fmt;
use core::fmt::Formatter;
use core::ops::{Add, Mul, Sub};
use crate::features::primitives::operations::Operations;
use crate::features::primitives::tuple_trait::Tuple as TupleTrait;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Tuple {
    pub fn create(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple {x, y, z, w}
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

    pub fn equals(&self, _tuple: Tuple) -> bool {
        self.x.equals(_tuple.x) && self.y.equals(_tuple.y) && self.z.equals(_tuple.z) && self.w.equals(_tuple.w)
    }
}

impl TupleTrait for Tuple {
    fn create(x: f64, y: f64, z: f64) -> Self {
        Tuple::create(x, y, z, 0.0)
    }

    fn zero() -> Self {
        Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        self.w
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: self.w + rhs.w()
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
            w: self.w - rhs.w()
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl core::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

// "Cross" product
impl core::ops::Mul for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: self.w
        }
    }
}

// "Dot" product (or "scalar" product)
impl core::ops::BitXor for Tuple {
    type Output = f64;

    fn bitxor(self, rhs: Tuple) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl core::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl core::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z:{}, w: {}", self.x, self.y, self.z, self.w)
    }
}
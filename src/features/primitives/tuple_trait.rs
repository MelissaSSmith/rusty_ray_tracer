use crate::{features::primitives::matrix::Matrix, features::transformations};

pub trait Tuple {
    fn create(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

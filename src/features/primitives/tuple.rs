use crate::features::primitives::matrix::Matrix;


pub trait Tuple {
    fn create(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

// impl<T> Transform for T
//     where
//         T: Tuple + Copy,
// {
//     fn transform(self, transformation: &Matrix) -> Self {
//         *transformation * self
//     }
// }

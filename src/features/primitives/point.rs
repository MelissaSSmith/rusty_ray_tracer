use std::ops::{Add, Mul, Sub};
use crate::features::primitives::vector::Vector;
use serde::{Deserialize, Serialize};
use crate::features::primitives::operations::Operations;
use crate::features::primitives::tuple::Tuple;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Point {
    pub(crate) fn equals(&self, _point: Point) -> bool {
        self.x.equals(_point.x) && self.y.equals(_point.y) && self.z.equals(_point.z)
    }
}

impl Tuple for Point {
    fn create(x: f64, y: f64, z: f64) -> Self {
        Point{ x, y, z, w: 1.0 }
    }

    fn zero() -> Self {
        Point{
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
        1.0
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
            w: 1.0
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::create(
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z()
        )
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::tuple::Tuple;
    use super::*;

    #[test]
    fn test_add_vector_and_point_creates_point() {
        let point = Point::create(3.0, -2.0, 5.0);
        let vector = Vector::create(-2.0, 3.0, 1.0);

        let new_point = point + vector;

        let expected_point = Point::create(1.0, 1.0, 6.0);
        assert!(expected_point.equals(new_point));
    }

    #[test]
    fn test_subtract_points_creates_vector() {
        let point_a = Point::create(3.0, 2.0, 1.0);
        let point_b = Point::create(5.0, 6.0, 7.0);

        let vector = point_a - point_b;

        let expected_vector = Vector::create(-2.0, -4.0, -6.0);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_subtract_vector_from_point_creates_point() {
        let point_a = Point::create(3.0, 2.0, 1.0);
        let vector_a = Vector::create(5.0, 6.0, 7.0);

        let point = point_a - vector_a;

        let expected_point = Point::create(-2.0, -4.0, -6.0);
        assert!(expected_point.equals(point));
    }
}
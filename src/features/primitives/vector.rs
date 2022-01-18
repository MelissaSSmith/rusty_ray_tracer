use std::ops::{Add, Mul, Sub};
use serde::{Deserialize, Serialize};
use crate::features::primitives::operations::Operations;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0))
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }

    pub fn reflect(&self, normal: Vector) -> Vector {
        *self - (normal * 2.0) * (*self ^ normal)
    }

    pub fn equals(&self, _vector: Vector) -> bool {
        self.x.equals(_vector.x) && self.y.equals(_vector.y) && self.z.equals(_vector.z)
    }
}

impl Tuple for Vector {
    fn create(x: f64, y: f64, z: f64) -> Self {
        Vector{ x, y, z, w: 0.0}
    }

    fn zero() -> Self {
        Vector{
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
        0.0
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 0.0
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
            w: 0.0
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 0.0
        }
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

// "Cross" product
impl std::ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0.0
        }
    }
}

// "Dot" product (or "scalar" product)
impl std::ops::BitXor for Vector {
    type Output = f64;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 0.0
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Div;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;

    #[test]
    fn test_add_two_vectors_creates_new_vector() {
        let vector_a = Vector::create(3.0, -2.0, 5.0);
        let vector_b = Vector::create(-2.0, 3.0, 1.0);

        let new_vector = vector_a + vector_b;

        let expected_vector = Vector::create(1.0, 1.0, 6.0);
        assert!(expected_vector.equals(new_vector));
    }

    #[test]
    fn test_subtract_vectors_creates_vector() {
        let vector_a = Vector::create(3.0, 2.0, 1.0);
        let vector_b = Vector::create(5.0, 6.0, 7.0);

        let vector = vector_a - vector_b;

        let expected_vector = Vector::create(-2.0, -4.0, -6.0);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_multiply_vector_by_scalar() {
        let vector_a = Vector::create(1.0, -2.0, 3.0);
        let scalar = 3.5;

        let vector = vector_a * scalar;

        let expected_vector = Vector::create(3.5, -7.0, 10.5);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_negate_vector() {
        let vector_a = Vector::create(1.0, -2.0, 3.0);

        let vector = -vector_a;

        let expected_vector = Vector::create(-1.0, 2.0, -3.0);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_compute_vector_magnitude_x_is_1() {
        let vector = Vector::create(1.0, 0.0, 0.0);

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_y_is_1() {
        let vector = Vector::create(0.0, 1.0, 0.0);

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_z_is_1() {
        let vector = Vector::create(0.0, 0.0, 1.0);

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_1_2_3() {
        let vector = Vector::create(1.0, 2.0, 3.0);

        let magnitude = vector.magnitude();

        let base: f64 = 14.0;
        assert_eq!(base.sqrt(), magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_1_2_3_negated() {
        let vector = Vector::create(-1.0, -2.0, -3.0);

        let magnitude = vector.magnitude();

        let base: f64 = 14.0;
        assert_eq!(base.sqrt(), magnitude);
    }

    #[test]
    fn test_normalize_vector() {
        let vector = Vector::create(4.0, 0.0, 0.0);

        let normalized_vector = vector.normalize();

        let expected_vector = Vector::create(1.0, 0.0, 0.0);
        assert!(expected_vector.equals(normalized_vector));
    }

    #[test]
    fn test_normalize_vector_1_2_3() {
        let vector = Vector::create(1.0, 2.0, 3.0);

        let normalized_vector = vector.normalize();

        let base: f64 = 14.0;
        assert_eq!(vector.x.div(base.sqrt()), normalized_vector.x);
        assert_eq!(vector.y.div(base.sqrt()), normalized_vector.y);
        assert_eq!(vector.z.div(base.sqrt()), normalized_vector.z);
    }

    #[test]
    fn test_magnitude_of_normalized_vector_is_one() {
        let vector = Vector::create(1.0, 2.0, 3.0);

        let normalized_vector = vector.normalize();
        let magnitude = normalized_vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_dot_product_of_two_vectors() {
        let vector_a = Vector::create(1.0, 2.0, 3.0);
        let vector_b = Vector::create(2.0, 3.0, 4.0);

        let dot_product = vector_a ^ vector_b;

        assert_eq!(20.0, dot_product);
    }

    #[test]
    fn test_cross_product_of_two_vectors_a_and_b() {
        let vector_a = Vector::create(1.0, 2.0, 3.0);
        let vector_b = Vector::create(2.0, 3.0, 4.0);

        let cross_a_b = vector_a * vector_b;

        let expected_vector = Vector::create(-1.0, 2.0, -1.0);
        assert!(expected_vector.equals(cross_a_b));
    }

    #[test]
    fn test_cross_product_of_two_vectors_b_and_a() {
        let vector_a = Vector::create(1.0, 2.0, 3.0);
        let vector_b = Vector::create(2.0, 3.0, 4.0);

        let cross_b_a = vector_b * vector_a;

        let expected_vector = Vector::create(1.0, -2.0, 1.0);
        assert!(expected_vector.equals(cross_b_a));
    }

    #[test]
    fn test_reflect_vector_approaching_45_degrees() {
        let vector = Vector::create(1.0, -1.0, 0.0);
        let normal = Vector::create(0.0, 1.0, 0.0);

        let reflection = vector.reflect(normal);

        assert!(reflection.equals(Vector::create(1.0, 1.0, 0.0)));
    }

    #[test]
    fn test_reflect_vector_off_a_slanted_surface() {
        let vector = Vector::create(0.0, -1.0, 0.0);
        let normal = Vector::create(2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0);

        let reflection = vector.reflect(normal);

        assert!(reflection.equals(Vector::create(1.0, 0.0, 0.0)));
    }
}
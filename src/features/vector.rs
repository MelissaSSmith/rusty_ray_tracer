use std::ops::{Div, Neg};

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn add(&self, _vector: Vector) -> Vector {
        Vector{
            x: self.x + _vector.x,
            y: self.y + _vector.y,
            z: self.z + _vector.z
        }
    }

    pub fn subtract(&self, _vector: Vector) -> Vector {
        Vector{
            x: self.x - _vector.x,
            y: self.y - _vector.y,
            z: self.z - _vector.z
        }
    }

    pub fn negate(&self) -> Vector {
        Vector{
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg()
        }
    }

    pub fn magnitude(&self) -> f64 {
        let base = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
        base.sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: self.x.div(magnitude),
            y: self.y.div(magnitude),
            z: self.z.div(magnitude)
        }
    }

    pub fn dot(&self, _vector: Vector) -> f64 {
        let product = self.x * _vector.x +
            self.y * _vector.y +
            self.z * _vector.z;
        product
    }

    pub fn cross(&self, _vector: Vector) -> Vector {
        Vector {
            x: self.y * _vector.z - self.z * _vector.y,
            y: self.z * _vector.x - self.x * _vector.z,
            z: self.x * _vector.y - self.y * _vector.x
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Div;
    use crate::features::vector::Vector;

    #[test]
    fn test_add_two_vectors_creates_new_vector() {
        let vector_a = Vector{x: 3.0, y:-2.0, z:5.0};
        let vector_b = Vector{x: -2.0, y:3.0, z:1.0};

        let new_vector = vector_a.add(vector_b);

        assert_eq!(1.0, new_vector.x);
        assert_eq!(1.0, new_vector.y);
        assert_eq!(6.0, new_vector.z);
    }

    #[test]
    fn test_subtract_vectors_creates_vector() {
        let vector_a = Vector{x:3.0, y:2.0, z:1.0};
        let vector_b = Vector{x:5.0, y:6.0, z:7.0};

        let vector = vector_a.subtract(vector_b);

        assert_eq!(-2.0, vector.x);
        assert_eq!(-4.0, vector.y);
        assert_eq!(-6.0, vector.z);
    }

    #[test]
    fn test_negate_vector() {
        let vector_a = Vector{x:1.0, y:-2.0, z:3.0};

        let vector = vector_a.negate();

        assert_eq!(-1.0, vector.x);
        assert_eq!(2.0, vector.y);
        assert_eq!(-3.0, vector.z);
    }

    #[test]
    fn test_compute_vector_magnitude_x_is_1() {
        let vector = Vector{x:1.0, y:0.0, z:0.0};

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_y_is_1() {
        let vector = Vector{x:0.0, y:1.0, z:0.0};

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_z_is_1() {
        let vector = Vector{x:0.0, y:0.0, z:1.0};

        let magnitude = vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_1_2_3() {
        let vector = Vector{x:1.0, y:2.0, z:3.0};

        let magnitude = vector.magnitude();

        let base: f64 = 14.0;
        assert_eq!(base.sqrt(), magnitude);
    }

    #[test]
    fn test_compute_vector_magnitude_1_2_3_negated() {
        let vector = Vector{x:-1.0, y:-2.0, z:-3.0};

        let magnitude = vector.magnitude();

        let base: f64 = 14.0;
        assert_eq!(base.sqrt(), magnitude);
    }

    #[test]
    fn test_normalize_vector() {
        let vector = Vector{x:4.0, y:0.0, z:0.0};

        let normalized_vector = vector.normalize();

        assert_eq!(1.0, normalized_vector.x);
        assert_eq!(0.0, normalized_vector.y);
        assert_eq!(0.0, normalized_vector.z);
    }

    #[test]
    fn test_normalize_vector_1_2_3() {
        let vector = Vector{x:1.0, y:2.0, z:3.0};

        let normalized_vector = vector.normalize();

        let base: f64 = 14.0;
        assert_eq!(vector.x.div(base.sqrt()), normalized_vector.x);
        assert_eq!(vector.y.div(base.sqrt()), normalized_vector.y);
        assert_eq!(vector.z.div(base.sqrt()), normalized_vector.z);
    }

    #[test]
    fn test_magnitude_of_normalized_vector_is_one() {
        let vector = Vector{x:1.0, y:2.0, z:3.0};

        let normalized_vector = vector.normalize();
        let magnitude = normalized_vector.magnitude();

        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn test_dot_product_of_two_vectors() {
        let vector_a = Vector{x:1.0, y:2.0, z:3.0};
        let vector_b = Vector{x:2.0, y:3.0, z:4.0};

        let dot_product = vector_a.dot(vector_b);

        assert_eq!(20.0, dot_product);
    }

    #[test]
    fn test_cross_product_of_two_vectors_a_and_b() {
        let vector_a = Vector{x:1.0, y:2.0, z:3.0};
        let vector_b = Vector{x:2.0, y:3.0, z:4.0};

        let cross_a_b = vector_a.cross(vector_b);

        assert_eq!(-1.0, cross_a_b.x);
        assert_eq!(2.0, cross_a_b.y);
        assert_eq!(-1.0, cross_a_b.z);
    }

    #[test]
    fn test_cross_product_of_two_vectors_b_and_a() {
        let vector_a = Vector{x:1.0, y:2.0, z:3.0};
        let vector_b = Vector{x:2.0, y:3.0, z:4.0};

        let cross_b_a = vector_b.cross(vector_a);

        assert_eq!(1.0, cross_b_a.x);
        assert_eq!(-2.0, cross_b_a.y);
        assert_eq!(1.0, cross_b_a.z);
    }
}
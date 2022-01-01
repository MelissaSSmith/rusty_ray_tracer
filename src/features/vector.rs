use crate::features::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct Vector {
    tuple: Tuple
}

impl Vector {
    pub fn create(x: f64, y:f64, z:f64) -> Vector {
        Vector{tuple: Tuple::create(x, y, z, 0.0)}
    }

    pub(crate) fn create_with_tuple(mut tuple: Tuple) -> Vector {
        tuple.w = 0.0;
        Vector{tuple}
    }

    pub fn value(&self) -> Tuple {
        self.tuple
    }

    pub fn add(&self, _vector: Vector) -> Vector {
        let tuple = self.tuple.add(_vector.tuple);
        Vector::create_with_tuple(tuple)
    }

    pub fn subtract(&self, _vector: Vector) -> Vector {
        let tuple = self.tuple.subtract(_vector.tuple);
        Vector::create_with_tuple(tuple)
    }

    pub fn multiply(&self, _scalar: f64) -> Vector {
        let tuple = self.tuple.multiply(_scalar);
        Vector::create_with_tuple(tuple)
    }

    pub fn negate(&self) -> Vector {
        let tuple = self.tuple.negate();
        Vector::create_with_tuple(tuple)
    }

    pub fn magnitude(&self) -> f64 {
        self.tuple.magnitude()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        let tuple = self.tuple.divide(magnitude);
        Vector::create_with_tuple(tuple)
    }

    pub fn dot(&self, _vector: Vector) -> f64 {
        self.tuple.dot(_vector.tuple)
    }

    pub fn cross(&self, _vector: Vector) -> Vector {
        let tuple = self.tuple.cross(_vector.tuple);
        Vector::create_with_tuple(tuple)
    }

    fn reflect(&self, normal: Vector) -> Vector {
        let temp = normal.multiply(2.0).multiply(self.dot(normal));
        self.subtract(temp)
    }

    pub(crate) fn equals(&self, _vector: Vector) -> bool {
        self.tuple.equals(_vector.tuple)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Div;
    use crate::features::vector::Vector;

    #[test]
    fn test_add_two_vectors_creates_new_vector() {
        let vector_a = Vector::create(3.0, -2.0, 5.0);
        let vector_b = Vector::create(-2.0, 3.0, 1.0);

        let new_vector = vector_a.add(vector_b);

        let expected_vector = Vector::create(1.0, 1.0, 6.0);
        assert!(expected_vector.equals(new_vector));
    }

    #[test]
    fn test_subtract_vectors_creates_vector() {
        let vector_a = Vector::create(3.0, 2.0, 1.0);
        let vector_b = Vector::create(5.0, 6.0, 7.0);

        let vector = vector_a.subtract(vector_b);

        let expected_vector = Vector::create(-2.0, -4.0, -6.0);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_multiply_tuple_type_by_scalar() {
        let vector_a = Vector::create(1.0, -2.0, 3.0);
        let scalar = 3.5;

        let vector = vector_a.multiply(scalar);

        let expected_vector = Vector::create(3.5, -7.0, 10.5);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_negate_vector() {
        let vector_a = Vector::create(1.0, -2.0, 3.0);

        let vector = vector_a.negate();

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
        assert_eq!(vector.tuple.x.div(base.sqrt()), normalized_vector.tuple.x);
        assert_eq!(vector.tuple.y.div(base.sqrt()), normalized_vector.tuple.y);
        assert_eq!(vector.tuple.z.div(base.sqrt()), normalized_vector.tuple.z);
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

        let dot_product = vector_a.dot(vector_b);

        assert_eq!(20.0, dot_product);
    }

    #[test]
    fn test_cross_product_of_two_vectors_a_and_b() {
        let vector_a = Vector::create(1.0, 2.0, 3.0);
        let vector_b = Vector::create(2.0, 3.0, 4.0);

        let cross_a_b = vector_a.cross(vector_b);

        let expected_vector = Vector::create(-1.0, 2.0, -1.0);
        assert!(expected_vector.equals(cross_a_b));
    }

    #[test]
    fn test_cross_product_of_two_vectors_b_and_a() {
        let vector_a = Vector::create(1.0, 2.0, 3.0);
        let vector_b = Vector::create(2.0, 3.0, 4.0);

        let cross_b_a = vector_b.cross(vector_a);

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
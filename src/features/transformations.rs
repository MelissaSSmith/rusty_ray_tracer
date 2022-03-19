use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;

pub trait Transform<Rhs = Self> {
    type Output;
    fn transform(self, matrix: Matrix) -> Self::Output;
}

impl Matrix {
    pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);
        matrix
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);
        matrix
    }

    pub fn rotate_x(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(1, 1, radians.cos());
        matrix.set(1, 2, -radians.sin());
        matrix.set(2, 1, radians.sin());
        matrix.set(2, 2, radians.cos());
        matrix
    }

    pub fn rotate_y(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 0, radians.cos());
        matrix.set(0, 2, radians.sin());
        matrix.set(2, 0, -radians.sin());
        matrix.set(2, 2, radians.cos());
        matrix
    }

    pub fn rotate_z(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 0, radians.cos());
        matrix.set(0, 1, -radians.sin());
        matrix.set(1, 0, radians.sin());
        matrix.set(1, 1, radians.cos());
        matrix
    }

    pub fn shear(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 1, x_y);
        matrix.set(0, 2, x_z);
        matrix.set(1, 0, y_x);
        matrix.set(1, 2, y_z);
        matrix.set(2, 0, z_x);
        matrix.set(2, 1, z_y);
        matrix
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let left = forward * up.normalize();
        let true_up = left * forward;
        let vec_1 = [left.x(), left.y(), left.z(), 0.0];
        let vec_2 = [true_up.x(), true_up.y(), true_up.z(), 0.0];
        let vec_3 = [-forward.x(), -forward.y(), -forward.z(), 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        let orientation = Matrix::create([vec_1, vec_2, vec_3, vec_4]);
        let translation = Matrix::translate(-from.x(), -from.y(), -from.z());
        orientation * translation
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;

    #[test]
    fn test_multiply_by_a_translation_matrix() {
        let transform = Matrix::translate(5.0, -3.0, 2.0);
        let point = Point::create(-3.0, 4.0, 5.0);

        let translation = transform * point;

        let expected_point = Point::create(2.0, 1.0, 7.0);

        assert!(expected_point.equals(translation))
    }

    #[test]
    fn test_multiply_by_inverse_of_translation_matrix() {
        let transform = Matrix::translate(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let point = Point::create(-3.0, 4.0, 5.0);

        let translation = inverse * point;

        let expected_point = Point::create(-8.0, 7.0, 3.0);

        assert!(expected_point.equals(translation))
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = Matrix::translate(5.0, -3.0, 2.0);
        let vector = Vector::create(-3.0, 4.0, 5.0);

        let translation = transform * vector;

        assert!(vector.equals(translation))
    }

    #[test]
    fn test_scaling_matrix_applied_to_point() {
        let transform = Matrix::scale(2.0, 3.0, 4.0);
        let point = Point::create(-4.0, 6.0, 8.0);

        let scaled_point = transform * point;

        let expected_point = Point::create(-8.0, 18.0, 32.0);
        assert!(expected_point.equals(scaled_point));
    }

    #[test]
    fn test_scaling_matrix_applied_to_vector() {
        let transform = Matrix::scale(2.0, 3.0, 4.0);
        let vector = Vector::create(-4.0, 6.0, 8.0);

        let scaled_vector = transform * vector;

        let expected_vector = Vector::create(-8.0, 18.0, 32.0);
        assert!(expected_vector.equals(scaled_vector));
    }

    #[test]
    fn test_multiply_by_inverse_of_scaling_matrix() {
        let transform = Matrix::scale(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let vector = Vector::create(-4.0, 6.0, 8.0);

        let scaled_vector = inverse * vector;

        let expected_vector = Vector::create(-2.0, 2.0, 2.0);
        assert!(expected_vector.equals(scaled_vector));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scale(-1.0, 1.0, 1.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let scaled_point = transform * point;

        let expected_point = Point::create(-2.0, 3.0, 4.0);
        assert!(expected_point.equals(scaled_point));
    }

    #[test]
    fn test_rotate_point_around_x_axis() {
        let point = Point::create(0.0, 1.0, 0.0);
        let half_quarter_radians = PI / 4.0;
        let half_quarter = Matrix::rotate_x(half_quarter_radians);

        let rotated_point = half_quarter * point;

        let expected_half_quarter_point = Point::create(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0);
        assert!(expected_half_quarter_point.equals(rotated_point));

        let point = Point::create(0.0, 1.0, 0.0);
        let full_quarter_radians = PI / 2.0;
        let full_quarter = Matrix::rotate_x(full_quarter_radians);

        let rotated_point = full_quarter * point;

        let expected_half_quarter_point = Point::create(0.0, 0.0, 1.0);
        assert!(expected_half_quarter_point.equals(rotated_point));
    }

    #[test]
    fn test_inverse_of_x_rotation_rotates_in_opposite_direction() {
        let point = Point::create(0.0, 1.0, 0.0);
        let half_quarter_radians = PI / 4.0;
        let half_quarter = Matrix::rotate_x(half_quarter_radians);
        let inverse = half_quarter.inverse();

        let rotated_point = inverse * point;

        let expected_half_quarter_point = Point::create(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        assert!(expected_half_quarter_point.equals(rotated_point));
    }

    #[test]
    fn test_rotate_point_around_y_axis() {
        let point = Point::create(0.0, 0.0, 1.0);
        let half_quarter_radians = PI / 4.0;
        let half_quarter = Matrix::rotate_y(half_quarter_radians);

        let rotated_point = half_quarter * point;

        let expected_half_quarter_point = Point::create(2.0_f64.sqrt()/2.0, 0.0, 2.0_f64.sqrt()/2.0);
        assert!(expected_half_quarter_point.equals(rotated_point));

        let point = Point::create(0.0, 0.0, 1.0);
        let full_quarter_radians = PI / 2.0;
        let full_quarter = Matrix::rotate_y(full_quarter_radians);

        let rotated_point = full_quarter * point;

        let expected_full_quarter_point = Point::create(1.0, 0.0, 0.0);
        assert!(expected_full_quarter_point.equals(rotated_point));
    }

    #[test]
    fn test_rotate_point_around_z_axis() {
        let point = Point::create(0.0, 1.0, 0.0);
        let half_quarter_radians = PI / 4.0;
        let half_quarter = Matrix::rotate_z(half_quarter_radians);

        let rotated_point = half_quarter * point;

        let expected_half_quarter_point = Point::create(-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0);
        assert!(expected_half_quarter_point.equals(rotated_point));

        let point = Point::create(0.0, 1.0, 0.0);
        let full_quarter_radians = PI / 2.0;
        let full_quarter = Matrix::rotate_z(full_quarter_radians);

        let rotated_point = full_quarter * point;

        let expected_half_quarter_point = Point::create(-1.0, 0.0, 0.0);
        assert!(expected_half_quarter_point.equals(rotated_point));
    }

    #[test]
    fn test_shearing_moves_x_in_proportion_to_y() {
        let transform = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(5.0, 3.0, 4.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_shearing_moves_x_in_proportion_to_z() {
        let transform = Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(6.0, 3.0, 4.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_shearing_moves_y_in_proportion_to_x() {
        let transform = Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(2.0, 5.0, 4.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_shearing_moves_y_in_proportion_to_z() {
        let transform = Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(2.0, 7.0, 4.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_shearing_moves_z_in_proportion_to_x() {
        let transform = Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(2.0, 3.0, 6.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_shearing_moves_z_in_proportion_to_y() {
        let transform = Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Point::create(2.0, 3.0, 4.0);

        let sheared_point = transform * point;

        let expected_point = Point::create(2.0, 3.0, 7.0);
        assert!(expected_point.equals(sheared_point));
    }

    #[test]
    fn test_individual_transformations_are_applied_in_sequence() {
        let point = Point::create(1.0, 0.0, 1.0);
        let a = Matrix::rotate_x(PI/2.0);
        let b = Matrix::scale(5.0, 5.0, 5.0);
        let c = Matrix::translate(10.0, 5.0, 7.0);

        let transformed_point = a * point;
        let expected_point = Point::create(1.0, -1.0, 0.0);
        assert!(expected_point.equals(transformed_point));

        let transformed_point = b * transformed_point;
        let expected_point = Point::create(5.0, -5.0, 0.0);
        assert!(expected_point.equals(transformed_point));

        let transformed_point = c * transformed_point;
        let expected_point = Point::create(15.0, 0.0, 7.0);
        assert!(expected_point.equals(transformed_point));
    }

    #[test]
    fn test_chained_transformations_in_reverse_order() {
        let point = Point::create(1.0, 0.0, 1.0);
        let a = Matrix::rotate_x(PI/2.0);
        let b = Matrix::scale(5.0, 5.0, 5.0);
        let c = Matrix::translate(10.0, 5.0, 7.0);

        let transformed_point = c * b * a * point; //todo fix

        let expected_point = Point::create(15.0, 0.0, 7.0);
        assert!(expected_point.equals(transformed_point));
    }

    #[test]
    fn test_transformation_matrix_for_the_default_orientation() {
        let from = Point::zero();
        let to = Point::create(0.0, 0.0, -1.0);
        let up = Vector::create(0.0, 1.0, 0.0);

        let matrix = Matrix::view_transform(from, to, up);

        assert!(matrix.equals(Matrix::identity()));
    }

    #[test]
    fn test_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::zero();
        let to = Point::create(0.0, 0.0, 1.0);
        let up = Vector::create(0.0, 1.0, 0.0);

        let matrix = Matrix::view_transform(from, to, up);

        assert!(matrix.equals(Matrix::scale(-1.0, 1.0, -1.0)));
    }

    #[test]
    fn test_transformation_matrix_moves_the_world() {
        let from = Point::create(0.0, 0.0, 8.0);
        let to = Point::zero();
        let up = Vector::create(0.0, 1.0, 0.0);

        let matrix = Matrix::view_transform(from, to, up);

        assert!(matrix.equals(Matrix::translate(0.0, 0.0, -8.0)));
    }

    #[test]
    fn test_arbitrary_view_transformation() {
        let from = Point::create(1.0, 3.0, 2.0);
        let to = Point::create(4.0, -2.0, 8.0);
        let up = Vector::create(1.0, 1.0, 0.0);

        let matrix = Matrix::view_transform(from, to, up);

        let vec_1 = [-0.50709, 0.50709, 0.67612, -2.36643];
        let vec_2 = [0.76772, 0.60609, 0.12122, -2.82843];
        let vec_3 = [-0.35857, 0.59761, -0.71714, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        assert!(matrix.equals(Matrix::create([vec_1, vec_2, vec_3, vec_4])));
    }
}
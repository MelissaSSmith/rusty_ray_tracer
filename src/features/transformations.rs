
fn translation(x: &f64, y: &f64, z: &f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::features::point::Point;
    use crate::features::transformations::translation;

    #[test]
    fn test_multiply_by_a_translation_matrix() {
        // let transform = translation(&5.0, &-3.0, &2.0);
        // let point = Point::create(-3.0, 4.0, 5.0);
        //
        // let translation = point.multiply(transform);
        //
        // let expected_point = Point::create(2.0, 1.0, 7.0);
        //
        // assert_eq!(expected_point.x, translation.x);
        // assert_eq!(expected_point.y, translation.y);
        // assert_eq!(expected_point.z, translation.z);
    }

    #[test]
    fn test_multiply_by_inverse_of_translation_matrix() {
        // let transform = translation(&5.0, &-3.0, &2.0);
        // let inverse = transform.inverse();
        // let point = Point::create(-3.0, 4.0, 5.0);
        //
        // let translation = point.multiply(inverse);
        //
        // let expected_point = Point::create(2-8.0, 7.0, 3.0);
        //
        // assert_eq!(expected_point.x, translation.x);
        // assert_eq!(expected_point.y, translation.y);
        // assert_eq!(expected_point.z, translation.z);
    }
}
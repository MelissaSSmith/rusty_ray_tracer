pub fn add(tuple_one:(f64, f64, f64, f64), tuple_two:(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let x = tuple_one.0 + tuple_two.0;
    let y = tuple_one.1 + tuple_two.1;
    let z = tuple_one.2 + tuple_two.2;
    let w = tuple_one.3 + tuple_two.3;
    (x, y, z, w)
}

pub fn subtract(tuple_one:(f64, f64, f64, f64), tuple_two:(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let x = tuple_one.0 - tuple_two.0;
    let y = tuple_one.1 - tuple_two.1;
    let z = tuple_one.2 - tuple_two.2;
    let w = tuple_one.3 - tuple_two.3;
    (x, y, z, w)
}

pub fn negate(tuple:(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (
        0.0 - tuple.0,
        0.0 - tuple.1,
        0.0 - tuple.2,
        0.0 - tuple.3
    )
}

pub fn multiply(tuple:(f64, f64, f64, f64), scalar:f64) -> (f64, f64, f64, f64) {
    (
        tuple.0 * scalar,
        tuple.1 * scalar,
        tuple.2 * scalar,
        tuple.3 * scalar
    )
}

pub fn divide(tuple:(f64, f64, f64, f64), scalar:f64) -> (f64, f64, f64, f64) {
    (
        tuple.0 / scalar,
        tuple.1 / scalar,
        tuple.2 / scalar,
        tuple.3 / scalar
    )
}

#[cfg(test)]
mod tests {
    use crate::operations::{add, subtract, negate, multiply, divide};
    use crate::features::create_feature;
    use crate::features::point::Point;
    use crate::features::vector::Vector;

    #[test]
    fn test_add_tuples_vector_and_point_creates_point() {
        let tuple_one = (3.0, -2.0, 5.0, 1.0);
        let tuple_two = (-2.0, 3.0, 1.0, 0.0);

        let sum = add(tuple_one, tuple_two);

        match create_feature(sum).as_any().downcast_ref::<Point>() {
            Some(point) => point,
            None => panic!("&feature isn't a Point!"),
        };

        assert_eq!(1.0, sum.0);
        assert_eq!(1.0, sum.1);
        assert_eq!(6.0, sum.2);
        assert_eq!(1.0, sum.3);
    }

    #[test]
    fn test_add_tuples_two_vectors_creates_vector() {
        let tuple_one = (3.0, -2.0, 5.0, 0.0);
        let tuple_two = (-2.0, 3.0, 1.0, 0.0);

        let sum = add(tuple_one, tuple_two);

        match create_feature(sum).as_any().downcast_ref::<Vector>() {
            Some(vector) => vector,
            None => panic!("&feature isn't a Vector!"),
        };


        assert_eq!(1.0, sum.0);
        assert_eq!(1.0, sum.1);
        assert_eq!(6.0, sum.2);
        assert_eq!(0.0, sum.3);
    }

    #[test]
    fn test_subtract_tuples_as_points_creates_vector() {
        let tuple_one = (3.0, 2.0, 1.0, 1.0);
        let tuple_two = (5.0, 6.0, 7.0, 1.0);

        let diff = subtract(tuple_one, tuple_two);

        match create_feature(diff).as_any().downcast_ref::<Vector>() {
            Some(vector) => vector,
            None => panic!("&feature isn't a Vector!"),
        };

        assert_eq!(-2.0, diff.0);
        assert_eq!(-4.0, diff.1);
        assert_eq!(-6.0, diff.2);
        assert_eq!(0.0, diff.3);
    }

    #[test]
    fn test_subtract_tuples_as_vector_creates_vector() {
        let tuple_one = (3.0, 2.0, 1.0, 0.0);
        let tuple_two = (5.0, 6.0, 7.0, 0.0);

        let diff = subtract(tuple_one, tuple_two);

        match create_feature(diff).as_any().downcast_ref::<Vector>() {
            Some(vector) => vector,
            None => panic!("&feature isn't a Vector!"),
        };

        assert_eq!(-2.0, diff.0);
        assert_eq!(-4.0, diff.1);
        assert_eq!(-6.0, diff.2);
        assert_eq!(0.0, diff.3);
    }

    #[test]
    fn test_subtract_tuples_point_and_vector_creates_point() {
        let tuple_one = (3.0, 2.0, 1.0, 1.0);
        let tuple_two = (5.0, 6.0, 7.0, 0.0);

        let diff = subtract(tuple_one, tuple_two);

        match create_feature(diff).as_any().downcast_ref::<Point>() {
            Some(point) => point,
            None => panic!("&feature isn't a Point!"),
        };

        assert_eq!(-2.0, diff.0);
        assert_eq!(-4.0, diff.1);
        assert_eq!(-6.0, diff.2);
        assert_eq!(1.0, diff.3);
    }

    #[test]
    fn test_negate_tuple() {
        let tuple_a = (1.0, -2.0, 3.0, -4.0);

        let tuple = negate(tuple_a);

        assert_eq!(-1.0, tuple.0);
        assert_eq!(2.0, tuple.1);
        assert_eq!(-3.0, tuple.2);
        assert_eq!(4.0, tuple.3);
    }

    #[test]
    fn test_multiply_tuple_by_scalar() {
        let tuple = (1.0, -2.0, 3.0, -4.0);
        let scalar = 3.5;

        let new_tuple = multiply(tuple, scalar);

        assert_eq!(3.5, new_tuple.0);
        assert_eq!(-7.0, new_tuple.1);
        assert_eq!(10.5, new_tuple.2);
        assert_eq!(-14.0, new_tuple.3);
    }

    #[test]
    fn test_multiply_tuple_by_fraction() {
        let tuple = (1.0, -2.0, 3.0, -4.0);
        let fraction = 1.0/2.0;

        let new_tuple = multiply(tuple, fraction);

        assert_eq!(0.5, new_tuple.0);
        assert_eq!(-1.0, new_tuple.1);
        assert_eq!(1.5, new_tuple.2);
        assert_eq!(-2.0, new_tuple.3);
    }

    #[test]
    fn test_divide_tuple_by_scalar() {
        let tuple = (1.0, -2.0, 3.0, -4.0);
        let scalar = 2.0;

        let new_tuple = divide(tuple, scalar);

        assert_eq!(0.5, new_tuple.0);
        assert_eq!(-1.0, new_tuple.1);
        assert_eq!(1.5, new_tuple.2);
        assert_eq!(-2.0, new_tuple.3);
    }
}
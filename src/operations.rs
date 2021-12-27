use std::ops::{Div, Mul, Neg};
use crate::features::{Point, Vector, Tuple};

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

impl Point {
    pub fn add(&self, _vector: Vector) -> Vector {
        Vector{
            x: self.x + _vector.x,
            y: self.y + _vector.y,
            z: self.z + _vector.z
        }
    }

    pub fn subtract_vector(&self, _vector: Vector) -> Vector {
        Vector{
            x: self.x - _vector.x,
            y: self.y - _vector.y,
            z: self.z - _vector.z
        }
    }

    pub fn subtract_point(&self, _point: Point) -> Vector {
        Vector{
            x: self.x - _point.x,
            y: self.y - _point.y,
            z: self.z - _point.z
        }
    }
}

impl Tuple {
    pub fn add(&self, _tuple: Tuple) -> Tuple {
        Tuple{
            x: self.x + _tuple.x,
            y: self.y + _tuple.y,
            z: self.z + _tuple.z,
            w: self.w + _tuple.w
        }
    }

    pub fn subtract(&self, _tuple: Tuple) -> Tuple {
        Tuple{
            x: self.x - _tuple.x,
            y: self.y - _tuple.y,
            z: self.z - _tuple.z,
            w: self.w - _tuple.w
        }
    }

    pub fn negate(&self) -> Tuple {
        Tuple{
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
            w: self.w.neg()
        }
    }

    pub fn multiply(&self, _scalar: f64) -> Tuple {
        Tuple{
            x: self.x.mul(_scalar),
            y: self.y.mul(_scalar),
            z: self.z.mul(_scalar),
            w: self.w.mul(_scalar)
        }
    }

    pub fn divide(&self, _scalar: f64) -> Tuple {
        Tuple{
            x: self.x.div(_scalar),
            y: self.y.div(_scalar),
            z: self.z.div(_scalar),
            w: self.w.div(_scalar)
        }
    }
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
    use crate::operations::{add, subtract, negate, multiply, divide};
    use crate::features::{create_feature, Point, Vector, Tuple};

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
    fn test_add_vector_and_point_creates_point() {
        let point = Point{x: 3.0, y:-2.0, z:5.0};
        let vector = Vector{x: -2.0, y:3.0, z:1.0};

        let new_point = point.add(vector);

        assert_eq!(1.0, new_point.x);
        assert_eq!(1.0, new_point.y);
        assert_eq!(6.0, new_point.z);
    }

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
    fn test_subtract_points_creates_vector() {
        let point_a = Point{x:3.0, y:2.0, z:1.0};
        let point_b = Point{x:5.0, y:6.0, z:7.0};

        let vector = point_a.subtract_point(point_b);

        assert_eq!(-2.0, vector.x);
        assert_eq!(-4.0, vector.y);
        assert_eq!(-6.0, vector.z);
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
    fn test_subtract_vector_from_point_creates_vector() {
        let point_a = Point{x:3.0, y:2.0, z:1.0};
        let vector_a = Vector{x:5.0, y:6.0, z:7.0};

        let vector = point_a.subtract_vector(vector_a);

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

    #[test]
    fn test_add_tuple_types() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};
        let tuple_b = Tuple{x:5.0, y:6.0, z:7.0, w:4.0};

        let tuple = tuple_a.add(tuple_b);

        assert_eq!(8.0, tuple.x);
        assert_eq!(8.0, tuple.y);
        assert_eq!(8.0, tuple.z);
        assert_eq!(7.0, tuple.w);
    }

    #[test]
    fn test_subtract_tuple_types() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};
        let tuple_b = Tuple{x:5.0, y:6.0, z:7.0, w:4.0};

        let tuple = tuple_a.subtract(tuple_b);

        assert_eq!(-2.0, tuple.x);
        assert_eq!(-4.0, tuple.y);
        assert_eq!(-6.0, tuple.z);
        assert_eq!(-1.0, tuple.w);
    }

    #[test]
    fn test_negate_tuple_type() {
        let tuple_a = Tuple{x:3.0, y:2.0, z:1.0, w:3.0};

        let tuple = tuple_a.negate();

        assert_eq!(-3.0, tuple.x);
        assert_eq!(-2.0, tuple.y);
        assert_eq!(-1.0, tuple.z);
        assert_eq!(-3.0, tuple.w);
    }

    #[test]
    fn test_multiply_tuple_type_by_scalar() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 3.5;

        let tuple = tuple_a.multiply(scalar);

        assert_eq!(3.5, tuple.x);
        assert_eq!(-7.0, tuple.y);
        assert_eq!(10.5, tuple.z);
        assert_eq!(-14.0, tuple.w);
    }

    #[test]
    fn test_multiply_tuple_type_by_fraction() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 0.5;

        let tuple = tuple_a.multiply(scalar);

        assert_eq!(0.5, tuple.x);
        assert_eq!(-1.0, tuple.y);
        assert_eq!(1.5, tuple.z);
        assert_eq!(-2.0, tuple.w);
    }

    #[test]
    fn test_divide_tuple_type_by_scalar() {
        let tuple_a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
        let scalar = 2.0;

        let tuple = tuple_a.divide(scalar);

        assert_eq!(0.5, tuple.x);
        assert_eq!(-1.0, tuple.y);
        assert_eq!(1.5, tuple.z);
        assert_eq!(-2.0, tuple.w);
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
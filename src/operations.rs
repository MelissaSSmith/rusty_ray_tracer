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
    todo!()
}

pub fn divide(tuple:(f64, f64, f64, f64), scalar:f64) -> (f64, f64, f64, f64) {
    todo!()
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
        todo!()
    }

    pub fn subtract(&self, _tuple: Tuple) -> Tuple {
        todo!()
    }

    pub fn negate(&self) -> Tuple {
        todo!()
    }

    pub fn multiply(&self, _tuple: Tuple) -> Tuple {
        todo!()
    }

    pub fn divide(&self, _tuple: Tuple) -> Tuple {
        todo!()
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
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{add, subtract, negate, multiply, divide};
    use crate::features::{create_feature, Point, Vector};

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

        let new_tuple = multiply(tuple, scalar);

        assert_eq!(0.5, new_tuple.0);
        assert_eq!(-1.0, new_tuple.1);
        assert_eq!(1.5, new_tuple.2);
        assert_eq!(-2.0, new_tuple.3);
    }
}
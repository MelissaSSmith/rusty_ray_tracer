use crate::features::tuple::Tuple;
use crate::features::vector::Vector;

#[derive(Clone, Copy)]
pub struct Point {
    tuple: Tuple
}

impl Point {
    pub fn create(x: f64, y:f64, z:f64) -> Point {
        Point{tuple: Tuple::create(x, y, z, 1.0)}
    }

    pub(crate) fn create_with_tuple(tuple: Tuple) -> Point {
        Point{tuple}
    }

    pub fn value(&self) -> Tuple {
        self.tuple
    }

    pub fn add(&self, _vector: Vector) -> Point {
        let tuple = self.tuple.add(_vector.value());
        Point::create_with_tuple(tuple)
    }

    pub fn subtract_vector(&self, _vector: Vector) -> Point {
        let tuple = self.tuple.subtract(_vector.value());
        Point::create_with_tuple(tuple)
    }

    pub fn subtract_point(&self, _point: Point) -> Vector {
        let tuple = self.tuple.subtract(_point.tuple);
        Vector::create_with_tuple(tuple)
    }

    pub(crate) fn equals(&self, _point: Point) -> bool {
        self.tuple.equals(_point.tuple)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::point::Point;
    use crate::features::vector::Vector;

    #[test]
    fn test_add_vector_and_point_creates_point() {
        let point = Point::create(3.0, -2.0, 5.0);
        let vector = Vector::create(-2.0, 3.0, 1.0);

        let new_point = point.add(vector);

        let expected_point = Point::create(1.0, 1.0, 6.0);
        assert!(expected_point.equals(new_point));
    }

    #[test]
    fn test_subtract_points_creates_vector() {
        let point_a = Point::create(3.0, 2.0, 1.0);
        let point_b = Point::create(5.0, 6.0, 7.0);

        let vector = point_a.subtract_point(point_b);

        let expected_vector = Vector::create(-2.0, -4.0, -6.0);
        assert!(expected_vector.equals(vector));
    }

    #[test]
    fn test_subtract_vector_from_point_creates_point() {
        let point_a = Point::create(3.0, 2.0, 1.0);
        let vector_a = Vector::create(5.0, 6.0, 7.0);

        let point = point_a.subtract_vector(vector_a);

        let expected_point = Point::create(-2.0, -4.0, -6.0);
        assert!(expected_point.equals(point));
    }
}
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;

#[derive(Clone, Copy)]
pub struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vector
}

impl Ray {
    pub fn create(origin: Point, direction: Vector) -> Ray {
        Ray{origin, direction}
    }

    pub fn position(&self, _t: f64) -> Point {
        self.origin + self.direction * _t
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn transform(&self, _matrix: Matrix) -> Ray {
        let new_point = _matrix.clone() * self.origin;
        let new_vector = _matrix * self.direction;
        Ray::create(new_point, new_vector)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::ray::Ray;
    use crate::features::primitives::vector::Vector;

    #[test]
    fn test_create_and_query_a_ray() {
        let origin = Point::create(1.0, 2.0, 3.0);
        let direction = Vector::create(1.0, 0.0, 0.0);

        let ray = Ray::create(origin, direction);

        assert!(ray.origin.equals(origin));
        assert!(ray.direction.equals(direction));
    }

    #[test]
    fn test_compute_point_from_a_distance() {
        let origin = Point::create(2.0, 3.0, 4.0);
        let direction = Vector::create(1.0, 0.0, 0.0);
        let ray = Ray::create(origin, direction);

        let point_1 = ray.position(0.0);
        assert!(Point::create(2.0,3.0,4.0).equals(point_1));

        let point_2 = ray.position(1.0);
        assert!(Point::create(3.0,3.0,4.0).equals(point_2));

        let point_3 = ray.position(-1.0);
        assert!(Point::create(1.0,3.0,4.0).equals(point_3));

        let point_4 = ray.position(2.5);
        assert!(Point::create(4.5,3.0,4.0).equals(point_4));
    }

    #[test]
    fn test_translate_a_ray() {
        let origin = Point::create(1.0, 2.0, 3.0);
        let direction = Vector::create(0.0, 1.0, 0.0);
        let ray = Ray::create(origin, direction);
        let matrix = Matrix::translate(3.0, 4.0, 5.0);

        let new_ray = ray.transform(matrix);

        assert!(new_ray.origin.equals(Point::create(4.0, 6.0, 8.0)));
        assert!(new_ray.direction.equals(Vector::create(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_scale_a_ray() {
        let origin = Point::create(1.0, 2.0, 3.0);
        let direction = Vector::create(0.0, 1.0, 0.0);
        let ray = Ray::create(origin, direction);
        let matrix = Matrix::scale(2.0, 3.0, 4.0);

        let new_ray = ray.transform(matrix);

        assert!(new_ray.origin.equals(Point::create(2.0, 6.0, 12.0)));
        assert!(new_ray.direction.equals(Vector::create(0.0, 3.0, 0.0)));
    }
}
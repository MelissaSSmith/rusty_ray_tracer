use crate::features::point::Point;
use crate::features::vector::Vector;

pub struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vector
}

impl Ray {
    pub(crate) fn create(origin: Point, direction: Vector) -> Ray {
        Ray{origin, direction}
    }

    fn position(&self, _t: f64) -> Point {
        self.origin.add(self.direction.multiply(_t))
    }
}

#[cfg(test)]
mod tests {
    use crate::features::point::Point;
    use crate::features::ray::Ray;
    use crate::features::vector::Vector;

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
}
use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Cube {}

impl Cube {
    fn check_axis(origin: &f64, direction: &f64) -> (f64, f64) {
        let mut tmin = 0.0;
        let mut tmax = 0.0;

        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            return Cube::swap(tmin, tmax);
        }
        (tmin, tmax)
    }

    fn swap(tmin: f64, tmax: f64) -> (f64, f64) {
        (tmax, tmin)
    }
}

impl Intersect for Cube {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = Cube::check_axis(&_ray.origin.x(), &_ray.direction.x());
        let (ytmin, ytmax) = Cube::check_axis(&_ray.origin.y(), &_ray.direction.y());
        let (ztmin, ztmax) = Cube::check_axis(&_ray.origin.z(), &_ray.direction.z());

        let tmin = vec![xtmin, ytmin, ztmin].iter().fold(-f64::INFINITY, |a, &b| a.max(b));
        let tmax = vec![xtmax, ytmax, ztmax].iter().fold(f64::INFINITY, |a, &b| a.min(b));

        if tmin > tmax {
            return vec![];
        }

        vec![Intersection::create(tmin, &_object), Intersection::create(tmax, &_object)]
    }
}

impl Normal for Cube {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let maxc = vec![_point.x().abs(), _point.y().abs(), _point.z().abs()]
            .iter()
            .fold(-f64::INFINITY, |a, &b| a.max(b));

        if maxc == _point.x().abs() {
            return Vector::create(_point.x(), 0.0, 0.0);
        } else if maxc == _point.y().abs() {
            return Vector::create(0.0, _point.y(), 0.0);
        }
        Vector::create(0.0, 0.0, _point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::cube::Cube;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_ray_intersects_a_cube() {
        let cube = Shape::Cube.create();

        let tests = vec![
            (Point::create(5.0, 0.5, 0.0), Vector::create(-1.0, 0.0, 0.0), 4.0, 6.0),
            (Point::create(-5.0, 0.5, 0.0), Vector::create(1.0, 0.0, 0.0), 4.0, 6.0),
            (Point::create(0.5, 5.0, 0.0), Vector::create(0.0, -1.0, 0.0), 4.0, 6.0),
            (Point::create(0.5, -5.0, 0.0), Vector::create(0.0, 1.0, 0.0), 4.0, 6.0),
            (Point::create(0.5, 0.0, 5.0), Vector::create(0.0, 0.0, -1.0), 4.0, 6.0),
            (Point::create(0.5, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), 4.0, 6.0),
            (Point::create(0.0, 0.5, 0.0), Vector::create(0.0, 0.0, 1.0), -1.0, 1.0),
        ];

        for test in tests {
            let ray = Ray::create(test.0, test.1);
            let intersections = Cube::intersect(&cube, &ray);

            assert!(intersections[0].t.equals(test.2));
            assert!(intersections[1].t.equals(test.3));
        }
    }

    #[test]
    fn test_ray_misses_a_cube() {
        let cube = Shape::Cube.create();

        let tests = vec![
            (Point::create(-2.0, 0.0, 0.0), Vector::create(0.2673, 0.5345, 0.8018)),
            (Point::create(0.0, -2.0, 0.0), Vector::create(0.8018, 0.2673, 0.5345)),
            (Point::create(0.0, 0.0, -2.0), Vector::create(0.5345, 0.8018, 0.2673)),
            (Point::create(2.0, 0.0, 2.0), Vector::create(0.0, 0.0, -1.0)),
            (Point::create(0.0, 2.0, 2.0), Vector::create(0.0, -1.0, 0.0)),
            (Point::create(2.0, 2.0, 0.0), Vector::create(-1.0, 0.0, 0.0)),
        ];

        for test in tests {
            let ray = Ray::create(test.0, test.1);
            let intersections = Cube::intersect(&cube, &ray);

            assert_eq!(intersections.len(), 0);
        }
    }

    #[test]
    fn test_normal_on_the_surface_of_a_cube() {
        let cube = Shape::Cube.create();

        let tests = vec![
            (Point::create(1.0, 0.5, -0.8), Vector::create(1.0, 0.0, 0.0)),
            (Point::create(-1.0, -0.2, 0.9), Vector::create(-1.0, 0.0, 0.0)),
            (Point::create(-0.4, 1.0, -0.1), Vector::create(0.0, 1.0, 0.0)),
            (Point::create(0.3, -1.0, -0.7), Vector::create(0.0, -1.0, 0.0)),
            (Point::create(-0.6, 0.3, 1.0), Vector::create(0.0, 0.0, 1.0)),
            (Point::create(0.4, 0.4, -1.0), Vector::create(0.0, 0.0, -1.0)),
            (Point::create(1.0, 1.0, 1.0), Vector::create(1.0, 0.0, 0.0)),
            (Point::create(-1.0, -1.0, -1.0), Vector::create(-1.0, 0.0, 0.0)),
        ];

        for test in tests {
            let normal = Cube::normal(&cube, &test.0);

            assert!(normal.equals(test.1));
        }
    }
}
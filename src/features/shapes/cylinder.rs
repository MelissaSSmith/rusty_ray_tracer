use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::operations::Operations;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy)]
pub struct Cylinder {
    maximum: f64,
    minimum: f64,
    closed: bool
}

impl Cylinder {
    pub fn create() -> Cylinder {
        Cylinder {
            maximum: f64::INFINITY,
            minimum: -f64::INFINITY,
            closed: false
        }
    }

    pub fn maximum_bound(&self) -> f64 {
        self.maximum
    }

    pub fn minimum_bound(&self) -> f64 {
        self.minimum
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn with_maximum_bound(self, bound: f64) -> Cylinder {
        Cylinder {
            maximum: bound,
            ..self
        }
    }

    pub fn with_minimum_bound(self, bound: f64) -> Cylinder {
        Cylinder {
            minimum: bound,
            ..self
        }
    }

    pub fn with_closed(self, closed: bool) -> Cylinder {
        Cylinder {
            closed,
            ..self
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps(object: &Object, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        if !object.closed() || ray.direction.y().equals(0.0) {
            return  intersections;
        }

        let t = (object.minimum_bound() - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            intersections.push(Intersection::create(t, object));
        }

        let t = (object.maximum_bound() - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            intersections.push(Intersection::create(t, object));
        }

        intersections
    }
}

impl Intersect for Cylinder {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        let a = _ray.direction.x().powi(2) + _ray.direction.z().powi(2);

        if !a.equals(0.0) {
            let b = 2.0 * _ray.origin.x() * _ray.direction.x() + 2.0 * _ray.origin.z() * _ray.direction.z();
            let c = _ray.origin.x().powi(2) + _ray.origin.z().powi(2) - 1.0;

            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return vec![];
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            let y0 = _ray.origin.y() + t0 * _ray.direction.y();
            if _object.minimum_bound() < y0 && y0 < _object.maximum_bound() {
                intersections.push(Intersection::create(t0, &_object));
            }

            let y1 = _ray.origin.y() + t1 * _ray.direction.y();
            if _object.minimum_bound() < y1 && y1 < _object.maximum_bound() {
                intersections.push(Intersection::create(t1, &_object));
            }
        }

        intersections.append(&mut Cylinder::intersect_caps(_object, _ray));

        intersections
    }
}

impl Normal for Cylinder {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let dist = _point.x().powi(2) + _point.z().powi(2);

        if dist < 1.0 && _point.y() >= (_object.maximum_bound() - EPSILON) {
            return Vector::create(0.0, 1.0, 0.0);
        }

        if dist < 1.0 && _point.y() <= (_object.minimum_bound() + EPSILON) {
            return Vector::create(0.0, -1.0, 0.0);
        }

        Vector::create(_point.x(), 0.0, _point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::cylinder::Cylinder;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_ray_misses_a_cylinder() {
        let cylinder = Shape::Cylinder(Cylinder::create()).create();

        let tests = vec![
            (Point::create(1.0, 0.0, 0.0), Vector::create(0.0, 1.0, 0.0)),
            (Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 1.0, 0.0)),
            (Point::create(0.0, 0.0, -5.0), Vector::create(1.0, 1.0, 1.0))
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);
            let intersections = Cylinder::intersect(&cylinder, &ray);

            assert_eq!(intersections.len(), 0);
        }
    }

    #[test]
    fn test_ray_hits_a_cylinder() {
        let cylinder = Shape::Cylinder(Cylinder::create()).create();

        let tests = vec![
            (Point::create(1.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), 5.0, 5.0),
            (Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), 4.0, 6.0),
            (Point::create(0.5, 0.0, -5.0), Vector::create(0.1, 1.0, 1.0), 6.80798, 7.08872)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);
            let intersections = Cylinder::intersect(&cylinder, &ray);

            assert_eq!(intersections.len(), 2);
            assert!(intersections[0].t.equals(test.2));
            assert!(intersections[1].t.equals(test.3));
        }
    }

    #[test]
    fn test_normal_vector_on_a_cylinder() {
        let cylinder = Shape::Cylinder(Cylinder::create()).create();

        let tests = vec![
            (Point::create(1.0, 0.0, 0.0), Vector::create(1.0, 0.0, 0.0)),
            (Point::create(0.0, 5.0, -1.0), Vector::create(0.0, 0.0, -1.0)),
            (Point::create(0.0, -2.0, 1.0), Vector::create(0.0, 0.0, 1.0)),
            (Point::create(-1.0, 1.0, 0.0), Vector::create(-1.0, 0.0, 0.0))
        ];

        for test in tests {
            let normal = Cylinder::normal(&cylinder, &test.0);

            assert!(normal.equals(test.1));
        }
    }

    #[test]
    fn test_maximum_and_minimum_bounds() {
        let cylinder = Shape::Cylinder(Cylinder::create()).create();

        assert_eq!(cylinder.maximum_bound(), f64::INFINITY);
        assert_eq!(cylinder.minimum_bound(), -f64::INFINITY);
    }

    #[test]
    fn test_truncated_cylinders() {
        let cylinder = Shape::Cylinder(
            Cylinder::create()
                .with_maximum_bound(2.0)
                .with_minimum_bound(1.0)
        ).create();

        let tests = vec![
            (Point::create(0.0, 1.5, 0.0), Vector::create(0.1, 1.0, 0.0), 0),
            (Point::create(0.0, 3.0, -5.0), Vector::create(0.0, 0.0, 1.0), 0),
            (Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), 0),
            (Point::create(0.0, 2.0, -5.0), Vector::create(0.0, 0.0, 1.0), 0),
            (Point::create(0.0, 1.0, -5.0), Vector::create(0.0, 0.0, 1.0), 0),
            (Point::create(0.0, 1.5, -2.0), Vector::create(0.0, 0.0, 1.0), 2)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let intersections = Cylinder::intersect(&cylinder, &ray);

            assert_eq!(intersections.len(), test.2);
        }
    }

    #[test]
    fn test_default_for_a_closed_cylinder() {
        let cylinder = Shape::Cylinder(Cylinder::create()).create();

        assert_eq!(false, cylinder.closed())
    }

    #[test]
    fn test_intersecting_the_caps_of_a_closed_cylinder() {
        let cylinder = Shape::Cylinder(
            Cylinder::create()
                .with_maximum_bound(2.0)
                .with_minimum_bound(1.0)
                .with_closed(true)
        ).create();

        let tests = vec![
            (Point::create(0.0, 3.0, 0.0), Vector::create(0.0, -1.0, 0.0), 2),
            (Point::create(0.0, 3.0, -2.0), Vector::create(0.0, -1.0, 2.0), 2),
            (Point::create(0.0, 4.0, -2.0), Vector::create(0.0, -1.0, 1.0), 2),
            (Point::create(0.0, 0.0, -2.0), Vector::create(0.0, 1.0, 2.0), 2),
            (Point::create(0.0, -1.0, -2.0), Vector::create(0.0, 1.0, 1.0), 2)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let intersections = Cylinder::intersect(&cylinder, &ray);

            assert_eq!(intersections.len(), test.2);
        }
    }

    #[test]
    fn test_normal_vector_on_cylinders_end_caps() {
        let cylinder = Shape::Cylinder(
            Cylinder::create()
                .with_maximum_bound(2.0)
                .with_minimum_bound(1.0)
                .with_closed(true)
        ).create();

        let tests = vec![
            (Point::create(0.0, 1.0, 0.0), Vector::create(0.0, -1.0, 0.0)),
            (Point::create(0.5, 1.0, 0.0), Vector::create(0.0, -1.0, 0.0)),
            (Point::create(0.0, 1.0, 0.5), Vector::create(0.0, -1.0, 0.0)),
            (Point::create(0.0, 2.0, 0.0), Vector::create(0.0, 1.0, 0.0)),
            (Point::create(0.5, 2.0, 0.0), Vector::create(0.0, 1.0, 0.0)),
            (Point::create(0.0, 2.0, 0.5), Vector::create(0.0, 1.0, 0.0))
        ];

        for test in tests {
            let normal = Cylinder::normal(&cylinder, &test.0);

            assert!(normal.equals(test.1));
        }
    }
}
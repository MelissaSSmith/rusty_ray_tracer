use crate::features::intersection::Intersection;
use crate::features::primitives::operations::Operations;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Cylinder {}

impl Intersect for Cylinder {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let a = _ray.direction.x().powi(2) + _ray.direction.z().powi(2);

        if a.equals(0.0) {
            return vec![];
        }

        let b = 2.0 * _ray.origin.x() * _ray.direction.x() + 2.0 * _ray.origin.z() * _ray.direction.z();
        let c = _ray.origin.x().powi(2) + _ray.origin.z().powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        vec![Intersection::create(t0, _object), Intersection::create(t1, _object)]
    }
}

impl Normal for Cylinder {
    fn normal(_object: &Object, _point: &Point) -> Vector {
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
        let cylinder = Shape::Cylinder.create();

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
        let cylinder = Shape::Cylinder.create();

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
        let cylinder = Shape::Cylinder.create();

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
}
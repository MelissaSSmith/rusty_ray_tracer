use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::point::Point;
use crate::features::ray::Ray;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Plane {}

impl Intersect for Plane {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        if _ray.direction.y().abs() < EPSILON {
            return vec![]
        }
        let t = -_ray.origin.y() / _ray.direction.y();
        vec![Intersection::create(t, _object)]
    }
}

impl Normal for Plane {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        Vector::create(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::plane::Plane;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::shape::{Object, Shape};

    #[test]
    fn test_normal_of_a_plane_is_constant_everywhere() {
        let plane = Shape::Plane.create();

        let n1 = Object::normal(&plane, &Point::zero());
        let n2 = Plane::normal(&plane, &Point::create(10.0, 0.0, -10.0));
        let n3 = Plane::normal(&plane, &Point::create(-5.0, 0.0, 150.0));

        assert!(n1.equals(Vector::create(0.0, 1.0, 0.0)));
        assert!(n2.equals(Vector::create(0.0, 1.0, 0.0)));
        assert!(n3.equals(Vector::create(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Shape::Plane.create();
        let ray = Ray::create(Point::create(0.0, 10.0, 0.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Plane::intersect(&plane, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        let plane = Shape::Plane.create();
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));

        let intersections = Plane::intersect(&plane, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_intersecting_a_plane_from_above() {
        let plane = Shape::Plane.create();
        let ray = Ray::create(Point::create(0.0, 1.0, 0.0), Vector::create(0.0, -1.0, 0.0));

        let intersections = Plane::intersect(&plane, &ray);

        assert_eq!(1, intersections.len());
        assert_eq!(1.0, intersections[0].t);
    }

    #[test]
    fn test_ray_intersection_a_plane_from_below() {
        let plane = Shape::Plane.create();
        let ray = Ray::create(Point::create(0.0, -1.0, 0.0), Vector::create(0.0, 1.0, 0.0));

        let intersections = Plane::intersect(&plane, &ray);

        assert_eq!(1, intersections.len());
        assert_eq!(1.0, intersections[0].t);
    }
}
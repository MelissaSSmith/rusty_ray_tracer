use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::operations::Operations;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy)]
pub struct Cone {
    maximum: f64,
    minimum: f64,
    closed: bool
}

impl Cone {
    pub fn create() -> Cone {
        Cone {
            maximum: f64::INFINITY,
            minimum: f64::NEG_INFINITY,
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

    pub fn with_maximum_bound(self, bound: f64) -> Cone {
        Cone {
            maximum: bound,
            ..self
        }
    }

    pub fn with_minimum_bound(self, bound: f64) -> Cone {
        Cone {
            minimum: bound,
            ..self
        }
    }

    pub fn with_closed(self, closed: bool) -> Cone {
        Cone {
            closed,
            ..self
        }
    }

    fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= radius.powi(2)
    }

    fn intersect_caps(object: &Object, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        if !object.closed() || ray.direction.y().equals(0.0) {
            return  intersections;
        }

        let t = (object.minimum_bound() - ray.origin.y()) / ray.direction.y();
        if Cone::check_cap(ray, t, object.minimum_bound()) {
            intersections.push(Intersection::create(t, object));
        }

        let t = (object.maximum_bound() - ray.origin.y()) / ray.direction.y();
        if Cone::check_cap(ray, t, object.maximum_bound()) {
            intersections.push(Intersection::create(t, object));
        }

        intersections
    }
}

impl Intersect for Cone {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        let a = _ray.direction.x().powi(2) - _ray.direction.y().powi(2) + _ray.direction.z().powi(2);
        let b = 2.0 * _ray.origin.x() * _ray.direction.x() -
            2.0 * _ray.origin.y() * _ray.direction.y() +
            2.0 * _ray.origin.z() * _ray.direction.z();
        let c = _ray.origin.x().powi(2) - _ray.origin.y().powi(2) + _ray.origin.z().powi(2);

        if a == 0.0 && b != 0.0 {
            let t = -c / (2.0 * b);
            intersections.push(Intersection::create(t, _object));
        }

        if a != 0.0 {
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

        intersections.append(&mut Cone::intersect_caps(_object, _ray));

        intersections
    }
}

impl Normal for Cone {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let dist = _point.x().powi(2) + _point.z().powi(2);

        if dist < 1.0 && _point.y() >= (_object.maximum_bound() - EPSILON) {
            return Vector::create(0.0, 1.0, 0.0);
        }

        if dist < 1.0 && _point.y() <= (_object.minimum_bound() + EPSILON) {
            return Vector::create(0.0, -1.0, 0.0);
        }

        let mut y = (_point.x().powi(2) + _point.z().powi(2)).sqrt();
        if _point.y() > 0.0 {
            y = -y;
        }

        Vector::create(_point.x(), y, _point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::cone::Cone;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_intersecting_a_cone_with_a_ray() {
        let cone = Shape::Cone(Cone::create()).create();

        let tests = vec![
            (Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), 5.0, 5.0),
            (Point::create(0.0, 0.0, -5.0), Vector::create(1.0, 1.0, 1.0), 8.66025, 8.66025),
            (Point::create(1.0, 1.0, -5.0), Vector::create(-0.5, -1.0, 1.0), 4.55006, 49.44994)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let intersections = Cone::intersect(&cone, &ray);

            assert_eq!(intersections.len(), 2);
            assert!(intersections[0].t.equals(test.2));
            assert!(intersections[1].t.equals(test.3));
        }
    }

    #[test]
    fn test_intersecting_a_cone_parallel_to_one_of_its_halves() {
        let cone = Shape::Cone(Cone::create()).create();

        let direction = Vector::create(0.0, 1.0, 1.0).normalize();
        let ray = Ray::create(Point::create(0.0, 0.0, -1.0), direction);

        let intersections = Cone::intersect(&cone, &ray);

        assert_eq!(intersections.len(), 1);
        assert!(intersections[0].t.equals(0.35355));
    }

    #[test]
    fn test_intersecting_a_cones_end_caps() {
        let cone = Shape::Cone(
            Cone::create()
                .with_minimum_bound(-0.5)
                .with_maximum_bound(0.5)
                .with_closed(true)
        ).create();

        let tests = vec![
            (Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 1.0, 0.0), 0),
            (Point::create(0.0, 0.0, -0.25), Vector::create(0.0, 1.0, 1.0), 2),
            (Point::create(0.0, 0.0, -0.25), Vector::create(0.0, 1.0, 0.0), 4)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let intersections = Cone::intersect(&cone, &ray);

            assert_eq!(intersections.len(), test.2);
        }
    }

    #[test]
    fn test_computing_the_normal_vector_on_a_cone() {
        let cone = Shape::Cone(Cone::create()).create();

        let tests = vec![
            (Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 0.0, 0.0)),
            (Point::create(1.0, 1.0, 1.0), Vector::create(1.0, -f64::sqrt(2.0), 1.0)),
            (Point::create(-1.0, -1.0, 0.0), Vector::create(-1.0, 1.0, 0.0))
        ];

        for test in tests {
            let normal = Cone::normal(&cone, &test.0);

            assert!(normal.equals(test.1));
        }
    }

    #[test]
    fn test_unbounded_cone_has_a_bounding_box() {
        let cone = Shape::Cone(Cone::create()).create();

        assert!(cone.bounds().minimum().equals(Point::create(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)));
        assert!(cone.bounds().maximum().equals(Point::create(f64::INFINITY, f64::INFINITY, f64::INFINITY)));
    }

    #[test]
    fn test_bounded_cone_has_a_bounding_box() {
        let cone = Shape::Cone(
            Cone::create()
                .with_minimum_bound(-5.0)
                .with_maximum_bound(3.0)
        ).create();

        assert!(cone.bounds().minimum().equals(Point::create(-5.0, -5.0, -5.0)));
        assert!(cone.bounds().maximum().equals(Point::create(5.0, 3.0, 5.0)));
    }
}
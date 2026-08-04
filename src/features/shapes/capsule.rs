use crate::features::intersection::Intersection;
use linear_algebra::f64_operations::Operations;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

/// A unit-radius capsule aligned with the y-axis: a cylindrical body spanning
/// `minimum` to `maximum` on y, closed off by a hemisphere of radius 1 at each
/// end. The hemisphere centres sit at y = `minimum` and y = `maximum`, so the
/// capsule's full extent on the y-axis is `[minimum - 1, maximum + 1]`.
///
/// Unlike a `Cylinder`, a capsule is always closed by its rounded ends, so it
/// has no `closed` flag and the bounds must be finite.
#[derive(Clone, Copy, Debug)]
pub struct Capsule {
    minimum: f64,
    maximum: f64
}

impl Capsule {
    pub fn create() -> Self {
        Capsule {
            minimum: -1.0,
            maximum: 1.0
        }
    }

    pub fn minimum_bound(&self) -> f64 {
        self.minimum
    }

    pub fn maximum_bound(&self) -> f64 {
        self.maximum
    }

    pub fn with_minimum_bound(self, bound: f64) -> Self {
        Self {
            minimum: bound,
            ..self
        }
    }

    pub fn with_maximum_bound(self, bound: f64) -> Self {
        Self {
            maximum: bound,
            ..self
        }
    }

    // Intersect the ray with the hemisphere whose centre is (0, centre_y, 0),
    // keeping only the points on the rounded end: the bottom cap keeps hits with
    // y <= centre_y, the top cap keeps hits with y >= centre_y. This excludes the
    // half of each sphere that overlaps the cylindrical body.
    fn intersect_hemisphere(object: &Object, ray: &Ray, centre_y: f64, is_bottom: bool, intersections: &mut Vec<Intersection>) {
        let oc_y = ray.origin().y() - centre_y;

        let a = ray.direction().x().powi(2) + ray.direction().y().powi(2) + ray.direction().z().powi(2);
        let b = 2.0 * (ray.origin().x() * ray.direction().x()
            + oc_y * ray.direction().y()
            + ray.origin().z() * ray.direction().z());
        let c = ray.origin().x().powi(2) + oc_y.powi(2) + ray.origin().z().powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return;
        }

        let sqrt_disc = disc.sqrt();
        let t0 = (-b - sqrt_disc) / (2.0 * a);
        let t1 = (-b + sqrt_disc) / (2.0 * a);

        let y0 = ray.origin().y() + t0 * ray.direction().y();
        if (is_bottom && y0 <= centre_y) || (!is_bottom && y0 >= centre_y) {
            intersections.push(Intersection::create(t0, object, 0.0, 0.0));
        }

        let y1 = ray.origin().y() + t1 * ray.direction().y();
        if (is_bottom && y1 <= centre_y) || (!is_bottom && y1 >= centre_y) {
            intersections.push(Intersection::create(t1, object, 0.0, 0.0));
        }
    }
}

impl Intersect for Capsule {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        let minimum = _object.minimum_bound();
        let maximum = _object.maximum_bound();

        // Cylindrical body: x^2 + z^2 = 1, restricted to minimum < y < maximum.
        let a = _ray.direction().x().powi(2) + _ray.direction().z().powi(2);
        if !a.equals(0.0) {
            let b = 2.0 * _ray.origin().x() * _ray.direction().x()
                + 2.0 * _ray.origin().z() * _ray.direction().z();
            let c = _ray.origin().x().powi(2) + _ray.origin().z().powi(2) - 1.0;

            let disc = b.powi(2) - 4.0 * a * c;
            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                let t0 = (-b - sqrt_disc) / (2.0 * a);
                let t1 = (-b + sqrt_disc) / (2.0 * a);

                let y0 = _ray.origin().y() + t0 * _ray.direction().y();
                if minimum < y0 && y0 < maximum {
                    intersections.push(Intersection::create(t0, _object, 0.0, 0.0));
                }

                let y1 = _ray.origin().y() + t1 * _ray.direction().y();
                if minimum < y1 && y1 < maximum {
                    intersections.push(Intersection::create(t1, _object, 0.0, 0.0));
                }
            }
        }

        // Rounded ends.
        Capsule::intersect_hemisphere(_object, _ray, minimum, true, &mut intersections);
        Capsule::intersect_hemisphere(_object, _ray, maximum, false, &mut intersections);

        intersections
    }
}

impl Normal for Capsule {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        if _point.y() > _object.maximum_bound() {
            // Top hemisphere, centred at (0, maximum, 0).
            Vector::create(_point.x(), _point.y() - _object.maximum_bound(), _point.z())
        } else if _point.y() < _object.minimum_bound() {
            // Bottom hemisphere, centred at (0, minimum, 0).
            Vector::create(_point.x(), _point.y() - _object.minimum_bound(), _point.z())
        } else {
            // Cylindrical body.
            Vector::create(_point.x(), 0.0, _point.z())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::intersection::Intersection;
    use linear_algebra::f64_operations::Operations;
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use linear_algebra::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::capsule::Capsule;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    fn sorted_ts(mut intersections: Vec<Intersection>) -> Vec<f64> {
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections.iter().map(|i| i.t).collect()
    }

    #[test]
    fn test_default_capsule_bounds() {
        let capsule = Capsule::create();

        assert_eq!(capsule.minimum_bound(), -1.0);
        assert_eq!(capsule.maximum_bound(), 1.0);
    }

    #[test]
    fn test_ray_hits_the_cylindrical_body() {
        let capsule = Shape::Capsule(Capsule::create()).create();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let ts = sorted_ts(Capsule::intersect(&capsule, &ray));

        assert_eq!(ts.len(), 2);
        assert!(ts[0].equals(4.0));
        assert!(ts[1].equals(6.0));
    }

    #[test]
    fn test_ray_passes_through_both_rounded_ends_along_the_axis() {
        let capsule = Shape::Capsule(Capsule::create()).create();
        let ray = Ray::create(Point::create(0.0, 5.0, 0.0), Vector::create(0.0, -1.0, 0.0));

        let ts = sorted_ts(Capsule::intersect(&capsule, &ray));

        // Enters the top dome at y = 2 (t = 3), exits the bottom dome at y = -2 (t = 7).
        assert_eq!(ts.len(), 2);
        assert!(ts[0].equals(3.0));
        assert!(ts[1].equals(7.0));
    }

    #[test]
    fn test_ray_hits_the_rounded_cap_where_a_plain_cylinder_would_miss() {
        let capsule = Shape::Capsule(Capsule::create()).create();
        // y = 1.5 is above the cylindrical body (max = 1) but inside the top dome.
        let ray = Ray::create(Point::create(0.0, 1.5, -5.0), Vector::create(0.0, 0.0, 1.0));

        let ts = sorted_ts(Capsule::intersect(&capsule, &ray));

        assert_eq!(ts.len(), 2);
        assert!(ts[0].equals(4.13397));
        assert!(ts[1].equals(5.86603));
    }

    #[test]
    fn test_ray_misses_the_capsule() {
        let capsule = Shape::Capsule(Capsule::create()).create();
        let ray = Ray::create(Point::create(3.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Capsule::intersect(&capsule, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_normal_on_the_cylindrical_body() {
        let capsule = Shape::Capsule(Capsule::create()).create();

        let normal = Capsule::normal(&capsule, &Point::create(1.0, 0.0, 0.0));

        assert_eq!(normal, Vector::create(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_on_the_top_hemisphere() {
        let capsule = Shape::Capsule(Capsule::create()).create();
        let sqrt_2_over_2 = 2.0_f64.sqrt() / 2.0;

        // A point on the top dome (centre (0, 1, 0)) at 45 degrees.
        let point = Point::create(sqrt_2_over_2, 1.0 + sqrt_2_over_2, 0.0);
        let normal = Capsule::normal(&capsule, &point).normalize();

        assert_eq!(normal, Vector::create(sqrt_2_over_2, sqrt_2_over_2, 0.0));
    }

    #[test]
    fn test_normal_on_the_bottom_hemisphere() {
        let capsule = Shape::Capsule(Capsule::create()).create();

        let normal = Capsule::normal(&capsule, &Point::create(0.0, -2.0, 0.0));

        assert_eq!(normal, Vector::create(0.0, -1.0, 0.0));
    }

    #[test]
    fn test_truncated_capsule_extends_bounds_by_the_radius() {
        let capsule = Shape::Capsule(
            Capsule::create()
                .with_minimum_bound(-2.0)
                .with_maximum_bound(3.0)
        ).create();

        assert_eq!(capsule.bounds().minimum(), Point::create(-1.0, -3.0, -1.0));
        assert_eq!(capsule.bounds().maximum(), Point::create(1.0, 4.0, 1.0));
    }
}

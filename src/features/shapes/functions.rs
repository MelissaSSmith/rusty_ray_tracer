use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::shapes::shape::Object;

impl Object {
    pub(crate) fn check_axis(origin: &f64, direction: &f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            return (tmax, tmin);
        }
        (tmin, tmax)
    }

    pub(crate) fn check_disk(ray: &Ray, t: f64) -> bool {
        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }
}
